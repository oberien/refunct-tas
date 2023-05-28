use std::ptr;
use std::sync::atomic::Ordering;
use crate::native::linux::GUOBJECTARRAY;
use crate::native::reflection::{ObjectWrapper, UObject};

/// Wraps the global FUObjectArray (symbol GUObjectArray)
pub struct GlobalObjectArrayWrapper {
    guobjectarray: *mut FUObjectArray
}

impl GlobalObjectArrayWrapper {
    pub fn get() -> GlobalObjectArrayWrapper {
        GlobalObjectArrayWrapper {
            guobjectarray: GUOBJECTARRAY.load(Ordering::SeqCst) as *mut FUObjectArray,
        }
    }

    pub fn object_array(&self) -> ObjectArrayWrapper {
        unsafe { ObjectArrayWrapper::new(ptr::addr_of_mut!((*self.guobjectarray).obj_objects)) }
    }
}

/// Wraps the actual list containing ObjectItems (TUObjectArray)
pub struct ObjectArrayWrapper {
    array: *mut TUObjectArray,
}

impl ObjectArrayWrapper {
    pub unsafe fn new(array: *mut TUObjectArray) -> ObjectArrayWrapper {
        ObjectArrayWrapper { array }
    }
    pub fn max_elements(&self) -> usize {
        unsafe { (*self.array).max_elements.try_into().unwrap() }
    }
    pub fn num_elements(&self) -> usize {
        unsafe { (*self.array).num_elements.try_into().unwrap() }
    }
    pub fn iter_elements<'a>(&'a self) -> impl Iterator<Item = ObjectItemWrapper> + 'a {
        struct ObjectArrayIterator<'a> {
            array: &'a ObjectArrayWrapper,
            index: usize,
        }
        impl<'a> Iterator for ObjectArrayIterator<'a> {
            type Item = ObjectItemWrapper;

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
    pub fn get_object(&self, obj: &ObjectWrapper) -> ObjectItemWrapper {
        let item = self.get(obj.internal_index());
        assert_eq!(item.object().as_ptr() as usize, obj.as_ptr() as usize);
        item
    }
    pub fn get(&self, index: usize) -> ObjectItemWrapper {
        unsafe {
            assert!(index < self.num_elements(), "assert {} < {}", index, self.num_elements());
            let item = (*self.array).objects.offset(index.try_into().unwrap());
            ObjectItemWrapper::new(item)
        }
    }
}

pub struct ObjectItemWrapper {
    item: *mut FUObjectItem,
}

impl ObjectItemWrapper {
    pub unsafe fn new(item: *mut FUObjectItem) -> ObjectItemWrapper {
        ObjectItemWrapper { item }
    }
    pub fn as_ptr(&self) -> *mut FUObjectItem {
        self.item
    }

    pub fn object(&self) -> ObjectWrapper {
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
