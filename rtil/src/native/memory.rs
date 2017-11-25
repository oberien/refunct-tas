use std::mem;

use native::FMEMORY_MALLOC;
use native::FMEMORY_FREE;

pub struct FMemory;

impl FMemory {
    pub unsafe fn malloc(size: usize) -> *mut () {
        let fun: extern "C" fn(usize, u32) -> *mut ()
            = mem::transmute(FMEMORY_MALLOC);
        fun(size, 0)
    }

    pub unsafe fn free(ptr: *mut ()) {
        let fun: extern "C" fn(*mut ())
            = mem::transmute(FMEMORY_FREE);
        fun(ptr)
    }
}