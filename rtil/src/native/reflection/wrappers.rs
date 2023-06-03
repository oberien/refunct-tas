use std::ffi::c_void;
use std::fmt::{Display, Formatter, Pointer};
use std::marker::PhantomData;
use std::ops::Deref;
use crate::native::reflection::{AActor, ArrayElement, DynamicValue, UArrayProperty, UClass, UeObjectWrapper, UObject, UObjectProperty, UProperty, UStruct, UStructProperty};
use crate::native::ue::{FName, FString, TArray};
use crate::native::UField;

#[derive(Debug, Clone)]
pub struct ObjectStructFieldWrapper<'a> {
    ptr: *mut u8,
    struct_information: StructWrapper<'a>,
    _marker: PhantomData<&'a mut UObject>,
}

impl<'a> Pointer for ObjectStructFieldWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.ptr, f)
    }
}

impl<'a> ObjectStructFieldWrapper<'a> {
    pub unsafe fn new(ptr: *mut u8, struct_information: StructWrapper<'a>) -> ObjectStructFieldWrapper<'a> {
        ObjectStructFieldWrapper { ptr, struct_information, _marker: PhantomData }
    }
    pub fn get_field(&self, name: &str) -> DynamicValue {
        unsafe {
            let field_info = self.struct_information.get_field_info(name);
            apply_field_info(self.ptr, field_info)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldInfo<'a> {
    offset: isize,
    prop: PropertyWrapper<'a>,
}
#[derive(Debug, Clone)]
pub struct ObjectWrapper<'a> {
    object: *mut UObject,
    _marker: PhantomData<&'a mut UObject>,
}
unsafe impl<'a> UeObjectWrapper for ObjectWrapper<'a> {
    type Wrapping = UObject;
    const CLASS_NAME: &'static str = "Object";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        ObjectWrapper::new(ptr)
    }
}
impl<'a> Pointer for ObjectWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.object, f)
    }
}
impl<'a> ObjectWrapper<'a> {
    pub unsafe fn new(object: *mut UObject) -> ObjectWrapper<'a> {
        ObjectWrapper { object, _marker: PhantomData }
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
    pub fn class(&self) -> ClassWrapper<'a> {
        unsafe { ClassWrapper::new((*self.object).class) }
    }

    pub fn get_field(&self, name: &str) -> DynamicValue<'a> {
        unsafe {
            let field_info = self.class().get_field_info(name);
            apply_field_info(self.object as *mut u8, field_info)
        }
    }

    pub fn upcast<T: UeObjectWrapper>(&self) -> T {
        self.try_upcast().unwrap_or_else(|| panic!("can't upcast to {}", T::CLASS_NAME))
    }
    pub fn try_upcast<T: UeObjectWrapper>(&self) -> Option<T> {
        if self.class().extends_from(T::CLASS_NAME) {
            unsafe { Some(T::create(self.as_ptr() as *mut T::Wrapping)) }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldWrapper<'a> {
    base: ObjectWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper for FieldWrapper<'a> {
    type Wrapping = UField;
    const CLASS_NAME: &'static str = "Field";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        FieldWrapper::new(ptr)
    }
}
impl<'a> Deref for FieldWrapper<'a> {
    type Target = ObjectWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for FieldWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> FieldWrapper<'a> {
    pub unsafe fn new(field: *mut UField) -> FieldWrapper<'a> {
        FieldWrapper { base: ObjectWrapper::new(field as *mut UObject)}
    }
    pub fn as_ptr(&self) -> *mut UField {
        self.base.as_ptr() as *mut UField
    }
    pub fn next_field(&self) -> Option<FieldWrapper<'a>> {
        let field = unsafe { (*self.as_ptr()).next };
        if field.is_null() {
            None
        } else {
            unsafe { Some(FieldWrapper::new(field)) }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropertyWrapper<'a> {
    base: FieldWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper for PropertyWrapper<'a> {
    type Wrapping = UProperty;
    const CLASS_NAME: &'static str = "Property";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        PropertyWrapper::new(ptr)
    }
}
impl<'a> Deref for PropertyWrapper<'a> {
    type Target = FieldWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for PropertyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> PropertyWrapper<'a> {
    pub unsafe fn new(prop: *mut UProperty) -> PropertyWrapper<'a> {
        PropertyWrapper { base: FieldWrapper::new(prop as *mut UField) }
    }
    pub fn as_ptr(&self) -> *mut UProperty {
        self.base.as_ptr() as *mut UProperty
    }
    pub fn as_uobjectproperty(&self) -> *mut UObjectProperty {
        assert_eq!(self.property_kind(), "ObjectProperty");
        self.as_ptr() as *mut UObjectProperty
    }
    pub fn as_uarrayproperty(&self) -> *mut UArrayProperty {
        assert_eq!(self.property_kind(), "ArrayProperty");
        self.as_ptr() as *mut UArrayProperty
    }
    pub fn property_kind(&self) -> String {
        self.class().name()
    }
    pub fn offset(&self) -> isize {
        unsafe { (*self.as_ptr()).offset_internal as isize }
    }
}
impl<'a> Display for PropertyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({:p}) (offset {:#x})", self.class().name(), self.name(), self.as_ptr(), self.offset())
    }
}

