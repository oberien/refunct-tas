use std::alloc::{Layout, System, GlobalAlloc};
use std::cell::Cell;
use std::ffi::c_void;
use std::fmt::{Display, Formatter, Pointer};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use itertools::Itertools;
use crate::native::reflection::{AActor, DynamicValue, UArrayProperty, UClass, UeObjectWrapper, UObject, UObjectProperty, UProperty, UStruct, UStructProperty};
use crate::native::ue::TArray;
use crate::native::{ArrayElement, UBoolProperty, UeObjectWrapperType, UField, UFunction, UOBJECT_PROCESSEVENT};

#[derive(Debug, Clone)]
pub struct BoolValueWrapper<'a> {
    ptr: *mut u8,
    bool_property: BoolPropertyWrapper<'a>,
    _marker: PhantomData<&'a mut bool>,
}
impl<'a> Pointer for BoolValueWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.ptr, f)
    }
}
impl<'a> ArrayElement<'a> for BoolValueWrapper<'a> {
    unsafe fn create(ptr: *mut c_void, prop: &PropertyWrapper<'a>) -> BoolValueWrapper<'a> {
        let bool_property = prop.upcast::<BoolPropertyWrapper<'a>>();
        BoolValueWrapper::new(ptr as *mut u8, bool_property)
    }
}
impl<'a> BoolValueWrapper<'a> {
    pub unsafe fn new(ptr: *mut u8, bool_property: BoolPropertyWrapper<'a>) -> BoolValueWrapper<'a> {
        assert!(!ptr.is_null());
        BoolValueWrapper { ptr, bool_property, _marker: PhantomData }
    }
    pub fn _get(&self) -> bool {
        unsafe {
            let ptr = self.ptr.offset(self.bool_property.byte_offset() as isize);
            if self.bool_property.field_mask() == 0xff {
                // full bool byte
                *ptr != 0
            } else {
                // bitfield
                (*ptr & self.bool_property.byte_mask()) != 0
            }
        }
    }
    pub fn set(&self, value: bool) {
        unsafe {
            let ptr = self.ptr.offset(self.bool_property.byte_offset() as isize);
            if self.bool_property.field_mask() == 0xff {
                // full bool byte
                *ptr = value as u8
            } else {
                // bitfield
                if value {
                    *ptr |= self.bool_property.byte_mask()
                } else {
                    *ptr &= !self.bool_property.byte_mask()
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct StructValueWrapper<'a> {
    ptr: *mut u8,
    struct_information: StructWrapper<'a>,
    limit_num_fields: usize,
    _marker: PhantomData<&'a mut UObject>,
}
impl<'a> Pointer for StructValueWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.ptr, f)
    }
}
impl<'a> ArrayElement<'a> for StructValueWrapper<'a> {
    unsafe fn create(ptr: *mut c_void, prop: &PropertyWrapper<'a>) -> StructValueWrapper<'a> {
        let struct_information = prop.upcast::<StructPropertyWrapper<'a>>().struct_();
        StructValueWrapper::new(ptr, struct_information)
    }
}
impl<'a> StructValueWrapper<'a> {
    pub unsafe fn new(ptr: *mut c_void, struct_information: StructWrapper<'a>) -> StructValueWrapper<'a> {
        Self::with_limit_num_fields(ptr, struct_information, usize::MAX)
    }
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr as *mut c_void
    }
    pub unsafe fn with_limit_num_fields(ptr: *mut c_void, struct_information: StructWrapper<'a>, limit_num_fields: usize) -> StructValueWrapper<'a> {
        assert!(!ptr.is_null());
        StructValueWrapper { ptr: ptr as *mut u8, struct_information, limit_num_fields, _marker: PhantomData }
    }
    pub fn get_field(&self, name: &str) -> DynamicValue {
        unsafe {
            let field_info = self.struct_information.get_field_info(name, self.limit_num_fields);
            apply_field_info(self.ptr, field_info)
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwningStructValueWrapper<'a> {
    inner: StructValueWrapper<'a>,
    layout: Layout,
}
impl<'a> Deref for OwningStructValueWrapper<'a> {
    type Target = StructValueWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Pointer for OwningStructValueWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.inner, f)
    }
}
impl<'a> OwningStructValueWrapper<'a> {
    pub unsafe fn new(struct_information: StructWrapper<'a>, limit_num_fields: usize, size: usize) -> OwningStructValueWrapper<'a> {
        // check that the size makes sense
        let mut calculated_size = 0;
        for field in struct_information.iter_fields().take(limit_num_fields) {
            let prop: PropertyWrapper = field.upcast();
            calculated_size = prop.offset() as usize + prop.size();
        }
        assert_eq!(size, calculated_size);
        let align = struct_information.min_alignment();
        let layout = Layout::from_size_align(size, align).unwrap();
        let ptr = System.alloc_zeroed(layout);
        OwningStructValueWrapper {
            inner: StructValueWrapper::with_limit_num_fields(ptr as *mut c_void, struct_information.clone(), limit_num_fields),
            layout,
        }
    }
}
impl<'a> Drop for OwningStructValueWrapper<'a> {
    fn drop(&mut self) {
        unsafe { System.dealloc(self.inner.ptr, self.layout) }
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
#[derive(Debug)]
pub enum ObjectWrapperType {}
impl UeObjectWrapperType for ObjectWrapperType {
    type UeObjectWrapper<'a> = ObjectWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for ObjectWrapper<'a> {
    type UeObjectWrapperType = ObjectWrapperType;
    type Wrapping = UObject;
    const CLASS_NAME: &'static str = "Object";

    unsafe fn create(ptr: *mut Self::Wrapping) -> ObjectWrapper<'a> {
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
        assert!(!object.is_null());
        ObjectWrapper { object, _marker: PhantomData }
    }
    pub unsafe fn new_nullable(object: *mut UObject) -> Option<ObjectWrapper<'a>> {
        (!object.is_null()).then(|| ObjectWrapper::new(object))
    }
    pub fn as_ptr(&self) -> *mut UObject {
        self.object
    }

    pub fn vtable(&self) -> *const () {
        unsafe { (*self.object).vtable }
    }
    pub fn internal_index(&self) -> i32 {
        unsafe { (*self.object).internal_index }
    }
    pub fn name(&self) -> String {
        unsafe { (*self.object).name.to_string_lossy() }
    }
    pub fn class(&self) -> ClassWrapper<'a> {
        unsafe { ClassWrapper::new((*self.object).class) }
    }

    pub fn get_field(&self, name: &str) -> DynamicValue<'a> {
        unsafe {
            let field_info = self.class().get_field_info(name, usize::MAX);
            apply_field_info(self.object as *mut u8, field_info)
        }
    }

    pub fn upcast<T: UeObjectWrapper<'a>>(&self) -> T {
        self.try_upcast().unwrap_or_else(|| panic!("can't upcast {} to {}", self.class().name(), T::CLASS_NAME))
    }
    pub fn try_upcast<T: UeObjectWrapper<'a>>(&self) -> Option<T> {
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
pub enum FieldWrapperType {}
impl UeObjectWrapperType for FieldWrapperType {
    type UeObjectWrapper<'a> = FieldWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for FieldWrapper<'a> {
    type UeObjectWrapperType = FieldWrapperType;
    type Wrapping = UField;
    const CLASS_NAME: &'static str = "Field";

    unsafe fn create(ptr: *mut Self::Wrapping) -> FieldWrapper<'a> {
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
        assert!(!field.is_null());
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

    pub fn iter_this_and_next_fields(&self) -> impl Iterator<Item = FieldWrapper<'a>> {
        struct FieldIter<'a> {
            next_field: Option<FieldWrapper<'a>>,
        }
        impl<'a> Iterator for FieldIter<'a> {
            type Item = FieldWrapper<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                if let Some(field) = self.next_field.take() {
                    self.next_field = field.next_field();
                    Some(field)
                } else {
                    None
                }
            }
        }
        FieldIter { next_field: Some(self.clone()) }
    }
}

#[derive(Debug, Clone)]
pub struct PropertyWrapper<'a> {
    base: FieldWrapper<'a>,
}
pub enum PropertyWrapperType {}
impl UeObjectWrapperType for PropertyWrapperType {
    type UeObjectWrapper<'a> = PropertyWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for PropertyWrapper<'a> {
    type UeObjectWrapperType = PropertyWrapperType;
    type Wrapping = UProperty;
    const CLASS_NAME: &'static str = "Property";

    unsafe fn create(ptr: *mut Self::Wrapping) -> PropertyWrapper<'a> {
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
        assert!(!prop.is_null());
        PropertyWrapper { base: FieldWrapper::new(prop as *mut UField) }
    }
    pub fn as_ptr(&self) -> *mut UProperty {
        self.base.as_ptr() as *mut UProperty
    }
    pub fn property_kind(&self) -> String {
        self.class().name()
    }
    pub fn offset(&self) -> isize {
        unsafe { (*self.as_ptr()).offset_internal as isize }
    }
    pub fn size(&self) -> usize {
        unsafe { (*self.as_ptr()).element_size.try_into().unwrap() }
    }

