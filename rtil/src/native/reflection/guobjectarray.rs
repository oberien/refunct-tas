use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::Ordering;
use crate::native::linux::GUOBJECTARRAY;
use crate::native::reflection::{ObjectWrapper, UObject};

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
        ObjectArrayWrapper { array, _marker: PhantomData }
    }
    pub fn max_elements(&self) -> usize {
        unsafe { (*self.array).max_elements.try_into().unwrap() }
    }
    pub fn num_elements(&self) -> usize {
        unsafe { (*self.array).num_elements.try_into().unwrap() }
    }
    pub fn iter_elements(&'a self) -> impl Iterator<Item = ObjectItemWrapper<'a>> + 'a {
        struct ObjectArrayIterator<'a> {
            array: &'a ObjectArrayWrapper<'a>,
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
                Some(self.array.get(index))
            }
        }
        ObjectArrayIterator {
            array: self,
            index: 0,
        }
    }
    pub fn get_object(&self, obj: &ObjectWrapper<'a>) -> ObjectItemWrapper<'a> {
        let item = self.get(obj.internal_index());
        assert_eq!(item.object().as_ptr() as usize, obj.as_ptr() as usize);
        item
    }
    pub fn get(&self, index: usize) -> ObjectItemWrapper<'a> {
        unsafe {
            assert!(index < self.num_elements(), "assert {} < {}", index, self.num_elements());
            let item = (*self.array).objects.offset(index.try_into().unwrap());
            ObjectItemWrapper::new(item)
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
        ObjectItemWrapper { item, _marker: PhantomData }
    }
    pub fn as_ptr(&self) -> *mut FUObjectItem {
        self.item
    }

    pub fn object(&self) -> ObjectWrapper<'a> {
        unsafe { ObjectWrapper::new((*self.item).object) }
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
