use std::slice;
use iced_x86::{BlockEncoder, BlockEncoderOptions, IcedError, Instruction, InstructionBlock};
use crate::isa_abi::Array;

mod args;
mod hook;
mod function_decoder;
mod trampoline;
mod isa_abi;
mod hook_memory_page;

pub use args::{ArgsRef, ArgsBoxed};
pub use isa_abi::{IsaAbi, X86_64_SystemV, I686_MSVC_Thiscall};
pub use hook::{RawHook, TypedHook};

// # Design Overview
//
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
// # Function ABIs
//
// +--------------------+---------------------+------------------+
// | function           | i686 MSVC thiscall  | x86_64 System V  |
// +--------------------+---------------------+------------------+
// | original function  | "thiscall"          | "sysv64"         |
// | interceptor        | "thiscall"          | "sysv64"         |
// | abi_fixer          | "fastcall"          | "sysv64"         |
// | hook               | "Rust"              | "Rust"           |
// | call_trampoline    | "C" -> "cdecl"      | "C" -> "sysv64"  |
// | trampoline         | "thiscall"          | "sysv64"         |
// +--------------------+---------------------+------------------+
//
// # ABI Definitions
//
// References:
// * <https://wiki.osdev.org/System_V_ABI>
// * <https://en.wikipedia.org/wiki/X86_calling_conventions>
// * <https://doc.rust-lang.org/reference/items/external-blocks.html#abi>
//
// +-----------------+-----------------+-------------------------+--------------------+------------------+
// | Register        | x86 MSVC cdecl  | x86 MSVC thiscall       | x86 MSVC fastcall  | x86_64 System V  |
// +-----------------+-----------------+-------------------------+--------------------+------------------+
// | eax / rax       | Return value    | Return value            | Return value       | Return value     |
// | ebx / rbx       | Callee-saved    | Callee-saved            | Callee-saved       | Callee-saved     |
// | ecx / rcx       | scratch reg     | this-pointer (1st arg)  | 1st argument       | 4th int arg      |
// | edx / rdx       | scratch reg     | 2nd arg (in some MSVC)  | 2nd argument       | 3rd int arg      |
// | esi / rsi       | Callee-saved    | Callee-saved            | Callee-saved       | 2nd int arg      |
// | edi / rdi       | Callee-saved    | Callee-saved            | Callee-saved       | 1st int arg      |
// | ebp / rbp       | Callee-saved    | Callee-saved            | Callee-saved       | Callee-saved     |
// | esp / rsp       | Stack pointer   | Stack pointer           | Stack pointer      | Stack pointer    |
// | r8              |                 |                         |                    | 5th int arg      |
// | r9              |                 |                         |                    | 6th int arg      |
// | r10–r11         |                 |                         |                    | scratch reg      |
// | r12–r15         |                 |                         |                    | Callee-saved     |
// | xmm0-xmm7       |                 |                         |                    | float args       |
// | xmm8–xmm15      |                 |                         |                    | scratch reg      |
// | more args       | stack rtl       | stack rtl               | stack rtl          | stack rtl        |
// | stack cleanup   | caller          | callee                  | callee             | caller           |
// | stack align     | 4 bytes (GCC 16)| 4 bytes                 | 4 bytes            | 16 bytes         |
// | frame pointers  | default yes     | default yes             | default yes        | default yes      |
// +-----------------+-----------------+-------------------------+--------------------+------------------+

struct Interceptor {
    pub instructions: Vec<Instruction>,
}
struct CallTrampoline {
    pub instructions: Vec<Instruction>,
}

fn assemble<IA: IsaAbi>(instructions: &[Instruction], ip: u64) -> Result<Vec<u8>, IcedError> {
    let block = InstructionBlock::new(&instructions, ip);
    BlockEncoder::encode(IA::BITNESS, block, BlockEncoderOptions::NONE)
        .map(|res| res.code_buffer)
}

unsafe fn get_orig_bytes<IA: IsaAbi>(orig_addr: usize) -> IA::JmpInterceptorBytesArray {
    let slice = unsafe { slice::from_raw_parts(orig_addr as *const u8, IA::JmpInterceptorBytesArray::LEN) };
    IA::JmpInterceptorBytesArray::load_from(slice)
}
