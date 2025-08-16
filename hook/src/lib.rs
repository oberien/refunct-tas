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
pub use isa_abi::{IsaAbi, X86_64_SystemV};
pub use hook::{RawHook, TypedHook};

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