#[derive(Debug, Clone)]
pub struct StructWrapper<'a> {
    base: FieldWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper for StructWrapper<'a> {
    type Wrapping = UStruct;
    const CLASS_NAME: &'static str = "Struct";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        StructWrapper::new(ptr)
    }
}
impl<'a> Deref for StructWrapper<'a> {
    type Target = FieldWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for StructWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> StructWrapper<'a> {
    pub unsafe fn new(struct_: *mut UStruct) -> StructWrapper<'a> {
        StructWrapper { base: FieldWrapper::new(struct_ as *mut UField) }
    }
    pub fn as_ptr(&self) -> *mut UStruct {
        self.base.as_ptr() as *mut UStruct
    }

    pub fn children(&self) -> Option<FieldWrapper<'a>> {
        let children = unsafe { (*self.as_ptr()).children };
        if children.is_null() {
            None
        } else {
            unsafe { Some(FieldWrapper::new(children)) }
        }
    }

    pub fn iter_fields(&self) -> impl Iterator<Item = FieldWrapper<'a>> {
        struct StructFieldIterator<'a> {
            next_struct: Option<StructWrapper<'a>>,
            next_field: Option<FieldWrapper<'a>>,
        }
        impl<'a> Iterator for StructFieldIterator<'a> {
            type Item = FieldWrapper<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                // we still have properties of this class left
                if let Some(curr) = self.next_field.take() {
                    self.next_field = curr.next_field();
                    return Some(curr);
                }

                // go deeper into the next class
                if let Some(class) = self.next_struct.take() {
                    self.next_field = class.children();
                    self.next_struct = class.super_struct();
                    self.next()
                } else {
                    None
                }
            }
        }
        StructFieldIterator {
            next_struct: Some(self.clone()),
            next_field: None,
        }
    }
    pub fn iter_properties(&self) -> impl Iterator<Item = PropertyWrapper<'a>> {
        self.iter_fields()
            .filter_map(|field| field.try_upcast::<PropertyWrapper>())
    }

    fn get_field_info(&self, mut name: &str) -> FieldInfo<'a> {
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
            .unwrap_or_else(|| panic!("cannot access property {name} of type {}", self.class().name()))
            .upcast::<PropertyWrapper>();
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

    pub fn super_struct(&self) -> Option<StructWrapper<'a>> {
        let super_class = unsafe { (*self.as_ptr()).super_struct };
        if super_class.is_null() {
            None
        } else {
            unsafe { Some(StructWrapper::new(super_class)) }
        }
    }

    pub fn extends_from(&self, name: &str) -> bool {
        let mut class = Some(self.clone());
        while let Some(c) = class {
            let class_name = c.name();
            if name == class_name {
                return true
            }
            class = c.super_struct();
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct ClassWrapper<'a> {
    base: StructWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper for ClassWrapper<'a> {
    type Wrapping = UClass;
    const CLASS_NAME: &'static str = "Class";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        ClassWrapper::new(ptr)
    }
}
impl<'a> Deref for ClassWrapper<'a> {
    type Target = StructWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for ClassWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> ClassWrapper<'a> {
    pub unsafe fn new(class: *mut UClass) -> ClassWrapper<'a> {
        ClassWrapper { base: StructWrapper::new(class as *mut UStruct) }
    }
    pub fn as_ptr(&self) -> *mut UClass {
        self.base.as_ptr() as *mut UClass
    }
    pub fn super_class(&self) -> Option<ClassWrapper<'a>> {
        self.super_struct().map(|p| p.upcast())
    }
}

