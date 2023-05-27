use std::ffi::c_void;
use std::fmt::{Formatter, Pointer};
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

#[derive(Debug)]
pub struct StructWrapper {
    ptr: *mut u8,
    struct_information: *mut UStruct,
}

impl Pointer for StructWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.ptr, f)
    }
}

impl StructWrapper {
    pub fn iter_properties(&self) -> impl Iterator<Item = PropertyWrapper> {
        // the cast from UStruct to UClass is technically not correct, but iter_properties only uses UStruct fields
        ClassWrapper { class: self.struct_information as *mut UClass }.iter_properties()
    }
    pub fn get_field(&self, name: &str) -> DynamicValue {
        unsafe {
            // the cast from UStruct to UClass is technically not correct, but get_field_info only uses UStruct fields
            let field_info = ClassWrapper { class: self.struct_information as *mut UClass }.get_field_info(name);
            apply_field_info(self.ptr, field_info)
        }
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    offset: isize,
    prop: PropertyWrapper,
}

#[derive(Debug)]
pub struct PropertyWrapper {
    prop: *mut UProperty,
}

impl Pointer for PropertyWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.prop, f)
    }
}

impl PropertyWrapper {
    pub unsafe fn new(prop: *mut UProperty) -> PropertyWrapper {
        PropertyWrapper { prop }
    }
    pub fn as_ptr(&self) -> *mut UProperty {
        self.prop
    }
    pub fn as_object(&self) -> ObjectWrapper {
        ObjectWrapper { object: unsafe { &mut (*self.prop).base_ufield.base_uobject } }
    }
    /// ### Safety:
    /// Only call when you are sure that it's in fact an UObjectProperty
    pub unsafe fn as_uobjectproperty(&self) -> *mut UObjectProperty {
        self.prop as *mut UObjectProperty
    }
    pub fn property_kind(&self) -> String {
        unsafe { (*(*self.prop).base_ufield.base_uobject.class).base_ustruct.base_ufield.base_uobject.name.to_string_lossy() }
    }
    pub fn name(&self) -> String {
        unsafe { (*self.prop).base_ufield.base_uobject.name.to_string_lossy() }
    }
    pub fn offset(&self) -> isize {
        unsafe { (*self.prop).offset_internal as isize }
    }
    pub fn next(&self) -> Option<PropertyWrapper> {
        let prop = unsafe { (*self.prop).base_ufield.next as *mut UProperty };
        if prop.is_null() {
            None
        } else {
            Some(PropertyWrapper { prop })
        }
    }
}

#[derive(Debug)]
pub struct ClassWrapper {
    class: *mut UClass,
}

impl ClassWrapper {
    pub unsafe fn new(class: *mut UClass) -> ClassWrapper {
        ClassWrapper { class }
    }

    pub fn as_object(&self) -> ObjectWrapper {
        ObjectWrapper { object: unsafe { &mut (*self.class).base_ustruct.base_ufield.base_uobject } }
    }

    pub fn children(&self) -> Option<PropertyWrapper> {
        let children = unsafe { (*self.class).base_ustruct.children as *mut UProperty };
        if children.is_null() {
            None
        } else {
            Some(PropertyWrapper { prop: children })
        }
    }

    pub fn iter_properties(&self) -> impl Iterator<Item = PropertyWrapper> {
        struct ClassPropertyIterator {
            next_class: Option<ClassWrapper>,
            next_property: Option<PropertyWrapper>,
        }
        impl Iterator for ClassPropertyIterator {
            type Item = PropertyWrapper;

            fn next(&mut self) -> Option<Self::Item> {
                // we still have properties of this class left
                if let Some(curr) = self.next_property.take() {
                    self.next_property = curr.next();
                    return Some(curr);
                }

                // go deeper into the next class
                if let Some(class) = self.next_class.take() {
                    self.next_property = class.children();
                    self.next_class = class.super_class();
                    self.next()
                } else {
                    None
                }
            }
        }
        ClassPropertyIterator {
            next_class: Some(ClassWrapper { class: self.class }),
            next_property: None,
        }
    }

    pub fn get_field_info(&self, mut name: &str) -> FieldInfo {
        let hacked_absolute = (name == "AbsoluteLocation" || name == "AbsoluteRotation" || name == "AbsoluteScale3D") && self.extends_from("SceneComponent");
        if hacked_absolute {
            match name {
                "AbsoluteLocation" => name = "RelativeLocation",
                "AbsoluteRotation" => name = "RelativeRotation",
                "AbsoluteScale3D" => name = "RelativeScale3D",
                _ => unreachable!(),
            }
        }
        let prop = self.iter_properties()
            .find(|prop| prop.name() == name)
            .unwrap();
        let offset = if hacked_absolute {
            match name {
                "RelativeLocation" => {
                    #[cfg(unix)] { 0x1a0 }
                    #[cfg(windows)] { 0x140 }
                },
                "RelativeRotation" => {
                    #[cfg(unix)] { 0x1ac }
                    #[cfg(windows)] { 0x14c }
                },
                "RelativeScale3D" => {
                    #[cfg(unix)] { 0x1b8 }
                    #[cfg(windows)] { 0x158 }
                },
                _ => unreachable!(),
            }
        } else {
            prop.offset()
        };
        FieldInfo { offset, prop }
    }

    pub fn super_class(&self) -> Option<ClassWrapper> {
        let super_class = unsafe { (*self.class).base_ustruct.super_struct as *mut UClass };
        if super_class.is_null() {
            None
        } else {
            Some(ClassWrapper { class: super_class })
        }
    }