    pub fn iter_this_and_next_properties(&self) -> impl Iterator<Item = PropertyWrapper<'a>> {
        self.iter_this_and_next_fields().map(|field| field.upcast())
    }
}
impl<'a> Display for PropertyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({:p}) (offset {:#x})", self.class().name(), self.name(), self.as_ptr(), self.offset())
    }
}

#[derive(Debug, Clone)]
pub struct ObjectPropertyWrapper<'a> {
    base: PropertyWrapper<'a>,
}
pub enum ObjectPropertyWrapperType {}
impl UeObjectWrapperType for ObjectPropertyWrapperType {
    type UeObjectWrapper<'a> = ObjectPropertyWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for ObjectPropertyWrapper<'a> {
    type UeObjectWrapperType = ObjectPropertyWrapperType;
    type Wrapping = UObjectProperty;
    const CLASS_NAME: &'static str = "ObjectProperty";

    unsafe fn create(ptr: *mut Self::Wrapping) -> ObjectPropertyWrapper<'a> {
        ObjectPropertyWrapper::new(ptr)
    }
}
impl<'a> Deref for ObjectPropertyWrapper<'a> {
    type Target = PropertyWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for ObjectPropertyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> ObjectPropertyWrapper<'a> {
    pub unsafe fn new(prop: *mut UObjectProperty) -> ObjectPropertyWrapper<'a> {
        assert!(!prop.is_null());
        ObjectPropertyWrapper { base: PropertyWrapper::new(prop as *mut UProperty) }
    }
    pub fn as_ptr(&self) -> *mut UObjectProperty {
        self.base.as_ptr() as *mut UObjectProperty
    }
    pub fn property_class(&self) -> ClassWrapper<'a> {
        unsafe { ClassWrapper::new((*self.as_ptr()).property_class) }
    }
}

