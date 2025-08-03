use std::arch::naked_asm;
use iced_x86::{BlockEncoder, BlockEncoderOptions, Formatter, IcedError, Instruction, InstructionBlock, NasmFormatter, Register};
use crate::function_decoder::FunctionDecoder;
use crate::hook_memory_page::HookMemoryPageBuilder;
use crate::isa_abi::{IsaAbi, X86_64_SystemV};

mod function_decoder;
mod trampoline;
mod isa_abi;
mod hook_memory_page;

// +------------+
// | caller of  |    +-------------------+
// | now hooked |    | original function |
// | function   |    +-------------------+
// +------------+      • (0)
//   ^     | (1)       •  first few instructions get overwritten
//   |     | call      •  now immediately jumps to our interceptor
//   |     '----.      •  it becomes the overwritten function        .-------------.
//   |          |      •                                             |             |
//   |          v      v                                             |             v
//   |     +-------------+  (2) immediately jump to interceptor      |      +-------------+
//   |     | overwritten |-------------------------------------------'      | interceptor |
//   |     |  function   |<-.                                               +-------------+  (10)
//   |     +-------------+  |                                             (3) |   ^     '---------.
//   |            | (7)     |                    store registers & arguments  |   |               |
//   |            |         |                         create the Args-struct  |   |  return to    |
//   |            |         |        call the abi_fixer using extern "C" ABI  |   |  interceptor  |
//   |            |         |                                                 v   | (9)           |
//   |            |         |                                             +-----------+           |
//   |            |         |                                             | abi_fixer |           |
//   |            |         |                                             +-----------+           |
//   |            |         |                                             (4) |   ^               |
//   |            |         |              call hook using extern "Rust" ABI  |   |               |
//   |            |         |                                                 |   '----.          |
//   |            |         |                                                 |        |          |
//   |            |         |            can call the trampoline in order     v        |          |
//   |            |         |             to call the original function  (5) +------+  |          |
//   |            |         |         .--------------------------------------| hook |  |          |
//   |            |         |         |                                   .->+------+  |          |
//   |            |         |         v                                   |     | (8)  |          |
//   |            |         |   +------------+ contains the overwritten   |     '------'          |
//   |            |         |   | trampoline | instructions from the      |    return to          |
//   |            |         |   +------------+ original function          |    abi_fixer          |
//   |            |         |         | (6)                               |                       |
//   |            |         |         |  jump to the hooked function      |                       |
//   |            |         |         |  behind the hook-instructions     |                       |
//   |            |         '---------'                                   |                       |
//   |            |                                                       |                       |
//   |            | return to its caller, i.e. return into the hook       |                       |
//   |            '-------------------------------------------------------'                       |
//   |                                                                                            |
//   '--------------------------------------------------------------------------------------------'
//                                                                  return to the original caller
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
// ; keep the rest of the last instruction we overwrote
// c:  00 00
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

#[repr(C)]
struct Hook<IA: IsaAbi> {
    /// address of the original function that we hooked
    orig_addr: usize,
    /// address of the trampoline, which we can call to call the original function
    trampoline_addr: usize,
    /// address of the function we jump to from the original function, that
    /// calls the hook
    interceptor_addr: usize,
    /// address of the hook function that should be called instead of the original function
    hook_fn_addr: usize,
    /// original bytes of the original function that are overwritten when enabling the hook
    orig_bytes: IA::JmpInterceptorBytesArray,
    /// argument-bytes passed to the original function via the stack
    orig_stack_arg_size: u16,
}
impl<IA: IsaAbi> Default for Hook<IA> {
    fn default() -> Self {
        Self {
            orig_addr: Default::default(),
            trampoline_addr: Default::default(),
            interceptor_addr: Default::default(),
            hook_fn_addr: Default::default(),
            orig_bytes: Default::default(),
            orig_stack_arg_size: Default::default(),
        }
    }
}

impl<IA: IsaAbi> Hook<IA> {
    pub fn enable(&self) {
        let _jmp = IA::create_jmp_to_interceptor(self.interceptor_addr);
        todo!()
    }
    pub fn disable(&self) {
        todo!()
    }
}

