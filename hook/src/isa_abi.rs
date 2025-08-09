use std::ffi::c_void;
use std::mem::offset_of;
use iced_x86::code_asm::{CodeAssembler, ptr, r8, r9, rax, rbp, rcx, rdi, rdx, rsi, rsp, xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7, AsmRegister64, r10, r11};
use iced_x86::{IcedError, Register};
use crate::{assemble, Interceptor, Hook, CallTrampoline};
use crate::args::{Args, ArgsLoadContext, ArgsRef, ArgsStoreContext};

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
    /// SAFETY: mplementation must be correct and valid for the ISA & ABI
    fn create_interceptor(hook_struct_addr: usize, stack_arg_size: u16) -> Interceptor;
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

extern "C" fn abi_fixer<IA: IsaAbi>(hook: &'static Hook<IA>, args_ref: ArgsRef<'_, IA>) {
    (hook.hook_fn)(hook, args_ref)
}

#[allow(non_camel_case_types)]
pub struct X86_64_SystemV;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct X86_64_SystemV_Args {
    /// xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7
    xmm: [u128; 8],
    /// rdi, rsi, rdx, rcx, r8, r9
    args: [u64; 6],
    /// return value to be returned to the original caller
    return_value: u64,
}

unsafe impl Args for X86_64_SystemV_Args {
    type This = Self;
    fn new() -> Self::This {
        Self::default()
    }
    fn next_int_arg(&mut self, ctx: &ArgsLoadContext) -> *mut usize {
        &raw mut self.args[ctx.int_args_consumed()] as *mut usize
    }
    fn next_float_arg(&mut self, ctx: &ArgsLoadContext) -> *mut f32 {
        &raw mut self.xmm[ctx.float_args_consumed()] as *mut f32
    }

    fn set_next_int_arg(&mut self, val: usize, ctx: &ArgsStoreContext) {
        self.args[ctx.int_args_stored()] = val as u64;
    }
    fn set_next_float_arg(&mut self, val: f32, ctx: &ArgsStoreContext) {
        self.xmm[ctx.float_args_stored()] = val.to_bits() as u128;
    }

    fn return_value(&self) -> usize {
        self.return_value.try_into().unwrap()
    }
    fn set_return_value(&mut self, ret_val: usize) {
        self.return_value = ret_val as u64;
    }
}

#[allow(private_interfaces)]
unsafe impl IsaAbi for X86_64_SystemV {
    const BITNESS: u32 = 64;
    type JmpInterceptorBytesArray = [u8; 12];
    type Args = X86_64_SystemV_Args;
    type AsmRegister = AsmRegister64;

