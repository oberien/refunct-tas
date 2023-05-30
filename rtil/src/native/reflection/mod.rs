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
pub unsafe trait ArrayElement {
    type ElementType;

    fn check_property_type(prop: PropertyWrapper);
    unsafe fn create(ptr: *mut Self::ElementType) -> Self;
}
unsafe impl<T: UeObjectWrapper> ArrayElement for T {
    type ElementType = *mut T::Wrapping;

    fn check_property_type(prop: PropertyWrapper) {
        let element_class = unsafe { ClassWrapper::new((*prop.as_uobjectproperty()).property_class) };
        assert!(element_class.extends_from(T::CLASS_NAME), "{} does not extend from {}", element_class.name(), T::CLASS_NAME);
    }

    unsafe fn create(ptr: *mut Self::ElementType) -> Self {
        T::create(*ptr)
    }
}
macro_rules! impl_array_element_for_primitives {
    ($($t:ty, $proptype:literal;)*) => {
        $(
            unsafe impl<'a> ArrayElement for &'a mut $t {
                type ElementType = $t;

                fn check_property_type(prop: PropertyWrapper) {
                    assert_eq!(prop.as_object().class().name(), $proptype);
                }

                unsafe fn create(ptr: *mut Self::ElementType) -> Self {
                    &mut *ptr
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
pub struct _UBoolProperty {
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

