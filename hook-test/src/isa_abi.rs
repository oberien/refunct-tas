use std::ffi::c_void;
use std::mem::offset_of;
use iced_x86::code_asm::{CodeAssembler, ptr, r10, r11, r8, r9, rax, rbp, rcx, rdi, rdx, rsi, rsp, xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7};
use crate::{assemble, Interceptor, Hook};
use crate::args::{Args, ArgsContext, ArgsRef};

pub unsafe trait IsaAbi: 'static {
    const BITNESS: u32;
    const DISPL_SIZE: u32 = Self::BITNESS / 8;
    /// SAFETY: must be [u8; Self::JMP_INTERCEPTOR_BYTE_LEN]
    type JmpInterceptorBytesArray: Array;
    type Args: Args;

    /// SAFETY: must be correct and valid for the ISA
    fn create_jmp_to_interceptor(interceptor_addr: usize) -> Self::JmpInterceptorBytesArray;
    /// SAFETY: must be correct and valid for the ISA & ABI
    fn create_interceptor(hook_struct_addr: usize, stack_arg_size: u16) -> Interceptor;
    /// # Safety
    /// * code on the memory pages containing the requested bytes must not be executed while this function is running
    unsafe fn make_rw(addr: usize, len: usize);
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
#[derive(Debug, Clone)]
pub struct X86_64_SystemV_Args {
    /// xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7
    xmm: [u128; 8],
    /// rdi, rsi, rdx, rcx, r8, r9
    args: [u64; 6],
    r10: u64,
    r11: u64,
    /// return value to be returned to the original caller
    return_value: u64,
}

unsafe impl Args for X86_64_SystemV_Args {
    fn next_int_arg(&mut self, ctx: &ArgsContext) -> *mut usize {
        &mut self.args[ctx.int_args_consumed()] as *mut u64 as *mut usize
    }

    fn next_float_arg(&mut self, ctx: &ArgsContext) -> *mut f32 {
        &mut self.xmm[ctx.float_args_consumed()] as *mut u128 as *mut f32
    }

    fn set_return_value(&mut self, ret_val: usize) {
        self.return_value = ret_val as u64;
    }
}

unsafe impl IsaAbi for X86_64_SystemV {
    const BITNESS: u32 = 64;
    type JmpInterceptorBytesArray = [u8; 12];
    type Args = X86_64_SystemV_Args;

    fn create_jmp_to_interceptor(interceptor_addr: usize) -> Self::JmpInterceptorBytesArray {
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();
        a.mov(rax, interceptor_addr as u64).unwrap();
        a.jmp(rax).unwrap();
        let buffer = assemble::<Self>(a.instructions(), 0).unwrap();
        buffer.try_into().unwrap()
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
        a.add(rsp, 0x80 + 0x48).unwrap();
        // restore the return value if the original function was called
        a.pop(rax).unwrap();
        // function epilogue
        a.pop(rbp).unwrap();
        if stack_arg_size == 0 {
            a.ret().unwrap();
        } else {
            a.ret_1(stack_arg_size as u32).unwrap();
        }

        Interceptor {
            instructions: a.take_instructions(),
        }
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
    a.push(r11).unwrap();
    a.push(r10).unwrap();
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
