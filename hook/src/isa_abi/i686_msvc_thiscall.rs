use std::mem::offset_of;
use iced_x86::code_asm::{AsmRegister32, CodeAssembler, eax, ebp, ebx, ecx, edi, edx, esi, esp, ptr, xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7};
use iced_x86::IcedError;
use crate::args::{Args, ArgsLoadContext, ArgsStoreContext};
use crate::{ArgsRef, assemble, CallTrampoline, Interceptor, IsaAbi, RawHook};

#[allow(non_camel_case_types)]
pub struct I686_MSVC_Thiscall;

const MAX_ARG_BYTES: usize = 0x60;
const MAX_ARG_NUM: usize = MAX_ARG_BYTES / size_of::<u32>();

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct I686_MSVC_Thiscall_Args {
    xmm0: u128,
    xmm1: u128,
    xmm2: u128,
    xmm3: u128,
    xmm4: u128,
    xmm5: u128,
    xmm6: u128,
    xmm7: u128,
    edi: u32,
    esi: u32,
    ecx: u32,
    ebx: u32,
    return_value: u32,
    _frame_pointer: u32,
    _return_address: u32,
    other_args: [u32; MAX_ARG_NUM],
}
impl AsRef<I686_MSVC_Thiscall_Args> for I686_MSVC_Thiscall_Args {
    fn as_ref(&self) -> &I686_MSVC_Thiscall_Args {
        self
    }
}

unsafe impl Args for I686_MSVC_Thiscall_Args {
    fn new() -> Self {
        Self::default()
    }
    fn next_int_arg(&mut self, ctx: &ArgsLoadContext) -> *mut usize {
        assert_eq!(size_of::<usize>(), size_of::<u32>());
        if ctx.int_args_consumed() == 0 {
            &raw mut self.ecx as *mut usize
        } else {
            let index = ctx.int_args_consumed() + ctx.float_args_consumed() - 1;
            &raw mut self.other_args[index] as *mut usize
        }
    }
    fn next_float_arg(&mut self, ctx: &ArgsLoadContext) -> *mut f32 {
        assert_eq!(size_of::<usize>(), size_of::<u32>());
        assert!(ctx.int_args_consumed() > 0, "must first consume the this-pointer as integer arg");
        let index = ctx.int_args_consumed() + ctx.float_args_consumed() - 1;
        &raw mut self.other_args[index] as *mut f32
    }

    fn set_next_int_arg(&mut self, val: usize, ctx: &ArgsStoreContext) {
        if ctx.int_args_stored() == 0 {
            self.ecx = val as u32;
        } else {
            let index = ctx.int_args_stored() + ctx.float_args_stored() - 1;
            self.other_args[index] = val as u32;
        }
    }
    fn set_next_float_arg(&mut self, val: f32, ctx: &ArgsStoreContext) {
        assert!(ctx.int_args_stored() > 0, "must first set the this-pointer as integer arg");
        let index = ctx.int_args_stored() + ctx.float_args_stored() - 1;
        self.other_args[index] = val.to_bits();
    }

    fn return_value(&self) -> usize {
        self.return_value as usize
    }
    fn set_return_value(&mut self, ret_val: usize) {
        self.return_value = ret_val as u32;
    }
}

#[allow(private_interfaces)]
unsafe impl IsaAbi for I686_MSVC_Thiscall {
    const BITNESS: u32 = 32;
    type JmpInterceptorBytesArray = [u8; 7];
    type Args = I686_MSVC_Thiscall_Args;
    type AsmRegister = AsmRegister32;

