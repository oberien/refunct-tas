use crate::native::ue::{FName, TArray};

mod dynamic_value;
pub use dynamic_value::*;
mod wrappers;
pub use wrappers::*;
mod guobjectarray;
pub use guobjectarray::*;

pub unsafe trait UeObjectWrapper {
    type Wrapping;
    const CLASS_NAME: &'static str;

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self;
}

#[repr(C)]
pub struct UObject {
    // technically its UObject : UObjectBaseUtility : UObjectBase
    // UObjectBase is the only one of those three actually containing fields
    // the others just add functions
    // simplified into a single struct
    pub vtable: *const (),
    pub _object_flags: i32,
    pub internal_index: i32,
    pub class: *mut UClass,
    pub name: FName,
    pub _outer_private: *const (),
}
#[repr(C)]
pub struct AActor {
    pub base_uobject: UObject,
    // ...
}

#[repr(C)]
pub struct UField {
    pub base_uobject: UObject,
    pub next: *mut UField,
}

#[repr(C)]
pub struct UStruct {
    pub base_ufield: UField,
    pub super_struct: *mut UStruct,
    pub children: *mut UField,
    pub properties_size: i32,
    pub min_alignment: i32,
    pub script: TArray<u8>,
    pub property_link: *mut UProperty,
    pub ref_link: *mut UProperty,
    pub destructor_link: *mut UProperty,
    pub post_construct_link: *mut UProperty,
    pub script_object_references: TArray<*mut UObject>,
}

#[repr(C)]
pub struct UClass {
    pub base_ustruct: UStruct,
    // ...
}

#[repr(C)]
pub struct UProperty {
    pub base_ufield: UField,
    pub array_dim: i32,
    pub element_size: i32,
    pub property_flags: u64,
    pub rep_index: u16,
    pub rep_notify_func: FName,
    pub offset_internal: i32,
    pub property_link_next: *mut UProperty,
    pub next_ref: *mut UProperty,
    pub destructor_link_next: *mut UProperty,
    pub post_construct_link_next: *mut UProperty,
}

// Byte, Int*, UInt* are all empty
#[repr(C)]
pub struct UBoolProperty {
    base_uproperty: UProperty,
    field_size: u8,
    byte_offset: u8,
    byte_mask: u8,
    field_mask: u8,
}
#[repr(C)]
pub struct UObjectProperty {
    // UObjectProperty : UObjectPropertyBase : UProperty
    // inlined UObjectPropertyBase here
    pub base_uproperty: UProperty,
    pub property_class: *mut UClass,
}
#[repr(C)]
pub struct UClassProperty {
    pub base_uproperty: UProperty,
    pub meta_class: *mut UClass,
}
#[repr(C)]
pub struct UInterfaceProperty {
    pub base_uproperty: UProperty,
    pub interface_class: *mut UClass,
}
#[repr(C)]
pub struct UArrayProperty {
    pub base_uproperty: UProperty,
    pub inner: *mut UProperty,
}
#[repr(C)]
pub struct UMapProperty {
    pub base_uproperty: UProperty,
    pub key_prop: *mut UProperty,
    pub value_prop: *mut UProperty,
    //pub map_layout: FScriptMapLayout,
}
#[repr(C)]
pub struct USetProperty {
    pub base_uproperty: UProperty,
    pub element_prop: *mut UProperty,
    //pub set_layout: FScriptSetLayout,
}
#[repr(C)]
pub struct UStructProperty {
    pub base_uproperty: UProperty,
    // technically UScriptStruct, but who cares
    pub struct_: *mut UStruct,
}
#[repr(C)]
pub struct UDelegateProperty {
    pub base_uproperty: UProperty,
    //pub signature_function: *mut UFunction,
}
#[repr(C)]
pub struct UMulticastDelegateProperty {
    pub base_uproperty: UProperty,
    //pub signature_function: *mut UFunction,
}
// {UMulticastInlineDelegateProperty, UMulticastSparseDelegateProperty} : UMulticastDelegateProperty
#[repr(C)]
pub struct UEnumProperty {
    pub base_uproperty: UProperty,
    // UNumericProperty - Byte, Int*, UInt*
    pub underlying_prop: *mut UProperty,
    //pub enum_: *mut UEnum,
}

