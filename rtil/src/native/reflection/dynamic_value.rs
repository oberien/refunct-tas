use std::ffi::c_void;
use crate::native::{ArrayElement, ObjectPropertyWrapper, ObjectWrapper, SizedArrayElement, UObject};
use crate::native::reflection::{PropertyWrapper};

#[derive(Debug)]
pub struct DynamicValue<'a> {
    /// single indirection when pointing to structs and primitives (*mut f32), double-indirection when pointing to e.g. UObject (*mut *mut UObject)
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

    pub fn unwrap_nullable<T: ArrayElement<'a>>(self) -> Option<T> {
        if self.ptr.is_null() {
            None
        } else {
            Some(self.unwrap())
        }
    }

    pub fn set_object(&self, object: &ObjectWrapper<'a>) {
        ObjectWrapper::check_property_type(&self.prop);
        let object_property = self.prop.upcast::<ObjectPropertyWrapper>();
        if !object.class().extends_from(&object_property.property_class().name()) {
            panic!(
                "trying to set object property of type {} to instance of {} which doesn't extend from the former",
                object.class().name(),
                object_property.property_class().name(),
            );
        }
        let ptr = self.ptr as *mut *mut UObject;
        unsafe { *ptr = object.as_ptr() }
    }
}