pub struct Interceptor {
    pub instructions: Vec<Instruction>,
}

fn assemble<IA: IsaAbi>(instructions: &[Instruction], ip: u64) -> Result<Vec<u8>, IcedError> {
    let block = InstructionBlock::new(&instructions, ip);
    BlockEncoder::encode(IA::BITNESS, block, BlockEncoderOptions::NONE)
        .map(|res| res.code_buffer)
}

unsafe fn hook_function<IA: IsaAbi>(orig_addr: usize, hook_fn: for<'a> fn(&'static Hook<IA>, ArgsRef<'a>)) -> &'static Hook<IA> {
    let hook_fn_addr = hook_fn as usize;
    let orig_stack_arg_size = unsafe { FunctionDecoder::<IA>::new(orig_addr) }.stack_argument_size();

    let builder = HookMemoryPageBuilder::<IA>::new();

    let trampoline = unsafe { trampoline::create_trampoline::<IA>(orig_addr) };
    let builder = builder.trampoline(trampoline);

    let interceptor = unsafe { IA::create_interceptor(builder.hook_struct_offset(), orig_stack_arg_size) };
    let mut builder = builder.interceptor(interceptor);

    let orig_bytes = get_orig_bytes::<IA>(orig_addr);
    let hook = Hook {
        orig_addr,
        trampoline_addr: builder.trampoline_addr(),
        interceptor_addr: builder.interceptor_addr(),
        hook_fn_addr,
        orig_bytes,
        orig_stack_arg_size,
    };
    builder.set_hook_struct(hook);

    builder.finalize()
}

fn get_orig_bytes<IA: IsaAbi>(_orig_addr: usize) -> IA::JmpInterceptorBytesArray {
   todo!() 
}

#[expect(unused)]
struct ArgsRef<'a> {
    args: &'a Args,
}
#[expect(unused)]
struct ArgsBoxed {
    args: Box<Args>,
}
struct Args {
    
}

fn main() {
    let mut decoder = unsafe { FunctionDecoder::<X86_64_SystemV>::new(test_function as usize) };
    let push = decoder.decode();
    let mut mov = decoder.decode();

    println!("fn-addr: {:#x?}", test_function as usize);
    push.print();
    mov.print();
    mov.set_memory_displ_size(X86_64_SystemV::DISPL_SIZE);
    mov.print();
    mov.set_memory_base(Register::None);
    mov.print();

    // println!("{}", unsafe { FunctionDecoder::new(thiscall_function as usize) }.stack_argument_size());
    println!("{}", unsafe { FunctionDecoder::<X86_64_SystemV>::new(print as usize) }.stack_argument_size());

    // Instruction::with1(mov.code(), MemoryOperand::new(
    //     Register::None,
    //     Register::None,
    //     0,
    //     offset,
    //
    // ))
    test_function();

    let hook = unsafe { hook_function(test_function as usize, custom_hook::<X86_64_SystemV>) };
    hook.enable();
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

fn custom_hook<IA: IsaAbi>(hook: &'static Hook<IA>, _args: ArgsRef) {
    hook.disable();
}

trait InstructionFormat {
    fn nasm(&self) -> String;
    fn bytes(&self) -> String;
    fn print(&self);
}
impl InstructionFormat for Instruction {
    fn nasm(&self) -> String {
        let mut s = String::new();
        let mut formatter = NasmFormatter::new();
        formatter.format(self, &mut s);
        s
    }

    fn bytes(&self) -> String {
        match assemble::<X86_64_SystemV>(&[self.clone()], self.ip()) {
            Ok(bytes) => {
                let mut s = String::new();
                for byte in bytes {
                    s.push_str(&format!("{byte:02x} "));
                }
                s.pop();
                s
            }
            Err(e) => format!("invalid opcode: {e}"),
        }
    }

    fn print(&self) {
        println!("{:<36}    {}: {:#?}", self.bytes(), self.nasm(), self);
    }
}
