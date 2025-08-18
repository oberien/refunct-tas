use iced_x86::code_asm::CodeAssembler;
use iced_x86::{IcedError, Register};
use crate::{Interceptor, CallTrampoline};
use crate::args::{Args};

mod x86_64_systemv;
mod i686_msvc_thiscall;

pub use x86_64_systemv::X86_64_SystemV;
pub use i686_msvc_thiscall::I686_MSVC_Thiscall;

#[allow(private_interfaces)]
pub unsafe trait IsaAbi: 'static {
    /// Number of bits of a `usize`
    const BITNESS: u32;
    /// Memory Displacement Size
    const DISPL_SIZE: u32 = Self::BITNESS / 8;
    /// Array large enough to store the assembled instruction bytes to jump
    /// from the original function to the interceptor
    type JmpInterceptorBytesArray: Array;
    /// Args-Struct representing the arguments pushed to / on the stack
    type Args: Args;
    /// Register type corresponding to the bitness on the target platform
    type AsmRegister: Into<Register> + Copy;

    /// List of unused scratch registers
    ///
    /// These should be caller-saved general purpose pointer-sized registers, that are not
    /// used for arguments or other ABI data.
    /// One of them will be used for register-indirect calls and jumps created via
    /// `Self::create_mov_reg_addr`, `Self::create_jmp_reg` and `Self::create_call_reg`.
    ///
    /// SAFETY: implementation must return correct registers for the ISA & ABI
    fn free_registers() -> &'static [Self::AsmRegister];
    /// Add a `mov <reg>, <addr>` instruction to the `CodeAssembler`
    ///
    /// SAFETY: must add only that instruction with exactly the provided register and address
    fn create_mov_reg_addr(a: &mut CodeAssembler, reg: Self::AsmRegister, addr: usize) -> Result<(), IcedError>;
    /// Add a `jmp <reg>` instruction to the `CodeAssembler`
    ///
    /// SAFETY: must add only that instruction with exactly the provided register
    fn create_jmp_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError>;
    /// Add a `call <reg>` instruction to the `CodeAssembler`
    ///
    /// SAFETY: must add only that instruction with exactly the provided register
    fn create_call_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError>;

    /// Create and assemble something like `mov rax, addr; jmp rax`, returning the assembled byte-array
    ///
    /// SAFETY: implementation must be correct and valid for the ISA & ABI
    fn create_jmp_to_interceptor(interceptor_addr: usize) -> Self::JmpInterceptorBytesArray;
    /// Create the instructions of the interceptor.
    ///
    /// The interceptor is jumped to if the original hooked function is called.
    ///
    /// The interceptor:
    /// * has the same ABI as the original function
    /// * has a function prologue
    /// * stores all callee-saved registers of the ABI
    /// * stores the Args-struct on the stack
    /// * calls the abi-fixer providing 2 arguments (including stack-alignment if needed)
    ///     * first arg: provided `hook_struct_addr`
    ///     * second arg: pointer to the Args-struct on the stack
    /// * reverts possible stack alignment
    /// * cleans up the stack
    /// * stores the return value stored from the Args-struct in the return-value register / location
    /// * restores callee-saved registers
    /// * has a function epilogue
    /// * returns to the caller (possibly using `retn <stack_arg_size>`)
    ///
    /// SAFETY: implementation must be correct and valid for the ISA & ABI
    ///
    /// # Safety
    /// * T must be the T that the `Hook` at `hook_struct_addr` uses
    unsafe fn create_interceptor<T: 'static>(hook_struct_addr: usize, stack_arg_size: u16) -> Interceptor;
    /// Create instructions for an `extern "C" call_trampoline(&Args)` function
    ///
    /// The `call_trampoline` function is called if the user wants to call the original function.
    /// It restores the (possibly modified) arguments to the original function to the corresponding
    /// argument registers / stack and calls the original function.
    /// It stores the return value of the original function into the passed `Args`-struct.
    ///
    /// The `call_trampoline` function:
    /// * must be `extern "C"` (e.g. `cdecl` on `i686-pc-windows-msvc` or
    ///   `sysv64` on `x86_64-unknown-linux-gnu`)
    /// * has a function prologue
    /// * restores all arguments to their registers and possibly stack
    /// * calls the original function in its original calling convention (including stack alignment if needed)
    /// * reverts possible stack alignment
    /// * if needed cleans up the stack
    /// * stores the return value in the `Args`-struct originally provided to it as parameter
    /// * has a function epilogue
    /// * returns
    ///
    /// SAFETY: implementation must return an `extern "C" fn` that correctly calls the provided
    ///         trampoline validly for the ISA & ABI
    fn create_call_trampoline(trampoline_addr: usize, stack_arg_size: u16) -> CallTrampoline;
}

pub trait Array {
    const LEN: usize;

    fn load_from(slice: &[u8]) -> Self;
    fn store_to(&self, slice: &mut [u8]);
    fn as_slice(&self) -> &[u8];
}
impl<const N: usize> Array for [u8; N] {
    const LEN: usize = N;

    fn load_from(slice: &[u8]) -> Self {
        slice[..N].try_into().unwrap()
    }
    fn store_to(&self, slice: &mut [u8]) {
        slice[..N].copy_from_slice(self)
    }
    fn as_slice(&self) -> &[u8] {
        self
    }
}