#[derive(Debug, Clone)]
pub struct ArrayPropertyWrapper<'a> {
    base: PropertyWrapper<'a>,
}
pub enum ArrayPropertyWrapperType {}
impl UeObjectWrapperType for ArrayPropertyWrapperType {
    type UeObjectWrapper<'a> = ArrayPropertyWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for ArrayPropertyWrapper<'a> {
    type UeObjectWrapperType = ArrayPropertyWrapperType;
    type Wrapping = UArrayProperty;
    const CLASS_NAME: &'static str = "ArrayProperty";

    unsafe fn create(ptr: *mut Self::Wrapping) -> ArrayPropertyWrapper<'a> {
        ArrayPropertyWrapper::new(ptr)
    }
}
impl<'a> Deref for ArrayPropertyWrapper<'a> {
    type Target = PropertyWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for ArrayPropertyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> ArrayPropertyWrapper<'a> {
    pub unsafe fn new(prop: *mut UArrayProperty) -> ArrayPropertyWrapper<'a> {
        assert!(!prop.is_null());
        ArrayPropertyWrapper { base: PropertyWrapper::new(prop as *mut UProperty) }
    }
    pub fn as_ptr(&self) -> *mut UArrayProperty {
        self.base.as_ptr() as *mut UArrayProperty
    }
    pub fn inner(&self) -> PropertyWrapper<'a> {
        unsafe { PropertyWrapper::new((*self.as_ptr()).inner) }
    }
}

