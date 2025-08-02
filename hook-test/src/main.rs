use std::arch::{global_asm, naked_asm};
use std::slice;
use iced_x86::{Decoder, DecoderOptions, FlowControl, Formatter, Instruction, NasmFormatter, OpKind, Register};
use memmap2::{MmapMut, MmapOptions};

#[cfg(target_pointer_width = "64")]
const BITNESS: u32 = 64;
#[cfg(target_pointer_width = "32")]
const BITNESS: u32 = 32;

const DISPL_SIZE: u32 = BITNESS / 8;
#[cfg(target_pointer_width = "64")]
const JMP_INTERCEPTOR_BYTE_LEN: usize = 12;
#[cfg(target_pointer_width = "32")]
const JMP_INTERCEPTOR_BYTE_LEN: usize = 7;

// +------------+
// | caller of  |    +-------------------+
// | now hooked |    | original function |
// | function   |    +-------------------+
// +------------+      • (0)
//   ^     | (1)       •  first few instructions get overwritten
//   |     | call      •  now immediately jumps to our interceptor
//   |     '----.      •  it becomes the overwritten function     .-------------.
//   |          |      •                                          |             |
//   |          v      v                                          |             v
//   |     +-------------+  (2) immediately jump to interceptor   |      +-------------+
//   |     | overwritten |----------------------------------------'      | interceptor |
//   |     |  function   |<-.                                            +-------------+
//   |     +-------------+  |                                               (3) | store registers & arguments
//   |            | (6)     |                                                   | create the Args-struct
//   |            |         |                                                   | jump to the hook
//   |            |         |            can call the trampoline in order       v
//   |            |         |             to call the original function  (4) +------+
//   |            |         |        .---------------------------------------| hook |
//   |            |         |        |                                    .->+------+
//   |            |         |        v                                    |     | (7)
//   |            |         |  +------------+ contains the overwritten    |     |
//   |            |         |  | trampoline | instructions from the       |     |
//   |            |         |  +------------+ original function           |     |
//   |            |         |        | (5)                                |     |
//   |            |         |        |  jump to the hooked function       |     |
//   |            |         |        |  behind the hook-instructions      |     |
//   |            |         '--------'                                    |     |
//   |            |                                                       |     |
//   |            |    return to its caller, i.e. return into the hook    |     |
//   |            '-------------------------------------------------------'     |
//   |                                                                          |
//   '--------------------------------------------------------------------------'
//             return to its caller, i.e. return to the original caller
//
//
//
// Original Function:
// 0:  48 8b 3d 04 00 00 00    mov    rdi, [rip+0x4]        # b <0xb>
// 7:  74 05                   je     e <end>
// 9:  e8 00 00 00 00          call   e <end>
// 000000000000000e <end>:
// e:  c3                      ret
//
// Overwritten Function:
// ; jump to our interceptor
// 0:  48 b8 ef cd ab 89 67    movabs rax,0x123456789abcdef <interceptor_address>
// 7:  45 23 01
// a:  ff e0                   jmp    rax
// ; nop the rest of the last instruction we overwrote
// c:  90                      nop
// d:  90                      nop
// 00000000000000 e <end>:
// e:  c3                      ret
//
// Trampoline:
// ; mov rdi, [rip+0x4]
// 0:  48 b8 0b 00 00 00 00    movabs rax,0xb
// 7:  00 00 00
// a:  48 8b 38                mov    rdi,QWORD PTR [rax]
// ; je e
// d:  75 0c                   jne    1b <skipped>
// f:  48 b8 0e 00 00 00 00    movabs rax,0xe
// 16: 00 00 00
// 19: ff e0                   jmp    rax
// 000000000000001b <skipped>:
// ; call e
// 1b: 48 b8 0e 00 00 00 00    movabs rax,0xe
// 22: 00 00 00
// 25: ff d0                   call   rax
// ; jump to original function
// 27: 48 b8 0e 00 00 00 00    movabs rax,0xe
// 2e: 00 00 00
// 31: ff e0                   jmp    rax
//
//


#[derive(Clone)]
struct MyDecoder {
    addr: usize,
    read: usize,
}
impl MyDecoder {
    /// # Safety
    /// * `addr` must point to a valid address
    /// * `addr` must live at least as long as this struct
    /// * `addr` must have at least as many bytes after it as needed for all
    ///    calls to `decode`, i.e., the memory region can't end prematurely
    pub unsafe fn new(addr: usize) -> Self {
        Self {
            addr,
            read: 0,
        }
    }

    unsafe fn decoder(&self) -> Decoder<'static> {
        // non-contrived x86_64 instructions are max 15 bytes
        let slice = unsafe { slice::from_raw_parts(self.addr as *const u8, 15) };
        Decoder::with_ip(BITNESS, slice, self.addr as u64, DecoderOptions::NONE)
    }

    pub fn decode(&mut self) -> Instruction {
        let instruction = unsafe { self.decoder() }.decode();
        if instruction.is_invalid() {
            panic!("decoded invalid instruction");
        }
        self.addr += instruction.len();
        self.read += instruction.len();
        instruction
    }

    /// Number of bytes of arguments passed via the stack
    /// 
    /// Gotten by decoding until the first `ret` instruction and taking its immediate
    /// if it exists, e.g. `retn 16`, or `0` if there isn't any, e.g. `ret`.
    /// Resets the Decoder afterwards.
    pub fn stack_argument_size(&self) -> u16 {
        let mut decoder = self.clone();
        loop {
            let instruction = decoder.decode();
            if instruction.flow_control() != FlowControl::Return {
                continue;
            }
            if instruction.op_count() == 0 {
                return 0;
            }
            assert_eq!(instruction.op_count(), 1);
            assert_eq!(instruction.op0_kind(), OpKind::Immediate16);
            return instruction.immediate16();
        }
    }
}

