pub struct FMemory;

impl FMemory {
    pub unsafe fn malloc(size: usize) -> *mut () {
        let fun: extern "C" fn(usize, u32) -> *mut ()
            = ::std::mem::transmute(::native::linux::FMEMORY_MALLOC);
        fun(size, 0)
    }

    pub unsafe fn free(ptr: *mut ()) {
        let fun: extern "C" fn(*mut ())
            = ::std::mem::transmute(::native::linux::FMEMORY_FREE);
        fun(ptr)
    }
}