use iced_x86::code_asm::CodeAssembler;
use iced_x86::{IcedError, Register};
use crate::{Interceptor, RawHook, CallTrampoline};
use crate::args::{Args, ArgsRef};

mod x86_64_systemv;
// mod i686_thiscall;

pub use x86_64_systemv::X86_64_SystemV;

#[allow(private_interfaces)]
pub unsafe trait IsaAbi: 'static {
    const BITNESS: u32;
    const DISPL_SIZE: u32 = Self::BITNESS / 8;
    /// SAFETY: must be [u8; Self::JMP_INTERCEPTOR_BYTE_LEN]
    type JmpInterceptorBytesArray: Array;
    type Args: Args;
    type AsmRegister: Into<Register> + Copy;

    /// List of unused caller-saved general purpose pointer-sized scratch registers
    ///
    /// SAFETY: implementation must be correct for the ISA & ABI
    fn free_registers() -> &'static [Self::AsmRegister];
    fn create_mov_reg_addr(a: &mut CodeAssembler, reg: Self::AsmRegister, addr: usize) -> Result<(), IcedError>;
    fn create_jmp_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError>;
    fn create_call_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError>;

    /// SAFETY: implementation must be correct and valid for the ISA
    fn create_jmp_to_interceptor(interceptor_addr: usize) -> Self::JmpInterceptorBytesArray;
    /// SAFETY: implementation must be correct and valid for the ISA & ABI
    /// # Safety
    /// * T must be the T that the `Hook` at `hook_struct_addr` uses
    unsafe fn create_interceptor<T: 'static>(hook_struct_addr: usize, stack_arg_size: u16) -> Interceptor;
    /// SAFETY: implementation must be correct and valid for the ISA & ABI
    fn create_call_trampoline(trampoline_addr: usize) -> CallTrampoline;
    /// SAFETY: implementation must be correct for the OS
    /// # Safety
    /// * code on the memory pages containing the requested bytes must not be executed while this function is running
    unsafe fn make_rw(addr: usize, len: usize);
    /// SAFETY: implementation must be correct for the OS
    /// # Safety
    /// * the memory pages containing the requested bytes must not be written to while this function is running
    unsafe fn make_rx(addr: usize, len: usize);
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

extern "C" fn abi_fixer<IA: IsaAbi, T>(hook: &'static RawHook<IA, T>, args_ref: ArgsRef<'_, IA>) {
    (hook.hook_fn)(hook, args_ref)
}