struct Hook {
    /// address of the original function that we hooked
    orig_addr: usize,
    /// address of the trampoline, which we can call to call the original function
    trampoline_addr: usize,
    /// address of the function we jump to from the original function, that
    /// calls the hook
    interceptor_addr: usize,
    /// address of the hook function that should be called instead of the original function
    hook_addr: usize,
    /// original bytes of the original function that are overwritten when enabling the hook
    orig_bytes: [u8; JMP_INTERCEPTOR_BYTE_LEN],
    /// argument-bytes passed to the original function via the stack
    orig_stack_arg_size: u16,
}

global_asm!(
    ".global interceptor",
    "interceptor:",
    "push eax",
    ".global end_interceptor",
    "end_interceptor:",
    "ret",
);
unsafe extern "C" {
    fn interceptor();
    fn end_interceptor();
}

fn hook_function(orig_addr: usize, hook: for<'a> fn(&'static Hook, ArgsRef<'a>)) -> &'static Hook {
    // The jump to the interceptor overwrites up to 12 bytes.
    // The overwritten code can have 11 bytes instructions, then a 15-byte instruction.
    // Then follows the jump to the next instruction of the original function, another 12 bytes.
    // -> the max size of the trampoline is 12 + 11 + 15 + 12 = 50 bytes
    let trampoline_size = 64;
    let interceptor_size = end_interceptor as usize - interceptor as usize;
    let mut page = MmapMut::map_anon(trampoline_size + interceptor_size).unwrap();

    let hook_addr = hook as usize;
    let orig_stack_arg_size = unsafe { MyDecoder::new(orig_addr) }.stack_argument_size();
    let trampoline_addr = create_trampoline(orig_addr, &mut page);
    let interceptor_addr = copy_interceptor(&mut page);
    let orig_bytes = get_orig_bytes(orig_addr);
    let hook = Box::leak(Box::new(Hook {
        orig_addr,
        trampoline_addr,
        interceptor_addr,
        hook_addr,
        orig_bytes,
        orig_stack_arg_size,
    }));
    modify_interceptor(&mut page, hook);
    hook
}

fn create_trampoline(orig_addr: usize, map: &mut MmapMut) -> usize {
    todo!()
}

fn copy_interceptor(map: &mut MmapMut) -> usize {
    todo!()
}

fn get_orig_bytes(orig_addr: usize) -> [u8; JMP_INTERCEPTOR_BYTE_LEN] {
   todo!() 
}

fn modify_interceptor(map: &mut MmapMut, hook: &Hook) {
    todo!()
}

struct ArgsRef<'a> {
    args: &'a Args,
}
struct ArgsBoxed {
    args: Box<Args>,
}
struct Args {
    
}

fn main() {
    let mut decoder = unsafe { MyDecoder::new(test_function as usize) };
    let push = decoder.decode();
    let mut mov = decoder.decode();


    println!("fn-addr: {:#x?}", test_function as usize);
    println!("{}: {push:#x?}", push.nasm());
    println!("{}: {mov:#x?}", mov.nasm());
    mov.set_memory_displ_size(DISPL_SIZE);
    println!("{}: {mov:#x?}", mov.nasm());
    mov.set_memory_base(Register::None);
    println!("{}: {mov:#x?}", mov.nasm());
    println!("{}: {mov:#x?}", mov.nasm());

    println!("{}", unsafe { MyDecoder::new(thiscall_function as usize) }.stack_argument_size());
    println!("{}", unsafe { MyDecoder::new(print as usize) }.stack_argument_size());

    // Instruction::with1(mov.code(), MemoryOperand::new(
    //     Register::None,
    //     Register::None,
    //     0,
    //     offset,
    //
    // ))
    test_function();
}

#[cfg(target_pointer_width = "32")]
extern "thiscall" fn thiscall_function(this: *const (), _: u8, _: u16, _: u32) {

}

#[cfg(target_pointer_width = "64")]
extern "C" fn print(val: u64) {
    println!("{val:x}");
}
#[cfg(target_pointer_width = "32")]
extern "C" fn print(val: u32) {
    println!("{val:x}");
}

#[cfg(target_pointer_width = "64")]
#[unsafe(naked)]
extern "C" fn test_function() {
    naked_asm!(
        "push rax",
        "mov rdi, [rip-12]",
        "call {print}",
        "pop rax",
        "ret",
        print = sym print,
    )
}
#[cfg(target_pointer_width = "32")]
#[unsafe(naked)]
extern "C" fn test_function() {
    naked_asm!(
        "push eax",
        "mov edi, [{test_function}+1-12]",
        "call {print}",
        "pop eax",
        "ret",
        test_function = sym test_function,
        print = sym print,
    )
}

trait InstructionFormat {
    fn nasm(&self) -> String;
}
impl InstructionFormat for Instruction {
    fn nasm(&self) -> String {
        let mut s = String::new();
        let mut formatter = NasmFormatter::new();
        formatter.format(self, &mut s);
        s
    }
}