#[derive(Debug, Clone)]
pub struct StructPropertyWrapper<'a> {
    base: PropertyWrapper<'a>,
}
pub enum StructPropertyWrapperType {}
impl UeObjectWrapperType for StructPropertyWrapperType {
    type UeObjectWrapper<'a> = StructPropertyWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for StructPropertyWrapper<'a> {
    type UeObjectWrapperType = StructPropertyWrapperType;
    type Wrapping = UStructProperty;
    const CLASS_NAME: &'static str = "StructProperty";

    unsafe fn create(ptr: *mut Self::Wrapping) -> StructPropertyWrapper<'a> {
        StructPropertyWrapper::new(ptr)
    }
}
impl<'a> Deref for StructPropertyWrapper<'a> {
    type Target = PropertyWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for StructPropertyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> StructPropertyWrapper<'a> {
    pub unsafe fn new(prop: *mut UStructProperty) -> StructPropertyWrapper<'a> {
        assert!(!prop.is_null());
        StructPropertyWrapper { base: PropertyWrapper::new(prop as *mut UProperty) }
    }
    pub fn as_ptr(&self) -> *mut UStructProperty {
        self.base.as_ptr() as *mut UStructProperty
    }
    pub fn struct_(&self) -> StructWrapper<'a> {
        unsafe { StructWrapper::new((*self.as_ptr()).struct_) }
    }
}

#[derive(Debug, Clone)]
pub struct BoolPropertyWrapper<'a> {
    base: PropertyWrapper<'a>,
}
pub enum BoolPropertyWrapperType {}
impl UeObjectWrapperType for BoolPropertyWrapperType {
    type UeObjectWrapper<'a> = BoolPropertyWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for BoolPropertyWrapper<'a> {
    type UeObjectWrapperType = BoolPropertyWrapperType;
    type Wrapping = UBoolProperty;
    const CLASS_NAME: &'static str = "BoolProperty";

    unsafe fn create(ptr: *mut Self::Wrapping) -> BoolPropertyWrapper<'a> {
        BoolPropertyWrapper::new(ptr)
    }
}
impl<'a> Deref for BoolPropertyWrapper<'a> {
    type Target = PropertyWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for BoolPropertyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> BoolPropertyWrapper<'a> {
    pub unsafe fn new(prop: *mut UBoolProperty) -> BoolPropertyWrapper<'a> {
        assert!(!prop.is_null());
        BoolPropertyWrapper { base: PropertyWrapper::new(prop as *mut UProperty) }
    }
    pub fn as_ptr(&self) -> *mut UBoolProperty {
        self.base.as_ptr() as *mut UBoolProperty
    }
    pub fn byte_offset(&self) -> usize {
        unsafe { (*self.as_ptr()).byte_offset as usize }
    }
    pub fn byte_mask(&self) -> u8 {
        unsafe { (*self.as_ptr()).byte_mask }
    }
    pub fn field_mask(&self) -> u8 {
        unsafe { (*self.as_ptr()).field_mask }
    }
}

