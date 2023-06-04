use std::collections::HashMap;
use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::native::GUOBJECTARRAY;
use crate::native::reflection::{ObjectWrapper, UObject};

#[derive(Debug, Clone, Copy)]
pub struct ObjectIndex {
    internal_index: i32,
    serial_number: i32,
}
static BUFFER: Lazy<Mutex<HashMap<u32, ObjectIndex>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub enum GetBufferedItemError {
    NotBuffered,
    Invalidated,
}

pub struct UeScope(());

impl UeScope {
    pub fn with<F: for<'a> FnOnce(&'a UeScope)>(f: F) {
        let scope = UeScope(());
        f(&scope)
    }

    fn object_array<'a>(&'a self) -> ObjectArrayWrapper<'a> {
        unsafe { GlobalObjectArrayWrapper::get().object_array() }
    }

    pub fn iter_global_object_array<'a>(&'a self) -> impl Iterator<Item = ObjectItemWrapper<'a>> + 'a {
        self.object_array().iter_elements()
    }

    pub fn object_index<'a>(&'a self, item: &ObjectItemWrapper<'a>) -> ObjectIndex {
        ObjectIndex {
            internal_index: item.object().internal_index(),
            serial_number: item.serial_number(),
        }
    }
    pub fn resolve_object_index<'a>(&'a self, index: ObjectIndex) -> Result<ObjectWrapper<'a>, GetBufferedItemError> {
        let item = self.object_array().try_get(index.internal_index)
            .ok_or(GetBufferedItemError::Invalidated)?;
        if item.serial_number() != index.serial_number {
            return Err(GetBufferedItemError::Invalidated);
        }
        Ok(item.object())
    }
    pub fn buffer_item<'a>(&'a self, ident: impl Into<u32>, item: &ObjectItemWrapper<'a>) {
        BUFFER.lock().unwrap().insert(ident.into(), self.object_index(item));
    }
    pub fn get_buffered<'a>(&'a self, ident: impl Into<u32>) -> Result<ObjectWrapper<'a>, GetBufferedItemError> {
        let index = BUFFER.lock().unwrap().get(&ident.into()).copied()
            .ok_or(GetBufferedItemError::NotBuffered)?;
        self.resolve_object_index(index)
    }
}

/// Wraps the global FUObjectArray (symbol GUObjectArray)
#[derive(Debug, Clone)]
pub struct GlobalObjectArrayWrapper<'a> {
    guobjectarray: *mut FUObjectArray,
    _marker: PhantomData<&'a mut FUObjectArray>,
}

impl<'a> GlobalObjectArrayWrapper<'a> {
    pub unsafe fn get() -> GlobalObjectArrayWrapper<'a> {
        GlobalObjectArrayWrapper {
            guobjectarray: GUOBJECTARRAY.load(Ordering::SeqCst) as *mut FUObjectArray,
            _marker: PhantomData,
        }
    }

    pub fn object_array(&self) -> ObjectArrayWrapper<'a> {
        unsafe { ObjectArrayWrapper::new(ptr::addr_of_mut!((*self.guobjectarray).obj_objects)) }
    }
}

/// Wraps the actual list containing ObjectItems (TUObjectArray)
#[derive(Debug, Clone)]
pub struct ObjectArrayWrapper<'a> {
    array: *mut TUObjectArray,
    _marker: PhantomData<&'a mut TUObjectArray>,
}

impl<'a> ObjectArrayWrapper<'a> {
    pub unsafe fn new(array: *mut TUObjectArray) -> ObjectArrayWrapper<'a> {
        assert!(!array.is_null());
        ObjectArrayWrapper { array, _marker: PhantomData }
    }
    pub fn _max_elements(&self) -> usize {
        unsafe { (*self.array).max_elements.try_into().unwrap() }
    }
    pub fn num_elements(&self) -> usize {
        unsafe { (*self.array).num_elements.try_into().unwrap() }
    }
    pub fn iter_elements(&self) -> impl Iterator<Item = ObjectItemWrapper<'a>> + 'a {
        struct ObjectArrayIterator<'a> {
            array: ObjectArrayWrapper<'a>,
            index: usize,
        }
        impl<'a> Iterator for ObjectArrayIterator<'a> {
            type Item = ObjectItemWrapper<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index >= self.array.num_elements() {
                    return None
                }
                let index = self.index;
                self.index += 1;
                Some(self.array.get(index.try_into().unwrap()))
            }
        }
        ObjectArrayIterator {
            array: self.clone(),
            index: 0,
        }
    }
    pub fn get_item_of_object(&self, obj: &ObjectWrapper<'a>) -> ObjectItemWrapper<'a> {
        let item = self.get(obj.internal_index());
        assert_eq!(item.object().as_ptr() as usize, obj.as_ptr() as usize);
        item
    }
    pub fn get(&self, internal_index: i32) -> ObjectItemWrapper<'a> {
        self.try_get(internal_index).unwrap_or_else(|| panic!("assert {} < {}", internal_index, self.num_elements()))
    }
    pub fn try_get(&self, internal_index: i32) -> Option<ObjectItemWrapper<'a>> {
        let index: usize = internal_index.try_into().unwrap();
        unsafe {
            if index >= self.num_elements() {
                None
            } else {
                let item = (*self.array).objects.offset(index.try_into().unwrap());
                Some(ObjectItemWrapper::new(item))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectItemWrapper<'a> {
    item: *mut FUObjectItem,
    _marker: PhantomData<&'a mut FUObjectItem>,
}

impl<'a> ObjectItemWrapper<'a> {
    pub unsafe fn new(item: *mut FUObjectItem) -> ObjectItemWrapper<'a> {
        assert!(!item.is_null());
        ObjectItemWrapper { item, _marker: PhantomData }
    }
    pub fn _as_ptr(&self) -> *mut FUObjectItem {
        self.item
    }

    pub fn object(&self) -> ObjectWrapper<'a> {
        unsafe { ObjectWrapper::new((*self.item).object) }
    }

    pub fn serial_number(&self) -> i32 {
        unsafe { (*self.item).serial_number }
    }

    pub fn mark_as_root_object(&self, val: bool) {
        unsafe {
            if val {
                (*self.item).flags |= EInternalObjectFlags::RootSet as i32;
            } else {
                (*self.item).flags &= !(EInternalObjectFlags::RootSet as i32);
            }
        }
    }
}

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
    pub object: *mut UObject,
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