    fn free_registers() -> &'static [Self::AsmRegister] {
        &[eax, edx]
    }
    fn create_mov_reg_addr(a: &mut CodeAssembler, reg: Self::AsmRegister, addr: usize) -> Result<(), IcedError> {
        a.mov(reg, addr as u32)
    }
    fn create_jmp_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError> {
        a.jmp(reg)
    }
    fn create_call_reg(a: &mut CodeAssembler, reg: Self::AsmRegister) -> Result<(), IcedError> {
        a.call(reg)
    }

    fn create_jmp_to_interceptor(interceptor_addr: usize) -> Self::JmpInterceptorBytesArray {
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();
        Self::create_mov_reg_addr(&mut a, eax, interceptor_addr).unwrap();
        Self::create_jmp_reg(&mut a, eax).unwrap();
        assemble::<Self>(a.instructions(), 0).unwrap().try_into().unwrap()
    }

    unsafe fn create_interceptor<T: 'static>(hook_struct_addr: usize, stack_arg_size: u16) -> Interceptor {
        assert_eq!(size_of::<usize>(), size_of::<u32>());
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();

        #[cfg(target_pointer_width = "32")]
        extern "fastcall" fn abi_fixer<T>(hook: &'static RawHook<I686_MSVC_Thiscall, T>, args_ref: ArgsRef<'_, I686_MSVC_Thiscall>) {
            (hook.hook_fn)(hook, args_ref)
        }
        #[cfg(not(target_pointer_width = "32"))]
        extern "sysv64" fn abi_fixer<T>(_hook: &'static RawHook<I686_MSVC_Thiscall, T>, _args_ref: ArgsRef<'_, I686_MSVC_Thiscall>) {
            unreachable!("I686_MSVC_Thiscall is only supported on 32-bit targets")
        }

        // function prologue with frame pointer
        a.push(ebp).unwrap();
        a.mov(ebp, esp).unwrap();
        // space for the return value
        a.push(eax).unwrap();
        // store all registers
        a.push(ebx).unwrap();
        a.push(ecx).unwrap();
        a.push(esi).unwrap();
        a.push(edi).unwrap();
        a.sub(esp, 0x80).unwrap();
        a.movdqu(ptr(esp + 0x70), xmm7).unwrap();
        a.movdqu(ptr(esp + 0x60), xmm6).unwrap();
        a.movdqu(ptr(esp + 0x50), xmm5).unwrap();
        a.movdqu(ptr(esp + 0x40), xmm4).unwrap();
        a.movdqu(ptr(esp + 0x30), xmm3).unwrap();
        a.movdqu(ptr(esp + 0x20), xmm2).unwrap();
        a.movdqu(ptr(esp + 0x10), xmm1).unwrap();
        a.movdqu(ptr(esp + 0x0), xmm0).unwrap();
        // setup `Hook` and `Args` arguments for `extern "C" abi_fixer`-call
        a.mov(ecx, hook_struct_addr as u32).unwrap();
        a.mov(edx, esp).unwrap();
        // call interceptor
        a.mov(eax, abi_fixer::<T> as u32).unwrap();
        a.call(eax).unwrap();
        // restore callee-saved registers
        // xmm0-7 may or may not be callee-saved according to different sources; just restore them
        a.movdqu(xmm7, ptr(esp + 0x70)).unwrap();
        a.movdqu(xmm6, ptr(esp + 0x60)).unwrap();
        a.movdqu(xmm5, ptr(esp + 0x50)).unwrap();
        a.movdqu(xmm4, ptr(esp + 0x40)).unwrap();
        a.movdqu(xmm3, ptr(esp + 0x30)).unwrap();
        a.movdqu(xmm2, ptr(esp + 0x20)).unwrap();
        a.movdqu(xmm1, ptr(esp + 0x10)).unwrap();
        a.movdqu(xmm0, ptr(esp + 0x0)).unwrap();
        a.add(esp, 0x80).unwrap();
        a.pop(edi).unwrap();
        a.pop(esi).unwrap();
        a.pop(ecx).unwrap(); // technically not needed as it's caller-saved
        a.pop(ebx).unwrap();
        // restore the return value
        a.pop(eax).unwrap();
        // function epilogue
        a.pop(ebp).unwrap();
        if stack_arg_size == 0 {
            a.ret().unwrap();
        } else {
            a.ret_1(stack_arg_size as u32).unwrap();
        }

        Interceptor { instructions: a.take_instructions() }
    }

    fn create_call_trampoline(trampoline_addr: usize, stack_arg_size: u16) -> CallTrampoline {
        assert_eq!(size_of::<usize>(), size_of::<u32>());
        assert!(
            stack_arg_size <= MAX_ARG_BYTES.try_into().unwrap(),
            "only {MAX_ARG_BYTES} stack argument bytes are supported, got {stack_arg_size}",
        );
        let mut a = CodeAssembler::new(Self::BITNESS).unwrap();

        // `call_trampoline` is an `extern "C" fn` -> `extern "cdecl" fn`

        // function prologue
        a.push(ebp).unwrap();
        a.mov(ebp, esp).unwrap();
        // store callee-saved registers
        a.push(ebx).unwrap();
        a.push(esi).unwrap();
        a.push(edi).unwrap();
        a.sub(esp, 0x80).unwrap();
        a.movdqu(ptr(esp + 0x70), xmm7).unwrap();
        a.movdqu(ptr(esp + 0x60), xmm6).unwrap();
        a.movdqu(ptr(esp + 0x50), xmm5).unwrap();
        a.movdqu(ptr(esp + 0x40), xmm4).unwrap();
        a.movdqu(ptr(esp + 0x30), xmm3).unwrap();
        a.movdqu(ptr(esp + 0x20), xmm2).unwrap();
        a.movdqu(ptr(esp + 0x10), xmm1).unwrap();
        a.movdqu(ptr(esp + 0x0), xmm0).unwrap();
        // get args-argument from stack
        a.mov(eax, ptr(ebp + 0x4 + 0x4)).unwrap();
        // copy argument-bytes from Args to this stack
        a.sub(esp, stack_arg_size as u32).unwrap();
        a.mov(edi, esp).unwrap();
        a.lea(esi, ptr(eax + offset_of!(Self::Args, other_args))).unwrap();
        a.mov(ecx, stack_arg_size as u32).unwrap();
        a.rep().movsb().unwrap();
        // restore all registers from Args
        a.movdqu(xmm0, ptr(eax + offset_of!(Self::Args, xmm0))).unwrap();
        a.movdqu(xmm1, ptr(eax + offset_of!(Self::Args, xmm1))).unwrap();
        a.movdqu(xmm2, ptr(eax + offset_of!(Self::Args, xmm2))).unwrap();
        a.movdqu(xmm3, ptr(eax + offset_of!(Self::Args, xmm3))).unwrap();
        a.movdqu(xmm4, ptr(eax + offset_of!(Self::Args, xmm4))).unwrap();
        a.movdqu(xmm5, ptr(eax + offset_of!(Self::Args, xmm5))).unwrap();
        a.movdqu(xmm6, ptr(eax + offset_of!(Self::Args, xmm6))).unwrap();
        a.movdqu(xmm7, ptr(eax + offset_of!(Self::Args, xmm7))).unwrap();
        a.mov(edi, ptr(eax + offset_of!(Self::Args, edi))).unwrap();
        a.mov(esi, ptr(eax + offset_of!(Self::Args, esi))).unwrap();
        a.mov(ecx, ptr(eax + offset_of!(Self::Args, ecx))).unwrap();
        a.mov(ebx, ptr(eax + offset_of!(Self::Args, ebx))).unwrap();
        // call original function
        a.mov(eax, trampoline_addr as u32).unwrap();
        a.call(eax).unwrap();
        // store return value
        a.mov(ebx, ptr(ebp + 0x4 + 0x4)).unwrap();
        a.mov(ptr(ebx + offset_of!(Self::Args, return_value)), ebx).unwrap();
        // restore callee-saved registers
        a.movdqu(xmm7, ptr(esp + 0x70)).unwrap();
        a.movdqu(xmm6, ptr(esp + 0x60)).unwrap();
        a.movdqu(xmm5, ptr(esp + 0x50)).unwrap();
        a.movdqu(xmm4, ptr(esp + 0x40)).unwrap();
        a.movdqu(xmm3, ptr(esp + 0x30)).unwrap();
        a.movdqu(xmm2, ptr(esp + 0x20)).unwrap();
        a.movdqu(xmm1, ptr(esp + 0x10)).unwrap();
        a.movdqu(xmm0, ptr(esp + 0x0)).unwrap();
        a.add(esp, 0x80).unwrap();
        a.pop(edi).unwrap();
        a.pop(esi).unwrap();
        a.pop(ebx).unwrap();
        // function epilogue
        a.pop(ebp).unwrap();
        a.ret().unwrap();

        CallTrampoline { instructions: a.take_instructions() }
    }
}
