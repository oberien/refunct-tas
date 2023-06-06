use std::ffi::c_void;
use crate::native::{ArrayElement};
use crate::native::reflection::{PropertyWrapper};

#[derive(Debug)]
pub struct DynamicValue<'a> {
    ptr: *mut c_void,
    prop: PropertyWrapper<'a>,
}

impl<'a> DynamicValue<'a> {
    pub unsafe fn new(ptr: *mut c_void, prop: PropertyWrapper<'a>) -> DynamicValue<'a> {
        DynamicValue { ptr, prop }
    }

    pub fn prop(&self) -> PropertyWrapper<'a> {
        self.prop.clone()
    }

    pub fn unwrap<T: ArrayElement<'a>>(self) -> T {
        unsafe { T::create(self.ptr, &self.prop) }
    }
}
