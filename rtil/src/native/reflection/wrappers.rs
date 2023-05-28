use std::fmt::{Formatter, Pointer};
use std::ptr;
use crate::native::reflection::{DynamicValue, UClass, UObject, UObjectProperty, UProperty, UStruct, UStructProperty};
use crate::native::ue::{FName, FString, TArray};

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
struct FieldInfo {
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
        unsafe { ObjectWrapper::new(ptr::addr_of_mut!((*self.prop).base_ufield.base_uobject)) }
    }
    /// ### Safety:
    /// Only call when you are sure that it's in fact an UObjectProperty
    pub unsafe fn as_uobjectproperty(&self) -> *mut UObjectProperty {
        self.prop as *mut UObjectProperty
    }
    pub fn property_kind(&self) -> String {
        self.as_object().class().name()
    }
    pub fn name(&self) -> String {
        self.as_object().name()
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
        unsafe { ObjectWrapper::new(ptr::addr_of_mut!((*self.class).base_ustruct.base_ufield.base_uobject)) }
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

    fn get_field_info(&self, mut name: &str) -> FieldInfo {
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
        self.as_object().name()
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
    pub fn as_ptr(&self) -> *mut UObject {
        self.object
    }

    pub fn vtable(&self) -> *const () {
        unsafe { (*self.object).vtable }
    }
    pub fn internal_index(&self) -> usize {
        unsafe { (*self.object).internal_index.try_into().unwrap() }
    }

    pub fn name(&self) -> String {
        unsafe { (*self.object).name.to_string_lossy() }
    }

    pub fn class(&self) -> ClassWrapper {
        ClassWrapper { class: unsafe { (*self.object).class } }
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

