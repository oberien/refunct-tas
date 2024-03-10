use std::ffi::c_void;
use std::fmt::Pointer;
use std::cell::Cell;
use std::ops::Deref;
use crate::native::ue::{FName, TArray, UeU64};

mod dynamic_value;
pub use dynamic_value::*;
mod wrappers;
pub use wrappers::*;
mod guobjectarray;
pub use guobjectarray::*;

pub trait UeObjectWrapperType {
    type UeObjectWrapper<'a>: UeObjectWrapper<'a, UeObjectWrapperType = Self>;
}

pub trait DerefToObjectWrapper<'a> {
    fn get_object_wrapper(&self) -> &ObjectWrapper<'a>;
}
impl<'a> DerefToObjectWrapper<'a> for ObjectWrapper<'a> {
    fn get_object_wrapper(&self) -> &ObjectWrapper<'a> {
        self
    }
}
impl<'a, T> DerefToObjectWrapper<'a> for T
where
    T: Deref,
    T::Target: DerefToObjectWrapper<'a>
{
    fn get_object_wrapper(&self) -> &ObjectWrapper<'a> {
        (**self).get_object_wrapper()
    }
}

/// the returned Self type must be bound to lifetime 'a
pub unsafe trait UeObjectWrapper<'a>: Pointer + DerefToObjectWrapper<'a> {
    type UeObjectWrapperType: UeObjectWrapperType<UeObjectWrapper<'a> = Self>;
    type Wrapping;
    const CLASS_NAME: &'static str;

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self;
}
/// the returned Self type must be bound to lifetime 'a
pub unsafe trait SizedArrayElement<'a> {
    type ElementType;

    fn check_property_type(prop: &PropertyWrapper<'a>);
    fn can_be_created_from(prop: &PropertyWrapper<'a>) -> bool;
    unsafe fn create(ptr: *mut Self::ElementType) -> Self;
}
unsafe impl<'a, T: UeObjectWrapper<'a>> SizedArrayElement<'a> for T {
    type ElementType = *mut T::Wrapping;

    fn check_property_type(prop: &PropertyWrapper<'a>) {
        let element_class = prop.upcast::<ObjectPropertyWrapper<'a>>().property_class();
        assert!(element_class.extends_from(T::CLASS_NAME), "{} does not extend from {}", element_class.name(), T::CLASS_NAME);
    }

    fn can_be_created_from(prop: &PropertyWrapper<'a>) -> bool {
        let element_class = prop.upcast::<ObjectPropertyWrapper<'a>>().property_class();
        element_class.extends_from(T::CLASS_NAME)
    }

    unsafe fn create(ptr: *mut Self::ElementType) -> T {
        T::create(*ptr)
    }
}
/// the returned Self type must be bound to lifetime 'a
pub trait ArrayElement<'a> {
    unsafe fn create(ptr: *mut c_void, prop: &PropertyWrapper<'a>) -> Self;
}
impl<'a, T: SizedArrayElement<'a>> ArrayElement<'a> for T {
    unsafe fn create(ptr: *mut c_void, prop: &PropertyWrapper<'a>) -> Self {
        T::check_property_type(prop);
        T::create(ptr as *mut T::ElementType)
    }
}
macro_rules! impl_array_element_for_primitives {
    ($($t:ty, $proptype:literal;)*) => {
        $(
            unsafe impl<'a> SizedArrayElement<'a> for $t {
                type ElementType = $t;

                fn check_property_type(prop: &PropertyWrapper<'a>) {
                    assert_eq!(prop.class().name(), $proptype);
                }

                fn can_be_created_from(prop: &PropertyWrapper<'a>) -> bool {
                    prop.class().name() == $proptype
                }

                unsafe fn create(ptr: *mut Self::ElementType) -> $t {
                    *ptr
                }
            }
            unsafe impl<'a> SizedArrayElement<'a> for &'a Cell<$t> {
                type ElementType = $t;

                fn check_property_type(prop: &PropertyWrapper<'a>) {
                    assert_eq!(prop.class().name(), $proptype);
                }

                fn can_be_created_from(prop: &PropertyWrapper<'a>) -> bool {
                    prop.class().name() == $proptype
                }

                unsafe fn create(ptr: *mut Self::ElementType) -> &'a Cell<$t> {
                    &*(ptr as *const _ as *const _)
                }
            }
        )*
    }
}
impl_array_element_for_primitives! {
    i8, "Int8Property";
    i16, "Int16Property";
    i32, "IntProperty";
    i64, "Int64Property";
    u8, "ByteProperty";
    u16, "UInt16Property";
    u32, "UInt32Property";
    u64, "UInt64Property";
    f32, "FloatProperty";
    f64, "DoubleProperty";
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
pub struct UFunction {
    pub base_ustruct: UStruct,
    pub function_flags: u32,
    pub rep_offset: u16,
    pub num_parms: u8,
    pub parms_size: u16,
    pub return_value_offset: u16,
    pub rpc_id: u16,
    pub rpc_response_id: u16,
    pub first_property_to_init: *mut UProperty,
    pub event_graph_function: *mut UFunction,
    pub event_graph_call_offset: i32,
    pub func: *mut c_void,
}

#[repr(C)]
pub struct UProperty {
    pub base_ufield: UField,
    pub array_dim: i32,
    pub element_size: i32,
    pub property_flags: UeU64,
    pub rep_index: u16,
    pub rep_notify_func: FName,
    pub offset_internal: i32,
    // ELifetimeCondition
    pub blueprint_replication_condition: i32,
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
pub struct _UClassProperty {
    pub base_uproperty: UProperty,
    pub meta_class: *mut UClass,
}
#[repr(C)]
pub struct _UInterfaceProperty {
    pub base_uproperty: UProperty,
    pub interface_class: *mut UClass,
}
#[repr(C)]
pub struct UArrayProperty {
    pub base_uproperty: UProperty,
    pub inner: *mut UProperty,
}
#[repr(C)]
pub struct _UMapProperty {
    pub base_uproperty: UProperty,
    pub key_prop: *mut UProperty,
    pub value_prop: *mut UProperty,
    //pub map_layout: FScriptMapLayout,
}
#[repr(C)]
pub struct _USetProperty {
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
pub struct _UDelegateProperty {
    pub base_uproperty: UProperty,
    //pub signature_function: *mut UFunction,
}
#[repr(C)]
pub struct _UMulticastDelegateProperty {
    pub base_uproperty: UProperty,
    //pub signature_function: *mut UFunction,
}
// {UMulticastInlineDelegateProperty, UMulticastSparseDelegateProperty} : UMulticastDelegateProperty
#[repr(C)]
pub struct _UEnumProperty {
    pub base_uproperty: UProperty,
    // UNumericProperty - Byte, Int*, UInt*
    pub underlying_prop: *mut UProperty,
    //pub enum_: *mut UEnum,
}