#[derive(Debug, Clone)]
pub struct ActorWrapper<'a> {
    base: ObjectWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper for ActorWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "Actor";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        ActorWrapper::new(ptr)
    }
}
impl<'a> Deref for ActorWrapper<'a> {
    type Target = ObjectWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for ActorWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> ActorWrapper<'a> {
    pub unsafe fn new(actor: *mut AActor) -> ActorWrapper<'a> {
        let wrapper = ActorWrapper { base: ObjectWrapper::new(actor as *mut UObject) };
        assert!(wrapper.class().extends_from("Actor"));
        wrapper
    }
    pub fn as_ptr(&self) -> *mut AActor {
        self.base.as_ptr() as *mut AActor
    }

    pub fn absolute_location(&self) -> (f32, f32, f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let absolute_location = root_component.get_field("AbsoluteLocation").unwrap_struct();
        (
            *absolute_location.get_field("X").unwrap_float(),
            *absolute_location.get_field("Y").unwrap_float(),
            *absolute_location.get_field("Z").unwrap_float(),
        )
    }
    pub fn _set_absolute_location(&self, x: f32, y: f32, z: f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let absolute_location = root_component.get_field("AbsoluteLocation").unwrap_struct();
        *absolute_location.get_field("X").unwrap_float() = x;
        *absolute_location.get_field("Y").unwrap_float() = y;
        *absolute_location.get_field("Z").unwrap_float() = z;
    }
    pub fn _absolute_rotation(&self) -> (f32, f32, f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let absolute_rotation = root_component.get_field("AbsoluteRotation").unwrap_struct();
        (
            *absolute_rotation.get_field("Pitch").unwrap_float(),
            *absolute_rotation.get_field("Yaw").unwrap_float(),
            *absolute_rotation.get_field("Roll").unwrap_float(),
        )
    }
    pub fn _set_absolute_rotation(&self, pitch: f32, yaw: f32, roll: f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let absolute_rotation = root_component.get_field("AbsoluteRotation").unwrap_struct();
        *absolute_rotation.get_field("Pitch").unwrap_float() = pitch;
        *absolute_rotation.get_field("Yaw").unwrap_float() = yaw;
        *absolute_rotation.get_field("Roll").unwrap_float() = roll;
    }
    pub fn _absolute_scale(&self) -> (f32, f32, f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let absolute_scale = root_component.get_field("AbsoluteScale3D").unwrap_struct();
        (
            *absolute_scale.get_field("X").unwrap_float(),
            *absolute_scale.get_field("Y").unwrap_float(),
            *absolute_scale.get_field("Z").unwrap_float(),
        )
    }
    pub fn _set_absolute_scale(&self, xscale: f32, yscale: f32, zscale: f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let absolute_scale = root_component.get_field("AbsoluteScale3D").unwrap_struct();
        *absolute_scale.get_field("X").unwrap_float() = xscale;
        *absolute_scale.get_field("Y").unwrap_float() = yscale;
        *absolute_scale.get_field("Z").unwrap_float() = zscale;
    }
    pub fn relative_location(&self) -> (f32, f32, f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let relative_location = root_component.get_field("RelativeLocation").unwrap_struct();
        (
            *relative_location.get_field("X").unwrap_float(),
            *relative_location.get_field("Y").unwrap_float(),
            *relative_location.get_field("Z").unwrap_float(),
        )
    }
    pub fn set_relative_location(&self, x: f32, y: f32, z: f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let relative_location = root_component.get_field("RelativeLocation").unwrap_struct();
        *relative_location.get_field("X").unwrap_float() = x;
        *relative_location.get_field("Y").unwrap_float() = y;
        *relative_location.get_field("Z").unwrap_float() = z;
    }
    pub fn relative_rotation(&self) -> (f32, f32, f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let relative_rotation = root_component.get_field("RelativeRotation").unwrap_struct();
        (
            *relative_rotation.get_field("Pitch").unwrap_float(),
            *relative_rotation.get_field("Yaw").unwrap_float(),
            *relative_rotation.get_field("Roll").unwrap_float(),
        )
    }
    pub fn set_relative_rotation(&self, pitch: f32, yaw: f32, roll: f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let relative_rotation = root_component.get_field("RelativeRotation").unwrap_struct();
        *relative_rotation.get_field("Pitch").unwrap_float() = pitch;
        *relative_rotation.get_field("Yaw").unwrap_float() = yaw;
        *relative_rotation.get_field("Roll").unwrap_float() = roll;
    }
    pub fn relative_scale(&self) -> (f32, f32, f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let relative_scale = root_component.get_field("RelativeScale3D").unwrap_struct();
        (
            *relative_scale.get_field("X").unwrap_float(),
            *relative_scale.get_field("Y").unwrap_float(),
            *relative_scale.get_field("Z").unwrap_float(),
        )
    }
    pub fn set_relative_scale(&self, xscale: f32, yscale: f32, zscale: f32) {
        let root_component = self.get_field("RootComponent").unwrap_object();
        let relative_scale = root_component.get_field("RelativeScale3D").unwrap_struct();
        *relative_scale.get_field("X").unwrap_float() = xscale;
        *relative_scale.get_field("Y").unwrap_float() = yscale;
        *relative_scale.get_field("Z").unwrap_float() = zscale;
    }
}


