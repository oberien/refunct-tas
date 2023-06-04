use std::ffi::c_void;
use crate::native::{ArrayElement, BoolInstanceWrapper};
use crate::native::reflection::{ObjectWrapper, StructInstanceWrapper, ArrayWrapper, PropertyWrapper};
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
    Bool(BoolInstanceWrapper<'a>),
    Object(Option<ObjectWrapper<'a>>),
    // Class
    // Interface,
    Name(FName),
    Str(*mut FString),
    Array(*mut c_void, PropertyWrapper<'a>),
    // Map
    // Set
    Struct(StructInstanceWrapper<'a>),
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
    pub fn unwrap_bool(self) -> BoolInstanceWrapper<'a> {
        match self {
            DynamicValue::Bool(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_object(self) -> ObjectWrapper<'a> {
        match self {
            DynamicValue::Object(val) => val.unwrap(),
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_nullable_object(self) -> Option<ObjectWrapper<'a>> {
        match self {
            DynamicValue::Object(val) => val,
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
        unsafe { ArrayWrapper::new(ptr as *mut TArray<c_void>, inner_prop) }
    }
    pub fn unwrap_struct(self) -> StructInstanceWrapper<'a> {
        match self {
            DynamicValue::Struct(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
}