#[derive(Debug, Clone)]
pub struct StructWrapper<'a> {
    base: FieldWrapper<'a>,
}
pub enum StructWrapperType {}
impl UeObjectWrapperType for StructWrapperType {
    type UeObjectWrapper<'a> = StructWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for StructWrapper<'a> {
    type UeObjectWrapperType = StructWrapperType;
    type Wrapping = UStruct;
    const CLASS_NAME: &'static str = "Struct";

    unsafe fn create(ptr: *mut Self::Wrapping) -> StructWrapper<'a> {
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
        assert!(!struct_.is_null());
        StructWrapper { base: FieldWrapper::new(struct_ as *mut UField) }
    }
    pub fn as_ptr(&self) -> *mut UStruct {
        self.base.as_ptr() as *mut UStruct
    }

    pub fn super_struct(&self) -> Option<StructWrapper<'a>> {
        let super_class = unsafe { (*self.as_ptr()).super_struct };
        if super_class.is_null() {
            None
        } else {
            unsafe { Some(StructWrapper::new(super_class)) }
        }
    }
    pub fn children(&self) -> Option<FieldWrapper<'a>> {
        let children = unsafe { (*self.as_ptr()).children };
        if children.is_null() {
            None
        } else {
            unsafe { Some(FieldWrapper::new(children)) }
        }
    }
    pub fn properties_size(&self) -> usize {
        unsafe { (*self.as_ptr()).properties_size.try_into().unwrap() }
    }
    pub fn min_alignment(&self) -> usize {
        unsafe { (*self.as_ptr()).min_alignment.try_into().unwrap() }
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
    pub fn find_property(&self, name: &str) -> Option<PropertyWrapper<'a>> {
        self.iter_properties().find(|f| f.name() == name)
    }
    pub fn iter_functions(&self) -> impl Iterator<Item = FunctionWrapper<'a>> {
        self.iter_fields()
            .filter_map(|field| field.try_upcast::<FunctionWrapper>())
    }
    pub fn find_function(&self, name: &str) -> Option<FunctionWrapper<'a>> {
        self.iter_functions().find(|f| f.name() == name)
    }

    fn get_field_info(&self, mut name: &str, limit_num_fields: usize) -> FieldInfo<'a> {
        let hacked_absolute = (name == "AbsoluteLocation" || name == "AbsoluteRotation" || name == "AbsoluteScale3D") && self.extends_from("SceneComponent");
        if hacked_absolute {
            match name {
                "AbsoluteLocation" => name = "RelativeLocation",
                "AbsoluteRotation" => name = "RelativeRotation",
                "AbsoluteScale3D" => name = "RelativeScale3D",
                _ => unreachable!(),
            }
        }
        let prop = self.iter_properties().take(limit_num_fields)
            .find(|prop| prop.name() == name)
            .unwrap_or_else(|| panic!("cannot access property {name} of type {}, properties available: {}", self.class().name(),
                self.iter_properties().take(limit_num_fields).map(|prop| format!("{} {}", prop.class().name(), prop.name())).join(", "),
            )).upcast::<PropertyWrapper>();
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
pub enum ClassWrapperType {}
impl UeObjectWrapperType for ClassWrapperType {
    type UeObjectWrapper<'a> = ClassWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for ClassWrapper<'a> {
    type UeObjectWrapperType = ClassWrapperType;
    type Wrapping = UClass;
    const CLASS_NAME: &'static str = "Class";

    unsafe fn create(ptr: *mut Self::Wrapping) -> ClassWrapper<'a> {
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
        assert!(!class.is_null());
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
pub struct FunctionWrapper<'a> {
    base: StructWrapper<'a>,
}
pub enum FunctionWrapperType {}
impl UeObjectWrapperType for FunctionWrapperType {
    type UeObjectWrapper<'a> = FunctionWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for FunctionWrapper<'a> {
    type UeObjectWrapperType = FunctionWrapperType;
    type Wrapping = UFunction;
    const CLASS_NAME: &'static str = "Function";

    unsafe fn create(ptr: *mut Self::Wrapping) -> FunctionWrapper<'a> {
        FunctionWrapper::new(ptr)
    }
}
impl<'a> Deref for FunctionWrapper<'a> {
    type Target = StructWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for FunctionWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.as_ptr(), f)
    }
}
impl<'a> FunctionWrapper<'a> {
    pub unsafe fn new(function: *mut UFunction) -> FunctionWrapper<'a> {
        assert!(!function.is_null());
        FunctionWrapper { base: StructWrapper::new(function as *mut UStruct) }
    }
    pub fn as_ptr(&self) -> *mut UFunction {
        self.base.as_ptr() as *mut UFunction
    }

    pub fn num_parms(&self) -> u8 {
        unsafe { (*self.as_ptr()).num_parms }
    }
    pub fn parms_size(&self) -> u16 {
        unsafe { (*self.as_ptr()).parms_size }
    }

    pub fn iter_params(&self) -> impl Iterator<Item = PropertyWrapper<'a>> {
        self.iter_fields().take(self.num_parms() as usize).map(|field| field.upcast())
    }

    pub fn create_argument_struct(&self) -> OwningStructValueWrapper<'a> {
        unsafe {
            OwningStructValueWrapper::new((**self).clone(), self.num_parms() as usize, self.parms_size() as usize)
        }
    }
    pub unsafe fn call<This>(&self, this: *mut This, args: &OwningStructValueWrapper<'a>) {
        self.call_raw(this, args.as_ptr())
    }
    pub unsafe fn call_raw<This, Args>(&self, this: *mut This, args: *mut Args) {
        assert!(!this.is_null());
        assert!(!args.is_null());
        let fun: extern_fn!(fn(this: *mut This, function: *mut UFunction, args: *mut Args))
            = ::std::mem::transmute(UOBJECT_PROCESSEVENT.load(Ordering::SeqCst));
        fun(this, self.as_ptr(), args);
    }
}


