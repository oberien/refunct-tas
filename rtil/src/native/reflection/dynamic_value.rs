use std::ffi::c_void;
use crate::native::reflection::{ClassWrapper, ObjectWrapper, ObjectStructFieldWrapper, ArrayWrapper, PropertyWrapper, ArrayElement};
use crate::native::ue::{FName, FString, TArray};

#[derive(Debug)]
pub enum DynamicValue<'a> {
    Int8(&'a mut i8),
    Int16(&'a mut i16),
    Int(&'a mut i32),
    Int64(&'a mut i64),
    Byte(&'a mut u8),
    UInt16(&'a mut u16),
    UInt32(&'a mut u32),
    UInt64(&'a mut u64),
    Float(&'a mut f32),
    Double(&'a mut f64),
    // Bool
    Object(ObjectWrapper<'a>),
    Class(ClassWrapper<'a>),
    Interface(ClassWrapper<'a>),
    Name(FName),
    Str(*mut FString),
    Array(*mut c_void, PropertyWrapper<'a>),
    // Map
    // Set
    Struct(ObjectStructFieldWrapper<'a>),
    // Function
    // Enum
    // Text
}
impl<'a> DynamicValue<'a> {
    pub fn unwrap_int8(self) -> &'a mut i8 {
        match self {
            DynamicValue::Int8(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_int16(self) -> &'a mut i16 {
        match self {
            DynamicValue::Int16(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_int(self) -> &'a mut i32 {
        match self {
            DynamicValue::Int(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_int64(self) -> &'a mut i64 {
        match self {
            DynamicValue::Int64(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_byte(self) -> &'a mut u8 {
        match self {
            DynamicValue::Byte(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_uint16(self) -> &'a mut u16 {
        match self {
            DynamicValue::UInt16(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_uint32(self) -> &'a mut u32 {
        match self {
            DynamicValue::UInt32(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_uint64(self) -> &'a mut u64 {
        match self {
            DynamicValue::UInt64(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_float(self) -> &'a mut f32 {
        match self {
            DynamicValue::Float(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_double(self) -> &'a mut f64 {
        match self {
            DynamicValue::Double(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_object(self) -> ObjectWrapper<'a> {
        match self {
            DynamicValue::Object(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_class(self) -> ClassWrapper<'a> {
        match self {
            DynamicValue::Class(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_interface(self) -> ClassWrapper<'a> {
        match self {
            DynamicValue::Interface(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_name(self) -> FName {
        match self {
            DynamicValue::Name(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_str(self) -> *mut FString {
        match self {
            DynamicValue::Str(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_array<T: ArrayElement>(self) -> ArrayWrapper<'a, T> {
        let (ptr, inner_prop) = match self {
            DynamicValue::Array(val, inner_prop) => (val, inner_prop),
            _ => panic!("tried to unwrap an incompatible value"),
        };
        T::check_property_type(inner_prop);
        let array = ptr as *mut TArray<T::ElementType>;
        unsafe { ArrayWrapper::new(array) }
    }
    pub fn unwrap_struct(self) -> ObjectStructFieldWrapper<'a> {
        match self {
            DynamicValue::Struct(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
}