    fn free_registers() -> &'static [Self::AsmRegister] {
        &[rax, r10, r11]
    }
    fn create_mov_reg_addr(a: &mut CodeAssembler, reg: Self::AsmRegister, addr: usize) -> Result<(), IcedError> {
        a.mov(reg, addr as u64)
    }
    fn create_jmp_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError> {
        a.jmp(reg)
    }
    fn create_call_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError> {
        a.call(reg)
    }

    fn create_jmp_to_interceptor(interceptor_addr: usize) -> Self::JmpInterceptorBytesArray {
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();
        Self::create_mov_reg_addr(&mut a, rax, interceptor_addr).unwrap();
        Self::create_jmp_reg(&mut a, rax).unwrap();
        assemble::<Self>(a.instructions(), 0).unwrap().try_into().unwrap()
    }

    fn create_interceptor(hook_struct_addr: usize, stack_arg_size: u16) -> Interceptor {
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();

        // function prologue with frame pointer
        a.push(rbp).unwrap();
        a.mov(rbp, rsp).unwrap();
        // space for the return value
        a.push(rax).unwrap();
        // store all registers
        pushall_x86_64_system_v(&mut a);
        // setup `Hook` and `Args` arguments
        a.mov(rdi, hook_struct_addr as u64).unwrap();
        a.mov(rsi, rsp).unwrap();
        // call interceptor
        a.mov(rax, abi_fixer::<X86_64_SystemV> as u64).unwrap();
        align_stack_pre_x86_64_system_v(&mut a);
        a.call(rax).unwrap();
        align_stack_post_x86_64_system_v(&mut a);
        // cleanup the stack
        a.add(rsp, 0x80 + 0x30).unwrap();
        // restore the return value if the original function was called
        a.pop(rax).unwrap();
        // function epilogue
        a.pop(rbp).unwrap();
        if stack_arg_size == 0 {
            a.ret().unwrap();
        } else {
            a.ret_1(stack_arg_size as u32).unwrap();
        }

        Interceptor { instructions: a.take_instructions() }
    }

    fn create_call_trampoline(trampoline_addr: usize) -> CallTrampoline {
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();

        // function prologue
        a.push(rbp).unwrap();
        a.mov(rbp, rsp).unwrap();
        a.push(rdi).unwrap();
        // align stack
        a.sub(rsp, 8).unwrap();
        // restore all registers from Args
        a.movdqu(xmm0, ptr(rdi + offset_of!(Self::Args, xmm) + 0x0)).unwrap();
        a.movdqu(xmm1, ptr(rdi + offset_of!(Self::Args, xmm) + 0x10)).unwrap();
        a.movdqu(xmm2, ptr(rdi + offset_of!(Self::Args, xmm) + 0x20)).unwrap();
        a.movdqu(xmm3, ptr(rdi + offset_of!(Self::Args, xmm) + 0x30)).unwrap();
        a.movdqu(xmm4, ptr(rdi + offset_of!(Self::Args, xmm) + 0x40)).unwrap();
        a.movdqu(xmm5, ptr(rdi + offset_of!(Self::Args, xmm) + 0x50)).unwrap();
        a.movdqu(xmm6, ptr(rdi + offset_of!(Self::Args, xmm) + 0x60)).unwrap();
        a.movdqu(xmm7, ptr(rdi + offset_of!(Self::Args, xmm) + 0x70)).unwrap();
        a.mov(rsi, ptr(rdi + offset_of!(Self::Args, args) + 0x08)).unwrap();
        a.mov(rdx, ptr(rdi + offset_of!(Self::Args, args) + 0x10)).unwrap();
        a.mov(rcx, ptr(rdi + offset_of!(Self::Args, args) + 0x18)).unwrap();
        a.mov(r8, ptr(rdi + offset_of!(Self::Args, args) + 0x20)).unwrap();
        a.mov(r9, ptr(rdi + offset_of!(Self::Args, args) + 0x28)).unwrap();
        // restore rdi last to not overwrite our pointer
        a.mov(rdi, ptr(rdi + offset_of!(Self::Args, args) + 0x0)).unwrap();
        // call original function
        a.mov(rax, trampoline_addr as u64).unwrap();
        a.call(rax).unwrap();
        // undo align stack
        a.add(rsp, 8).unwrap();
        // store return value
        a.pop(rdi).unwrap();
        a.mov(ptr(rdi + offset_of!(Self::Args, return_value)), rax).unwrap();
        // function epilogue
        a.pop(rbp).unwrap();
        a.ret().unwrap();

        CallTrampoline { instructions: a.take_instructions() }
    }

    unsafe fn make_rw(addr: usize, len: usize) {
        let start_page = addr & !0xfff;
        let end_page = (addr + len) & !0xfff;
        let len = end_page - start_page + 0x1000;
        let page = start_page as *mut c_void;
        unsafe { libc::mprotect(page, len, libc::PROT_READ | libc::PROT_WRITE); }
    }
    unsafe fn make_rx(addr: usize, len: usize) {
        let start_page = addr & !0xfff;
        let end_page = (addr + len) & !0xfff;
        let len = end_page - start_page + 0x1000;
        let page = start_page as *mut c_void;
        unsafe { libc::mprotect(page, len, libc::PROT_READ | libc::PROT_EXEC); }
    }
}

fn pushall_x86_64_system_v(a: &mut CodeAssembler) {
    a.push(r9).unwrap();
    a.push(r8).unwrap();
    a.push(rcx).unwrap();
    a.push(rdx).unwrap();
    a.push(rsi).unwrap();
    a.push(rdi).unwrap();
    a.sub(rsp, 0x80).unwrap();
    a.movdqu(ptr(rsp + 0x70), xmm7).unwrap();
    a.movdqu(ptr(rsp + 0x60), xmm6).unwrap();
    a.movdqu(ptr(rsp + 0x50), xmm5).unwrap();
    a.movdqu(ptr(rsp + 0x40), xmm4).unwrap();
    a.movdqu(ptr(rsp + 0x30), xmm3).unwrap();
    a.movdqu(ptr(rsp + 0x20), xmm2).unwrap();
    a.movdqu(ptr(rsp + 0x10), xmm1).unwrap();
    a.movdqu(ptr(rsp + 0x0), xmm0).unwrap();
}

fn align_stack_pre_x86_64_system_v(a: &mut CodeAssembler) {
    a.push(rbp).unwrap();
    a.mov(rbp, rsp).unwrap();
    a.and(rsp, 0xfffffff0u32 as i32).unwrap();
}
fn align_stack_post_x86_64_system_v(a: &mut CodeAssembler) {
    a.mov(rsp, rbp).unwrap();
    a.pop(rbp).unwrap();
}
