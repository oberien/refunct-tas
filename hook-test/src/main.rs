use std::arch::naked_asm;
use std::{mem, slice};
use iced_x86::{BlockEncoder, BlockEncoderOptions, IcedError, Instruction, InstructionBlock};
use crate::args::ArgsRef;
use crate::function_decoder::FunctionDecoder;
use crate::hook_memory_page::HookMemoryPageBuilder;
use crate::isa_abi::{Array, IsaAbi, X86_64_SystemV};

mod args;
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
//   |     |  function   |<-.                                               +-------------+  (12)
//   |     +-------------+  |                                             (3) |   ^     '---------.
//   |            | (8)     |                    store registers & arguments  |   |               |
//   |            |         |                         create the Args-struct  |   |  return to    |
//   |            |         |        call the abi_fixer using extern "C" ABI  |   |  interceptor  |
//   |            |         |                                                 v   | (11)          |
//   |            |         |                                             +-----------+           |
//   |            |         |                                             | abi_fixer |           |
//   |            |         |                                             +-----------+           |
//   |            |         |                                             (4) |   ^               |
//   |            |         |              call hook using extern "Rust" ABI  |   |               |
//   |            |         |                                                 |   '----.          |
//   |            |         |                                                 |        |          |
//   |            |         |              can call the trampoline in order   |        |          |
//   |            |         |                 to call the original function   v        |          |
//   |            |         |                  +-----------------+       (5) +------+  |          |
//   |            |         |         .--------| call_trampoline |<----------| hook |  |          |
//   |            |         |         | (6)    +-----------------+---------->+------+  |          |
//   |            |         |         |  restore saved         ^   (9) ret      | (10) |          |
//   |            |         |         |  regs and args         |                '------'          |
//   |            |         |         |                        '----------.    return to          |
//   |            |         |         v                                   |    abi_fixer          |
//   |            |         |   +------------+ contains the overwritten   |                       |
//   |            |         |   | trampoline | instructions from the      |                       |
//   |            |         |   +------------+ original function          |                       |
//   |            |         |         | (7)                               |                       |
//   |            |         |         |  jump to the hooked function      |                       |
//   |            |         |         |  behind the hook-instructions     |                       |
//   |            |         '---------'                                   |                       |
//   |            |                                                       |                       |
//   |            | return to call_trampoline                             |                       |
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
    /// `extern "C" fn(&Args)` to restore registers and args and call the trampoline
    call_trampoline_addr: usize,
    /// function pointer of the hook function that should be called instead of the original function
    hook_fn: for<'a> fn(&'static Hook<IA>, ArgsRef<'a, IA>),
    /// original bytes of the original function that are overwritten when enabling the hook
    orig_bytes: IA::JmpInterceptorBytesArray,
    /// argument-bytes passed to the original function via the stack
    orig_stack_arg_size: u16,
}

impl<IA: IsaAbi> Hook<IA> {
    pub fn enable(&self) {
        let jmp = IA::create_jmp_to_interceptor(self.interceptor_addr);
        unsafe { IA::make_rw(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
        let slice = unsafe { slice::from_raw_parts_mut(self.orig_addr as *mut u8, IA::JmpInterceptorBytesArray::LEN) };
        jmp.store_to(slice);
        unsafe { IA::make_rx(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
    }
    pub fn disable(&self) {
        let slice = unsafe { slice::from_raw_parts_mut(self.orig_addr as *mut u8, IA::JmpInterceptorBytesArray::LEN) };
        unsafe { IA::make_rw(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
        slice.copy_from_slice(self.orig_bytes.as_slice());
        unsafe { IA::make_rx(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
    }
    pub fn call_original_function(&self, args: &IA::Args) {
        unsafe {
            let function: extern "C" fn(&IA::Args) = mem::transmute(self.call_trampoline_addr);
            function(args)
        }
    }
}

pub struct Interceptor {
    pub instructions: Vec<Instruction>,
}
pub struct CallTrampoline {
    pub instructions: Vec<Instruction>,
}

fn assemble<IA: IsaAbi>(instructions: &[Instruction], ip: u64) -> Result<Vec<u8>, IcedError> {
    let block = InstructionBlock::new(&instructions, ip);
    BlockEncoder::encode(IA::BITNESS, block, BlockEncoderOptions::NONE)
        .map(|res| res.code_buffer)
}

unsafe fn hook_function<IA: IsaAbi>(orig_addr: usize, hook_fn: for<'a> fn(&'static Hook<IA>, ArgsRef<'a, IA>)) -> &'static Hook<IA> {
    let orig_stack_arg_size = unsafe { FunctionDecoder::<IA>::new(orig_addr) }.stack_argument_size();

    let builder = HookMemoryPageBuilder::<IA>::new();

    let trampoline = unsafe { trampoline::create_trampoline::<IA>(orig_addr) };
    let builder = builder.trampoline(trampoline);

    let interceptor = IA::create_interceptor(builder.hook_struct_addr(), orig_stack_arg_size);
    let builder = builder.interceptor(interceptor);

    let call_trampoline = IA::create_call_trampoline(builder.trampoline_addr());
    let builder = builder.call_trampoline(call_trampoline);

    let orig_bytes = unsafe { get_orig_bytes::<IA>(orig_addr) };
    let hook = Hook {
        orig_addr,
        trampoline_addr: builder.trampoline_addr(),
        interceptor_addr: builder.interceptor_addr(),
        call_trampoline_addr: builder.call_trampoline_addr(),
        hook_fn,
        orig_bytes,
        orig_stack_arg_size,
    };
    builder.finalize(hook)
}

unsafe fn get_orig_bytes<IA: IsaAbi>(orig_addr: usize) -> IA::JmpInterceptorBytesArray {
    let slice = unsafe { slice::from_raw_parts(orig_addr as *const u8, IA::JmpInterceptorBytesArray::LEN) };
    IA::JmpInterceptorBytesArray::load_from(slice)
}

fn main() {
    let hook = unsafe { hook_function(test_function as usize, custom_hook::<X86_64_SystemV>) };

    test_function(1337);
    hook.enable();
    test_function(42);
    test_function(21);
}

fn custom_hook<IA: IsaAbi>(hook: &'static Hook<IA>, mut args: ArgsRef<'_, IA>) {
    let arg = args.without_this_pointer::<u32>();
    println!("from inside the hook; original argument: {arg}");
    hook.call_original_function(args.as_args());
    let arg = args.without_this_pointer::<u32>();
    println!("setting argument to 314");
    *arg = 314;
    hook.call_original_function(args.as_args());
    println!("disabling the hook within the hook");
    hook.disable();
}
#[cfg(target_pointer_width = "64")]
extern "C" fn print(val: u64) {
    println!("from inside hooked function: {val}");
}
#[unsafe(link_section = ".custom_section")]
#[cfg(target_pointer_width = "64")]
#[unsafe(naked)]
extern "C" fn test_function(_arg: u32) {
    naked_asm!(
        "push rax",
        "call {print}",
        "pop rax",
        "ret",
        print = sym print,
    )
}


#[cfg(target_pointer_width = "32")]
extern "thiscall" fn thiscall_function(this: *const (), _: u8, _: u16, _: u32) {

}

#[cfg(target_pointer_width = "32")]
extern "C" fn print(val: u32) {
    println!("{val:x}");
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
