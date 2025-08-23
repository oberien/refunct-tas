use std::{ptr, slice};
use std::mem;
use std::ops::{Index, IndexMut};
use std::sync::atomic::Ordering;

#[cfg(unix)] use libc::c_void;
#[cfg(windows)] use winapi::ctypes::c_void;

use crate::native::{FMemory, FNAME_FNAME};
use crate::native::FNAME_APPENDSTRING;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FVector2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct FRotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct TArray<T> {
    pub ptr: *mut T,
    pub len: i32,
    pub capacity: i32,
}

impl<T> TArray<T> {
    #[allow(unused)]
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

    pub fn len(&self) -> usize {
        assert!(self.len >= 0);
        self.len as usize
    }
    pub fn capacity(&self) -> usize {
        assert!(self.capacity >= 0);
        self.capacity as usize
    }

    pub fn push(&mut self, t: T) {
        assert!(self.len  < self.capacity);
        unsafe { *self.ptr.offset(self.len as isize) = t };
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        unsafe { Some(&*self.ptr.offset(self.check_index_for_indexing(index).ok()?)) }
    }

    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        unsafe { Some(&mut *self.ptr.offset(self.check_index_for_indexing(index).ok()?)) }
    }

    pub fn check_index_for_indexing(&self, index: usize) -> Result<isize, String> {
        assert!(mem::size_of::<usize>() >= mem::size_of::<i32>());
        if !(index <= i32::MAX as usize) {
            return Err("index must be smaller than i32::MAX".to_string());
        }
        if !((index as i32) < self.len) {
            return Err(format!("tried to access element {} of len {}", index, self.len));
        }
        let index = index as isize;
        assert!(index >= 0, "somehow {index}, which is smaller than i32::MAX, is between isize::MAX and usize::MAX");
        Ok(index)
    }
}

impl<T> Index<usize> for TArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.ptr.offset(self.check_index_for_indexing(index).unwrap()) }
    }
}
impl<T> IndexMut<usize> for TArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.ptr.offset(self.check_index_for_indexing(index).unwrap()) }
    }
}

impl<'a, T> IntoIterator for &'a TArray<T> {
    type Item = &'a T;
    type IntoIter = TArrayIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TArrayIterator {
            array: self,
            index: 0,
        }
    }
}

pub struct TArrayIterator<'a, T> {
    array: &'a TArray<T>,
    index: usize,
}

impl<'a, T> Iterator for TArrayIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(&self.array[index])
    }
}

impl<T> Drop for TArray<T> {
    fn drop(&mut self) {
        unsafe {
            FMemory::free(self.ptr as *mut c_void)
        }
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
    #[allow(unused)]
    pub fn new() -> FString {
        FString(TArray::new())
    }

    pub fn as_ptr(&self) -> *const TCHAR {
        self.0.ptr
    }

    pub fn to_string_lossy(&self) -> String {
        unsafe {
            assert!(self.0.len >= 0);
            assert!(mem::size_of::<isize>() >= mem::size_of::<i32>());
            if self.0.len == 0 {
                return String::new();
            }
            assert!(mem::size_of::<usize>() >= mem::size_of::<i32>());
            // check zero-termination
            assert_eq!(self.0[(self.0.len - 1) as usize], 0);

            #[cfg(windows)] {
                String::from_utf16_lossy(slice::from_raw_parts(self.0.ptr as *const TCHAR, (self.0.len - 1) as usize))
            }
            #[cfg(unix)] {
                assert_eq!(mem::size_of::<TCHAR>(), mem::size_of::<char>());
                 slice::from_raw_parts(self.0.ptr as *const TCHAR, (self.0.len - 1) as usize)
                    .iter().copied()
                    .map(|c| char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER))
                    .collect()
            }
        }
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

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FName {
    #[allow(unused)]
    pub number: UeU64,
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum EFindName {
    #[expect(unused)]
    FNAME_FIND,
    FNAME_ADD,
}

impl FName {
    #[allow(non_upper_case_globals)]
    pub const NAME_None: FName = FName { number: UeU64::new(0) };

    pub fn append_string(mut self, out: &mut FString) {
        unsafe {
            let fun: extern_fn!(fn(this: *mut FName, out: *mut FString))
                = mem::transmute(FNAME_APPENDSTRING.load(Ordering::SeqCst));
            fun(&mut self, out);
        }
    }

    pub fn to_string_lossy(self) -> String {
        let mut string = FString::new();
        self.append_string(&mut string);
        string.to_string_lossy()
    }
}

impl<T: Into<FString>> From<T> for FName {
    fn from(s: T) -> FName {
        let s = s.into();
        let mut name = FName::NAME_None;
        unsafe {
            let fun: extern_fn!(fn(this: *mut FName, name: *const TCHAR, find_type: EFindName) -> u64)
                = mem::transmute(FNAME_FNAME.load(Ordering::SeqCst));
            fun(&mut name as *mut FName, s.as_ptr(), EFindName::FNAME_ADD);
        }
        name
    }
}

#[derive(Debug)]
#[repr(C)]
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

#[derive(Debug)]
#[repr(C)]
#[expect(unused)]
pub struct FColor {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<(u8, u8, u8)> for FColor {
    fn from((red, green, blue): (u8, u8, u8)) -> Self {
        FColor {
            alpha: 255,
            red,
            green,
            blue
        }
    }
}

impl From<(u8, u8, u8, u8)> for FColor {
    fn from((alpha, red, green, blue): (u8, u8, u8, u8)) -> Self {
        FColor { alpha, red, green, blue }
    }
}

#[expect(unused)]
#[derive(Debug)]
#[repr(C)]
pub struct FQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// fuck msvc, fuck UE, fuck rust, fuck the rust reference
// https://github.com/rust-lang/reference/issues/1235
macro_rules! fuck_alignment {
    ($($name:ident, $t:ty;)*) => {
        $(
            #[cfg_attr(target_os = "linux", repr(align(8)))]
            #[cfg_attr(target_os = "windows", repr(align(4)))]
            #[repr(C)]
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
            pub struct $name {
                val: [u8; 8],
            }
            #[allow(unused)]
            impl $name {
                pub const fn new(val: $t) -> Self {
                    Self { val: val.to_ne_bytes() }
                }
                pub fn get(&self) -> $t {
                    <$t>::from_ne_bytes(self.val.clone())
                }
                pub fn set(&mut self, val: $t) {
                    self.val = val.to_ne_bytes();
                }
            }
        )*
    }
}
fuck_alignment! {
    UeU64, u64;
    UeI64, i64;
    UeF64, f64;
}
