use std::{mem, slice};
use std::ffi::c_void;
use crate::{ArgsRef, get_orig_bytes, IsaAbi, trampoline};
use crate::function_decoder::FunctionDecoder;
use crate::hook_memory_page::HookMemoryPageBuilder;
use crate::isa_abi::Array;

#[repr(C)]
pub struct RawHook<IA: IsaAbi, T: 'static> {
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
    pub(crate) hook_fn: for<'a> fn(&'static RawHook<IA, T>, ArgsRef<'a, IA>),
    /// original bytes of the original function that are overwritten when enabling the hook
    orig_bytes: IA::JmpInterceptorBytesArray,
    /// argument-bytes passed to the original function via the stack
    orig_stack_arg_size: u16,
    user_context: T,
}

impl<IA: IsaAbi> RawHook<IA, ()> {
    #[must_use]
    pub unsafe fn create(orig_addr: usize, hook_fn: for<'a> fn(&'static RawHook<IA, ()>, ArgsRef<'a, IA>)) -> &'static RawHook<IA, ()> {
        unsafe { Self::with_context(orig_addr, hook_fn, ()) }
    }
}
impl<IA: IsaAbi, T> RawHook<IA, T> {
    #[must_use]
    pub unsafe fn with_context(orig_addr: usize, hook_fn: for<'a> fn(&'static RawHook<IA, T>, ArgsRef<'a, IA>), user_context: T) -> &'static RawHook<IA, T> {
        let orig_stack_arg_size = unsafe { FunctionDecoder::<IA>::new(orig_addr) }.stack_argument_size();

        let builder = HookMemoryPageBuilder::<IA, T>::new();

        let trampoline = unsafe { trampoline::create_trampoline::<IA>(orig_addr) };
        let builder = builder.trampoline(trampoline);

        let interceptor = unsafe { IA::create_interceptor::<T>(builder.hook_struct_addr(), orig_stack_arg_size) };
        let builder = builder.interceptor(interceptor);

        let call_trampoline = IA::create_call_trampoline(builder.trampoline_addr(), orig_stack_arg_size);
        let builder = builder.call_trampoline(call_trampoline);

        let orig_bytes = unsafe { get_orig_bytes::<IA>(orig_addr) };
        let hook = RawHook {
            orig_addr,
            trampoline_addr: builder.trampoline_addr(),
            interceptor_addr: builder.interceptor_addr(),
            call_trampoline_addr: builder.call_trampoline_addr(),
            hook_fn,
            orig_bytes,
            orig_stack_arg_size,
            user_context,
        };
        builder.finalize(hook)
    }

    pub fn enable(&self) {
        let jmp = IA::create_jmp_to_interceptor(self.interceptor_addr);
        unsafe { make_rw(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
        let slice = unsafe { slice::from_raw_parts_mut(self.orig_addr as *mut u8, IA::JmpInterceptorBytesArray::LEN) };
        jmp.store_to(slice);
        unsafe { make_rx(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
    }
    pub fn enabled(&self) -> &Self {
        self.enable();
        self
    }
    pub fn disable(&self) {
        let slice = unsafe { slice::from_raw_parts_mut(self.orig_addr as *mut u8, IA::JmpInterceptorBytesArray::LEN) };
        unsafe { make_rw(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
        slice.copy_from_slice(self.orig_bytes.as_slice());
        unsafe { make_rx(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
    }
    pub unsafe fn call_original_function(&self, args: impl AsRef<IA::Args>) {
        unsafe {
            let function: extern "C" fn(&IA::Args) = mem::transmute(self.call_trampoline_addr);
            function(args.as_ref())
        }
    }
    pub fn trampoline(&self) -> *const () {
        self.call_trampoline_addr as *const ()
    }
    pub fn context(&self) -> &T {
        &self.user_context
    }
}

/// SAFETY: implementation must be correct for the OS
/// # Safety
/// * code on the memory pages containing the requested bytes must not be executed while this function is running
unsafe fn make_rw(addr: usize, len: usize) {
    let start_page = addr & !0xfff;
    let end_page = (addr + len) & !0xfff;
    let len = end_page - start_page + 0x1000;
    let page = start_page as *mut c_void;
    #[cfg(windows)] {
        let mut out = 0;
        unsafe { winapi::um::memoryapi::VirtualProtect(page, len, winapi::um::winnt::PAGE_READWRITE, &mut out); }
    }
    #[cfg(unix)] {
        unsafe { libc::mprotect(page, len, libc::PROT_READ | libc::PROT_WRITE); }
    }
}
/// SAFETY: implementation must be correct for the OS
/// # Safety
/// * the memory pages containing the requested bytes must not be written to while this function is running
unsafe fn make_rx(addr: usize, len: usize) {
    let start_page = addr & !0xfff;
    let end_page = (addr + len) & !0xfff;
    let len = end_page - start_page + 0x1000;
    let page = start_page as *mut c_void;
    #[cfg(windows)] {
        let mut out = 0;
        unsafe { winapi::um::memoryapi::VirtualProtect(page, len, winapi::um::winnt::PAGE_EXECUTE_READ, &mut out); }
    }
    #[cfg(unix)] {
        unsafe { libc::mprotect(page, len, libc::PROT_READ | libc::PROT_EXEC); }
    }
}