    pub fn extends_from(&self, name: &str) -> bool {
        let mut class = Some(ClassWrapper { class: self.class });
        while let Some(c) = class {
            let class_name = c.name();
            if name == class_name {
                return true
            }
            class = c.super_class();
        }
        false
    }


    pub fn name(&self) -> String {
        unsafe { (*self.class).base_ustruct.base_ufield.base_uobject.name.to_string_lossy() }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ObjectWrapper {
    object: *mut UObject,
}

impl Pointer for ObjectWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.object, f)
    }
}

impl ObjectWrapper {
    pub unsafe fn new(object: *mut UObject) -> ObjectWrapper {
        ObjectWrapper { object }
    }

    pub fn name(&self) -> String {
        unsafe { (*self.object).name.to_string_lossy() }
    }

    pub fn class(&self) -> ClassWrapper {
        ClassWrapper { class: unsafe { (*self.object).class } }
    }

    pub fn vtable(&self) -> *const () {
        unsafe { (*self.object).vtable }
    }

    pub fn get_field(&self, name: &str) -> DynamicValue {
        unsafe {
            let field_info = self.class().get_field_info(name);
            apply_field_info(self.object as *mut u8, field_info)
        }
    }
}

unsafe fn apply_field_info(ptr: *mut u8, info: FieldInfo) -> DynamicValue {
    let value_ptr = ptr.offset(info.offset);
    let property_kind = info.prop.property_kind();

    fn checked_cast<T>(ptr: *mut u8) -> *mut T {
        assert_eq!(ptr as usize % std::mem::align_of::<T>(), 0, "alignment of pointer cast from *mut u8 to *mut {} doesn't fit", std::any::type_name::<T>());
        ptr as _
    }

    match property_kind.as_str() {
        "Int8Property" => DynamicValue::Int8(checked_cast::<i8>(value_ptr)),
        "Int16Property" => DynamicValue::Int16(checked_cast::<i16>(value_ptr)),
        "IntProperty" => DynamicValue::Int(checked_cast::<i32>(value_ptr)),
        "Int64Property" => DynamicValue::Int64(checked_cast::<i64>(value_ptr)),
        "ByteProperty" => DynamicValue::Byte(checked_cast::<u8>(value_ptr)),
        "UInt16Property" => DynamicValue::UInt16(checked_cast::<u16>(value_ptr)),
        "UInt32Property" => DynamicValue::UInt32(checked_cast::<u32>(value_ptr)),
        "UInt64Property" => DynamicValue::UInt64(checked_cast::<u64>(value_ptr)),
        "FloatProperty" => DynamicValue::Float(checked_cast::<f32>(value_ptr)),
        "DoubleProperty" => DynamicValue::Double(checked_cast::<f64>(value_ptr)),
        "BoolProperty" => todo!("BoolProperty"),
        "ObjectProperty" | "WeakObjectProperty" | "LazyObjectProperty" | "SoftObjectProperty" => {
            DynamicValue::Object(ObjectWrapper { object: *checked_cast::<*mut UObject>(value_ptr) })
        },
        "ClassProperty" | "SoftClassProperty" => todo!("ClassProperty"),
        "InterfaceProperty" => todo!("InterfaceProperty"),
        "NameProperty" => DynamicValue::Name(*checked_cast::<FName>(value_ptr)),
        "StrProperty" => DynamicValue::Str(checked_cast::<FString>(value_ptr)),
        "ArrayProperty" => DynamicValue::Array(checked_cast::<TArray<*mut ()>>(value_ptr)),
        "MapProperty" => todo!("MapProperty"),
        "SetProperty" => todo!("SetProperty"),
        "StructProperty" => DynamicValue::Struct(StructWrapper {
            ptr: value_ptr,
            struct_information: (*(info.prop.as_ptr() as *mut UStructProperty)).struct_,
        }),
        "DelegateProperty" | "MulticastDelegateProperty" | "MulticastInlineDelegateProperty" | "MulticastSparseDelegateProperty" => todo!("Function-based Properties"),
        "EnumProperty" => todo!("EnumProperty"),
        "TextProperty" => todo!("TextProperty"),
        _ => unreachable!("Got unknown UE property kind {property_kind}"),
    }
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

// root stuff to mark texture as non-GC-able
#[repr(C)]
pub struct FUObjectArray {
    pub obj_first_gc_index: i32,
    pub obj_last_non_gc_index: i32,
    pub max_objects_not_considered_by_gc: i32,
    pub open_for_disregard_for_gc: bool,
    pub obj_objects: TUObjectArray,
    // ...
}

// typedef'd from FFixedUObjectArray
#[repr(C)]
pub struct TUObjectArray {
    pub objects: *mut FUObjectItem,
    pub max_elements: i32,
    pub num_elements: i32,
}

#[repr(C)]
pub struct FUObjectItem {
    pub object: *mut c_void,
    pub flags: i32,
    pub cluster_index: i32,
    pub serial_number: i32,
}

#[allow(unused)]
#[repr(i32)]
pub enum EInternalObjectFlags {
    None = 0,
    ReachableInCluster = 1 << 23,
    ClusterRoot = 1 << 24,
    Native = 1 << 25,
    Async = 1 << 26,
    AsyncLoading = 1 << 27,
    Unreachable = 1 << 28,
    PendingKill = 1 << 29,
    RootSet = 1 << 30,
    PendingConstruction = 1 << 31,
}
