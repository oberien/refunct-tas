use std::ptr;
use std::mem;

use native::FMemory;
use native::FNAME_FNAME;

#[derive(Debug)]
#[repr(C, packed)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct TArray<T> {
    ptr: *mut T,
    len: i32,
    capacity: i32,
}

impl<T> TArray<T> {
    pub fn new() -> TArray<T> {
        TArray::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> TArray<T> {
        let ptr = if cap > 0 {
            unsafe { FMemory::malloc(cap * mem::size_of::<T>()) as *mut T }
        } else {
            ptr::null_mut()
        };
        TArray {
            ptr,
            len: 0,
            capacity: cap as i32,
        }
    }

    pub fn push(&mut self, t: T) {
        assert!(self.len  < self.capacity);
        unsafe { *self.ptr.offset(self.len as isize) = t };
        self.len += 1;
    }
}

#[cfg(unix)]
pub type TCHAR = u32;
#[cfg(windows)]
pub type TCHAR = u16;

/// Null-terminated utf-32 (linux) / utf-16 (windows) array
#[derive(Debug)]
#[repr(transparent)]
pub struct FString(TArray<TCHAR>);

impl FString {
    pub fn new() -> FString {
        FString(TArray::new())
    }

    pub fn as_ptr(&self) -> *const TCHAR {
        self.0.ptr
    }
}

impl<S: AsRef<str>> From<S> for FString {
    fn from(s: S) -> Self {
        let s = s.as_ref();
        let len = s.chars().count();
        let mut arr = TArray::with_capacity(len + 1);
        for c in s.chars() {
            arr.push(c as TCHAR);
        }
        arr.push(0 as TCHAR);

        FString(arr)
    }
}

impl<T> Drop for TArray<T> {
    fn drop(&mut self) {
        unsafe {
            FMemory::free(self.ptr as *mut ())
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct FName {
    number: u64,
}

impl<T: Into<FString>> From<T> for FName {
    fn from(s: T) -> FName {
        let s = s.into();
        let mut name = FName {
            number: 0,
        };
        unsafe {
            let fun: extern "C" fn(*mut FName, *const TCHAR, u64) -> u64
                = mem::transmute(FNAME_FNAME);
            fun(&mut name as *mut FName, s.as_ptr(), 1);
        }
        name
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct FLinearColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl FLinearColor {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> FLinearColor {
        FLinearColor {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<(f32, f32, f32)> for FLinearColor {
    fn from(t: (f32, f32, f32)) -> FLinearColor {
        FLinearColor::new(t.0, t.1, t.2, 1.0)
    }
}

impl From<(f32, f32, f32, f32)> for FLinearColor {
    fn from(t: (f32, f32, f32, f32)) -> FLinearColor {
        FLinearColor::new(t.0, t.1, t.2, t.3)
    }
}
