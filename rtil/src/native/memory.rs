use std::mem;

#[cfg(unix)] use libc::c_void;
#[cfg(windows)] use winapi::ctypes::c_void;

use native::{FMEMORY_FREE, FMEMORY_MALLOC};

pub struct FMemory;

impl FMemory {
    pub unsafe fn malloc(size: usize) -> *mut c_void {
        let fun: extern_fn!(fn(count: usize, alignment: u32) -> *mut c_void)
            = mem::transmute(FMEMORY_MALLOC);
        fun(size, 0)
    }

    pub unsafe fn free(ptr: *mut c_void) {
        let fun: extern_fn!(fn(original: *mut c_void))
            = mem::transmute(FMEMORY_FREE);
        fun(ptr)
    }
}