#[derive(Debug, Clone)]
pub struct ActorWrapper<'a> {
    base: ObjectWrapper<'a>,
}
pub enum ActorWrapperType {}
impl UeObjectWrapperType for ActorWrapperType {
    type UeObjectWrapper<'a> = ActorWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for ActorWrapper<'a> {
    type UeObjectWrapperType = ActorWrapperType;
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "Actor";

    unsafe fn create(ptr: *mut Self::Wrapping) -> ActorWrapper<'a> {
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
        assert!(!actor.is_null());
        let wrapper = ActorWrapper { base: ObjectWrapper::new(actor as *mut UObject) };
        assert!(wrapper.class().extends_from("Actor"));
        wrapper
    }
    pub fn as_ptr(&self) -> *mut AActor {
        self.base.as_ptr() as *mut AActor
    }

    pub fn absolute_location(&self) -> (f32, f32, f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let absolute_location: StructValueWrapper = root_component.get_field("AbsoluteLocation").unwrap();
        (
            absolute_location.get_field("X").unwrap(),
            absolute_location.get_field("Y").unwrap(),
            absolute_location.get_field("Z").unwrap(),
        )
    }
    pub fn _set_absolute_location(&self, x: f32, y: f32, z: f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let absolute_location: StructValueWrapper = root_component.get_field("AbsoluteLocation").unwrap();
        absolute_location.get_field("X").unwrap::<&Cell<f32>>().set(x);
        absolute_location.get_field("Y").unwrap::<&Cell<f32>>().set(y);
        absolute_location.get_field("Z").unwrap::<&Cell<f32>>().set(z);
    }
    pub fn _absolute_rotation(&self) -> (f32, f32, f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let absolute_rotation: StructValueWrapper = root_component.get_field("AbsoluteRotation").unwrap();
        (
            absolute_rotation.get_field("Pitch").unwrap(),
            absolute_rotation.get_field("Yaw").unwrap(),
            absolute_rotation.get_field("Roll").unwrap(),
        )
    }
    pub fn _set_absolute_rotation(&self, pitch: f32, yaw: f32, roll: f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let absolute_rotation: StructValueWrapper = root_component.get_field("AbsoluteRotation").unwrap();
        absolute_rotation.get_field("Pitch").unwrap::<&Cell<f32>>().set(pitch);
        absolute_rotation.get_field("Yaw").unwrap::<&Cell<f32>>().set(yaw);
        absolute_rotation.get_field("Roll").unwrap::<&Cell<f32>>().set(roll);
    }
    pub fn _absolute_scale(&self) -> (f32, f32, f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let absolute_scale: StructValueWrapper = root_component.get_field("AbsoluteScale3D").unwrap();
        (
            absolute_scale.get_field("X").unwrap(),
            absolute_scale.get_field("Y").unwrap(),
            absolute_scale.get_field("Z").unwrap(),
        )
    }
    pub fn _set_absolute_scale(&self, xscale: f32, yscale: f32, zscale: f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let absolute_scale: StructValueWrapper = root_component.get_field("AbsoluteScale3D").unwrap();
        absolute_scale.get_field("X").unwrap::<&Cell<f32>>().set(xscale);
        absolute_scale.get_field("Y").unwrap::<&Cell<f32>>().set(yscale);
        absolute_scale.get_field("Z").unwrap::<&Cell<f32>>().set(zscale);
    }
    pub fn relative_location(&self) -> (f32, f32, f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let relative_location: StructValueWrapper = root_component.get_field("RelativeLocation").unwrap();
        (
            relative_location.get_field("X").unwrap(),
            relative_location.get_field("Y").unwrap(),
            relative_location.get_field("Z").unwrap(),
        )
    }
    pub fn _set_relative_location(&self, x: f32, y: f32, z: f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let relative_location: StructValueWrapper = root_component.get_field("RelativeLocation").unwrap();
        relative_location.get_field("X").unwrap::<&Cell<f32>>().set(x);
        relative_location.get_field("Y").unwrap::<&Cell<f32>>().set(y);
        relative_location.get_field("Z").unwrap::<&Cell<f32>>().set(z);
    }
    pub fn relative_rotation(&self) -> (f32, f32, f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let relative_rotation: StructValueWrapper = root_component.get_field("RelativeRotation").unwrap();
        (
            relative_rotation.get_field("Pitch").unwrap(),
            relative_rotation.get_field("Yaw").unwrap(),
            relative_rotation.get_field("Roll").unwrap(),
        )
    }
    pub fn _set_relative_rotation(&self, pitch: f32, yaw: f32, roll: f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let relative_rotation: StructValueWrapper = root_component.get_field("RelativeRotation").unwrap();
        relative_rotation.get_field("Pitch").unwrap::<&Cell<f32>>().set(pitch);
        relative_rotation.get_field("Yaw").unwrap::<&Cell<f32>>().set(yaw);
        relative_rotation.get_field("Roll").unwrap::<&Cell<f32>>().set(roll);
    }
    pub fn relative_scale(&self) -> (f32, f32, f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let relative_scale: StructValueWrapper = root_component.get_field("RelativeScale3D").unwrap();
        (
            relative_scale.get_field("X").unwrap(),
            relative_scale.get_field("Y").unwrap(),
            relative_scale.get_field("Z").unwrap(),
        )
    }
    pub fn _set_relative_scale(&self, xscale: f32, yscale: f32, zscale: f32) {
        let root_component: ObjectWrapper = self.get_field("RootComponent").unwrap();
        let relative_scale: StructValueWrapper = root_component.get_field("RelativeScale3D").unwrap();
        relative_scale.get_field("X").unwrap::<&Cell<f32>>().set(xscale);
        relative_scale.get_field("Y").unwrap::<&Cell<f32>>().set(yscale);
        relative_scale.get_field("Z").unwrap::<&Cell<f32>>().set(zscale);
    }
    /// origin x,y,z, half-size x,y,z
    pub fn get_actor_bounds(&self) -> (f32, f32, f32, f32, f32, f32) {
        let get_actor_bounds = self.class().find_function("GetActorBounds").unwrap();
        let args = get_actor_bounds.create_argument_struct();
        unsafe { get_actor_bounds.call(self.as_ptr(), &args) };
        let origin: StructValueWrapper = args.get_field("Origin").unwrap();
        let extent: StructValueWrapper = args.get_field("BoxExtent").unwrap();
        (
            origin.get_field("X").unwrap(), origin.get_field("Y").unwrap(), origin.get_field("Z").unwrap(),
            extent.get_field("X").unwrap(), extent.get_field("Y").unwrap(), extent.get_field("Z").unwrap(),
        )
    }
}


