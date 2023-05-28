use crate::native::reflection::{ClassWrapper, ObjectWrapper, StructWrapper};
use crate::native::ue::{FName, FString, TArray};

#[derive(Debug)]
pub enum DynamicValue {
    Int8(*mut i8),
    Int16(*mut i16),
    Int(*mut i32),
    Int64(*mut i64),
    Byte(*mut u8),
    UInt16(*mut u16),
    UInt32(*mut u32),
    UInt64(*mut u64),
    Float(*mut f32),
    Double(*mut f64),
    Bool(bool),
    Object(ObjectWrapper),
    Class(ClassWrapper),
    Interface(ClassWrapper),
    Name(FName),
    Str(*mut FString),
    Array(*mut TArray<*mut ()>),
    // Map
    // Set
    Struct(StructWrapper),
    // Function
    // Enum
    // Text
}
impl DynamicValue {
    pub fn unwrap_int8(self) -> *mut i8 {
        match self {
            DynamicValue::Int8(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_int16(self) -> *mut i16 {
        match self {
            DynamicValue::Int16(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_int(self) -> *mut i32 {
        match self {
            DynamicValue::Int(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_int64(self) -> *mut i64 {
        match self {
            DynamicValue::Int64(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_byte(self) -> *mut u8 {
        match self {
            DynamicValue::Byte(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_uint16(self) -> *mut u16 {
        match self {
            DynamicValue::UInt16(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_uint32(self) -> *mut u32 {
        match self {
            DynamicValue::UInt32(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_uint64(self) -> *mut u64 {
        match self {
            DynamicValue::UInt64(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_float(self) -> *mut f32 {
        match self {
            DynamicValue::Float(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_double(self) -> *mut f64 {
        match self {
            DynamicValue::Double(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_bool(self) -> bool {
        match self {
            DynamicValue::Bool(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_object(self) -> ObjectWrapper {
        match self {
            DynamicValue::Object(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_class(self) -> ClassWrapper {
        match self {
            DynamicValue::Class(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_interface(self) -> ClassWrapper {
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
    pub fn unwrap_array(self) -> *mut TArray<*mut ()> {
        match self {
            DynamicValue::Array(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
    pub fn unwrap_struct(self) -> StructWrapper {
        match self {
            DynamicValue::Struct(val) => val,
            _ => panic!("tried to unwrap an incompatible value"),
        }
    }
}


