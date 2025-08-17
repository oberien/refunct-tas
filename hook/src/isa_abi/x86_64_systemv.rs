use std::mem::offset_of;
use iced_x86::code_asm::{AsmRegister64, CodeAssembler, ptr, r10, r11, r12, r13, r14, r15, r8, r9, rax, rbp, rbx, rcx, rdi, rdx, rsi, rsp, xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7};
use iced_x86::IcedError;
use crate::args::{Args, ArgsLoadContext, ArgsStoreContext};
use crate::{ArgsRef, assemble, CallTrampoline, Interceptor, IsaAbi, RawHook};

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
impl AsRef<X86_64_SystemV_Args> for X86_64_SystemV_Args {
    fn as_ref(&self) -> &X86_64_SystemV_Args {
        self
    }
}

unsafe impl Args for X86_64_SystemV_Args {
    fn new() -> Self {
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

    unsafe fn create_interceptor<T: 'static>(hook_struct_addr: usize, stack_arg_size: u16) -> Interceptor {
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();

        #[cfg(not(target_pointer_width = "32"))]
        extern "sysv64" fn abi_fixer<T>(hook: &'static RawHook<X86_64_SystemV, T>, args_ref: ArgsRef<'_, X86_64_SystemV>) {
            (hook.hook_fn)(hook, args_ref)
        }
        #[cfg(target_pointer_width = "32")]
        extern "fastcall" fn abi_fixer<T>(_hook: &'static RawHook<X86_64_SystemV, T>, _args_ref: ArgsRef<'_, X86_64_SystemV>) {
            unreachable!("X86_64_SystemV is only supported on 64-bit targets")
        }

        // function prologue with frame pointer
        a.push(rbp).unwrap();
        a.mov(rbp, rsp).unwrap();
        // store callee-saved registers
        a.push(rbx).unwrap();
        a.push(r12).unwrap();
        a.push(r13).unwrap();
        a.push(r14).unwrap();
        a.push(r15).unwrap();
        // space for the return value
        a.push(rax).unwrap();
        // store all argument registers
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
        // setup `Hook` and `Args` arguments for `extern "C" abi_fixer`-call
        a.mov(rdi, hook_struct_addr as u64).unwrap();
        a.mov(rsi, rsp).unwrap();
        // call interceptor
        a.mov(rax, abi_fixer::<T> as u64).unwrap();
        // no stack alignment needed; ret-addr + 13 registers + 0x80 xmm
        // a.sub(rsp, 0x8).unwrap();
        a.call(rax).unwrap();
        // no undo stack alignment needed
        // a.sub(rsp, 0x8).unwrap();
        // cleanup the stack
        a.add(rsp, 0x80 + 0x30).unwrap();
        // restore the return value
        a.pop(rax).unwrap();
        // restore callee-saved registers
        a.pop(r15).unwrap();
        a.pop(r14).unwrap();
        a.pop(r13).unwrap();
        a.pop(r12).unwrap();
        a.pop(rbx).unwrap();
        // function epilogue
        a.pop(rbp).unwrap();
        if stack_arg_size == 0 {
            a.ret().unwrap();
        } else {
            a.ret_1(stack_arg_size as u32).unwrap();
        }

        Interceptor { instructions: a.take_instructions() }
    }

    fn create_call_trampoline(trampoline_addr: usize, stack_arg_size: u16) -> CallTrampoline {
        assert_eq!(stack_arg_size, 0, "stack-arguments are currently not supported on x86_64 SystemV");
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();

        // `call_trampoline` is an `extern "C" fn` -> `extern "sysv64" fn`

        // function prologue
        a.push(rbp).unwrap();
        a.mov(rbp, rsp).unwrap();
        // store callee-saved registers
        a.push(rbx).unwrap();
        a.push(r12).unwrap();
        a.push(r13).unwrap();
        a.push(r14).unwrap();
        a.push(r15).unwrap();
        // save argument for later (for storing the return-value)
        a.push(rdi).unwrap();
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
        // no stack alignment needed; ret-addr + 7 registers
        // a.sub(rsp, 8).unwrap();
        a.call(rax).unwrap();
        // no undo align stack needed
        // a.add(rsp, 8).unwrap();
        // store return value
        a.pop(rdi).unwrap();
        a.mov(ptr(rdi + offset_of!(Self::Args, return_value)), rax).unwrap();
        // restore callee-saved registers
        a.pop(r15).unwrap();
        a.pop(r14).unwrap();
        a.pop(r13).unwrap();
        a.pop(r12).unwrap();
        a.pop(rbx).unwrap();
        // function epilogue
        a.pop(rbp).unwrap();
        a.ret().unwrap();

        CallTrampoline { instructions: a.take_instructions() }
    }
}