/// Wrapper for a UE-owned array
#[derive(Debug)]
pub struct ArrayWrapper<'a, T: ArrayElement<'a>> {
    array: *mut TArray<u8>,
    element_prop: PropertyWrapper<'a>,
    _marker: PhantomData<&'a mut [T]>,
}
impl<'a, T: ArrayElement<'a>> Pointer for ArrayWrapper<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.array, f)
    }
}
// get rid of the implied T: Clone in derived Clone impls
impl<'a, T: ArrayElement<'a>> Clone for ArrayWrapper<'a, T> {
    fn clone(&self) -> Self {
        Self {
            array: self.array,
            element_prop: self.element_prop.clone(),
            _marker: PhantomData,
        }
    }
}
impl<'a, T: ArrayElement<'a>> ArrayElement<'a> for ArrayWrapper<'a, T> {
    unsafe fn create(ptr: *mut c_void, prop: &PropertyWrapper<'a>) -> ArrayWrapper<'a, T> {
        let element_prop = prop.upcast::<ArrayPropertyWrapper<'a>>().inner();
        ArrayWrapper::new(ptr as *mut TArray<c_void>, element_prop)
    }
}
impl<'a, T: ArrayElement<'a>> ArrayWrapper<'a, T> {
    pub unsafe fn new(array: *mut TArray<c_void>, element_prop: PropertyWrapper<'a>) -> ArrayWrapper<'a, T> {
        ArrayWrapper { array: array as *mut TArray<u8>, element_prop, _marker: PhantomData }
    }
    pub fn len(&self) -> usize {
        unsafe { (*self.array).len() }
    }
    pub fn capacity(&self) -> usize {
        unsafe { (*self.array).capacity() }
    }
    pub fn get(&self, index: usize) -> Option<T> {
        unsafe {
            let index = (*self.array).check_index_for_indexing(index).ok()?;
            let offset = index.checked_mul(self.element_prop.size().try_into().unwrap()).unwrap();
            let ptr = (*self.array).ptr.offset(offset);
            Some(T::create(ptr as *mut c_void, &self.element_prop))
        }
    }
}
pub struct ArrayWrapperIter<'a, T: ArrayElement<'a>> {
    array_wrapper: ArrayWrapper<'a, T>,
    index: usize,
}
impl<'a, T: ArrayElement<'a>> Iterator for ArrayWrapperIter<'a, T> {
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
impl<'a, 'b, T: ArrayElement<'a>> IntoIterator for &'b ArrayWrapper<'a, T> {
    type Item = T;
    type IntoIter = ArrayWrapperIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayWrapperIter {
            array_wrapper: (*self).clone(),
            index: 0,
        }
    }
}

unsafe fn apply_field_info<'a>(ptr: *mut u8, info: FieldInfo<'a>) -> DynamicValue<'a> {
    assert!(!ptr.is_null());
    let value_ptr = ptr.offset(info.offset) as *mut c_void;
    DynamicValue::new(value_ptr, info.prop)
}