/// Wrapper for a UE-owned array
#[derive(Debug)]
pub struct ArrayWrapper<'a, T: ArrayElement> {
    array: *mut TArray<T::ElementType>,
    _marker: PhantomData<&'a mut ()>,
}
impl<'a, T: ArrayElement> Pointer for ArrayWrapper<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.array, f)
    }
}
// get rid of the implied T: Clone in derived Clone impls
impl<'a, T: ArrayElement> Clone for ArrayWrapper<'a, T> {
    fn clone(&self) -> Self {
        Self {
            array: self.array,
            _marker: PhantomData,
        }
    }
}
unsafe impl<'a, T: ArrayElement> UeObjectWrapper for ArrayWrapper<'a, T> {
    type Wrapping = TArray<T::ElementType>;
    const CLASS_NAME: &'static str = "Array";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        ArrayWrapper::new(ptr)
    }
}
impl<'a, T: ArrayElement> ArrayWrapper<'a, T> {
    pub unsafe fn new(array: *mut TArray<T::ElementType>) -> ArrayWrapper<'a, T> {
        ArrayWrapper { array, _marker: PhantomData }
    }
    pub fn len(&self) -> usize {
        unsafe { (*self.array).len() }
    }
    pub fn capacity(&self) -> usize {
        unsafe { (*self.array).capacity() }
    }
    pub fn get(&self, index: usize) -> Option<T> {
        unsafe { (*self.array).get_mut(index).map(|ptr| T::create(ptr)) }
    }
}
pub struct ArrayWrapperIter<'a, T: ArrayElement> {
    array_wrapper: ArrayWrapper<'a, T>,
    index: usize,
}
impl<'a, T: ArrayElement> Iterator for ArrayWrapperIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array_wrapper.len() {
            return None;
        }
        let e = self.array_wrapper.get(self.index).unwrap();
        self.index += 1;
        Some(e)
    }
}
impl<'a, 'b, T: ArrayElement> IntoIterator for &'b ArrayWrapper<'a, T> {
    type Item = T;
    type IntoIter = ArrayWrapperIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayWrapperIter {
            array_wrapper: (*self).clone(),
            index: 0,
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
        "Int8Property" => DynamicValue::Int8(&mut *checked_cast::<i8>(value_ptr)),
        "Int16Property" => DynamicValue::Int16(&mut *checked_cast::<i16>(value_ptr)),
        "IntProperty" => DynamicValue::Int(&mut *checked_cast::<i32>(value_ptr)),
        "Int64Property" => DynamicValue::Int64(&mut *checked_cast::<i64>(value_ptr)),
        "ByteProperty" => DynamicValue::Byte(&mut *checked_cast::<u8>(value_ptr)),
        "UInt16Property" => DynamicValue::UInt16(&mut *checked_cast::<u16>(value_ptr)),
        "UInt32Property" => DynamicValue::UInt32(&mut *checked_cast::<u32>(value_ptr)),
        "UInt64Property" => DynamicValue::UInt64(&mut *checked_cast::<u64>(value_ptr)),
        "FloatProperty" => DynamicValue::Float(&mut *checked_cast::<f32>(value_ptr)),
        "DoubleProperty" => DynamicValue::Double(&mut *checked_cast::<f64>(value_ptr)),
        "BoolProperty" => todo!("BoolProperty"),
        "ObjectProperty" | "WeakObjectProperty" | "LazyObjectProperty" | "SoftObjectProperty" => {
            DynamicValue::Object(ObjectWrapper::new(*checked_cast::<*mut UObject>(value_ptr)))
        },
        "ClassProperty" | "SoftClassProperty" => todo!("ClassProperty"),
        "InterfaceProperty" => todo!("InterfaceProperty"),
        "NameProperty" => DynamicValue::Name(*checked_cast::<FName>(value_ptr)),
        "StrProperty" => DynamicValue::Str(checked_cast::<FString>(value_ptr)),
        "ArrayProperty" => DynamicValue::Array(checked_cast::<TArray<*mut ()>>(value_ptr) as *mut c_void, PropertyWrapper::new((*info.prop.as_uarrayproperty()).inner)),
        "MapProperty" => todo!("MapProperty"),
        "SetProperty" => todo!("SetProperty"),
        "StructProperty" => DynamicValue::Struct(ObjectStructFieldWrapper::new(value_ptr, StructWrapper::new((*(info.prop.as_ptr() as *mut UStructProperty)).struct_))),
        "DelegateProperty" | "MulticastDelegateProperty" | "MulticastInlineDelegateProperty" | "MulticastSparseDelegateProperty" => todo!("Function-based Properties"),
        "EnumProperty" => todo!("EnumProperty"),
        "TextProperty" => todo!("TextProperty"),
        _ => unreachable!("Got unknown UE property kind {property_kind}"),
    }
}

