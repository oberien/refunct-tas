use std::{mem, ptr, slice};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;
use image::RgbaImage;
use crate::native::{FUNTYPEDBULKDATA_LOCK, FUNTYPEDBULKDATA_UNLOCK, UTEXTURE2D_CREATETRANSIENT, UTEXTURE2D_GETRUNNINGPLATFORMDATA, UTEXTURE2D_UPDATERESOURCE};
use crate::native::reflection::{GlobalObjectArrayWrapper, ObjectWrapper, UObject};
use crate::native::ue::TArray;

pub struct UTexture2D(*mut UTexture2DUE);
pub(in crate::native) enum UTexture2DUE {}

// WARNING: somewhat unsound - see AMyCharacter
unsafe impl Send for UTexture2D {}

impl UTexture2D {
    fn as_object(&self) -> ObjectWrapper<'_> {
        unsafe { ObjectWrapper::new(self.0 as *mut UObject) }
    }
    fn create_transient(width: i32, height: i32, format: EPixelFormat) -> *mut UTexture2DUE {
        let fun: extern "C" fn(
            in_size_x: i32, in_size_y: i32, in_format: EPixelFormat
        ) -> *mut UTexture2DUE = unsafe { mem::transmute(UTEXTURE2D_CREATETRANSIENT.load(Ordering::SeqCst)) };
        fun(width, height, format)
    }

    fn get_running_platform_data(&self) -> *mut *mut FTexturePlatformData {
        let fun: extern_fn!(fn(
            this: *mut UTexture2DUE
        ) -> *mut *mut FTexturePlatformData) = unsafe { mem::transmute(UTEXTURE2D_GETRUNNINGPLATFORMDATA.load(Ordering::SeqCst)) };
        fun(self.0)
    }

    fn update_resource(texture: *mut UTexture2DUE) {
        let fun: extern_fn!(fn(
            this: *mut UTexture2DUE
        )) = unsafe { mem::transmute(UTEXTURE2D_UPDATERESOURCE.load(Ordering::SeqCst)) };
        fun(texture)
    }

    pub fn width(&self) -> i32 {
        unsafe { (**self.get_running_platform_data()).size_x }
    }
    pub fn height(&self) -> i32 {
        unsafe { (**self.get_running_platform_data()).size_y }
    }

    pub(in crate::native) fn as_ptr(&self) -> *mut UTexture2DUE {
        self.0
    }

    pub fn set_image(&mut self, image: &RgbaImage) {
        assert_eq!(self.width() as u32, image.width());
        assert_eq!(self.height() as u32, image.height());
        self.set_image_raw(image)
    }
    pub fn set_image_raw(&mut self, image: &[u8]) {
        self.as_mut_slice().copy_from_slice(image);
    }

    pub fn create(image: &RgbaImage) -> UTexture2D {
        Self::create_with_pixelformat(image, image.width().try_into().unwrap(), image.height().try_into().unwrap(), EPixelFormat::R8G8B8A8)
    }
    pub fn create_with_pixelformat(image: &[u8], width: i32, height: i32, pixel_format: EPixelFormat) -> UTexture2D {
        let texture = UTexture2D::create_transient(width, height, pixel_format);
        log!("texture: {:p}", texture);
        let mut texture = UTexture2D(texture);
        texture.set_image_raw(image);
        // mark texture as root-object to not be cleaned by the GC
        texture.mark_as_root_object(true);
        texture
    }

    pub fn as_mut_slice(&mut self) -> UTexture2DSliceMut<'_> {
        unsafe {
            let platform_data = self.get_running_platform_data();
            let mip_map = (&(**platform_data).mips)[0];
            let bulk_data = ptr::addr_of_mut!((*mip_map).bulk_data) as *mut FByteBulkData;
            let ptr = FByteBulkData::lock(bulk_data, EBulkDataLockFlags::LockReadWrite);
            let len = self.width() as usize * self.height() as usize * 4;
            let slice = slice::from_raw_parts_mut(ptr, len);
            UTexture2DSliceMut {
                texture: self.0,
                slice,
                bulk_data,
            }
        }
    }

    fn mark_as_root_object(&self, val: bool) {
        unsafe { GlobalObjectArrayWrapper::get().object_array().get_item_of_object(&self.as_object()).mark_as_root_object(val) }
    }
}

impl Drop for UTexture2D {
    fn drop(&mut self) {
        // mark texture as non-root-object to be cleaned by the GC
        self.mark_as_root_object(false)
    }
}

pub struct UTexture2DSliceMut<'a> {
    texture: *mut UTexture2DUE,
    slice: &'a mut [u8],
    bulk_data: *mut FByteBulkData,
}
impl<'a> Deref for UTexture2DSliceMut<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}
impl<'a> DerefMut for UTexture2DSliceMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice
    }
}
impl<'a> Drop for UTexture2DSliceMut<'a> {
    fn drop(&mut self) {
        unsafe {
            FByteBulkData::unlock(self.bulk_data);
            UTexture2D::update_resource(self.texture);
        }
    }
}

#[repr(C)]
struct FTexturePlatformData {
    size_x: i32,
    size_y: i32,
    packed_data: u32,
    pixel_format: EPixelFormat,
    mips: TArray<*mut FTexture2DMipMap>,
}
#[repr(C)]
struct FTexture2DMipMap {
    size_x: i32,
    size_y: i32,
    bulk_data: FByteBulkData,
}
#[repr(C)]
struct FByteBulkData {
    // stub, we only need a pointer to this struct
}
impl FByteBulkData {
    unsafe fn lock(this: *mut FByteBulkData, mode: EBulkDataLockFlags) -> *mut u8 {
        let fun: extern_fn!(fn(
            this: *mut FByteBulkData, mode: EBulkDataLockFlags
        ) -> *mut u8) = unsafe { mem::transmute(FUNTYPEDBULKDATA_LOCK.load(Ordering::SeqCst)) };
        fun(this, mode)
    }
    unsafe fn unlock(this: *mut FByteBulkData) {
        let fun: extern_fn!(fn(
            this: *mut FByteBulkData
        )) = unsafe { mem::transmute(FUNTYPEDBULKDATA_UNLOCK.load(Ordering::SeqCst)) };
        fun(this)
    }
}

// enums

#[allow(unused)]
#[repr(i32)]
enum EBulkDataLockFlags {
    LockReadOnly = 1,
    LockReadWrite = 2,
}

#[allow(unused)]
#[repr(u32)]
#[derive(Clone, Copy)]
enum EDerivedDataFlags {
    None = 0,
    Required = 1 << 0,
    Optional = 1 << 1,
    MemoryMapped = 1 << 2,
}

#[allow(non_camel_case_types, unused)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum EPixelFormat {
    Unknown               =0,
    A32B32G32R32F         =1,
    B8G8R8A8              =2,
    G8                    =3,
    G16                   =4,
    DXT1                  =5,
    DXT3                  =6,
    DXT5                  =7,
    UYVY                  =8,
    FloatRGB              =9,
    FloatRGBA             =10,
    DepthStencil          =11,
    ShadowDepth           =12,
    R32_FLOAT             =13,
    G16R16                =14,
    G16R16F               =15,
    G16R16F_FILTER        =16,
    G32R32F               =17,
    A2B10G10R10           =18,
    A16B16G16R16          =19,
    D24                   =20,
    R16F                  =21,
    R16F_FILTER           =22,
    BC5                   =23,
    V8U8                  =24,
    A1                    =25,
    FloatR11G11B10        =26,
    A8                    =27,
    R32_UINT              =28,
    R32_SINT              =29,
    PVRTC2                =30,
    PVRTC4                =31,
    R16_UINT              =32,
    R16_SINT              =33,
    R16G16B16A16_UINT     =34,
    R16G16B16A16_SINT     =35,
    R5G6B5_UNORM          =36,
    R8G8B8A8              =37,
    A8R8G8B8              =38,
    BC4                   =39,
    R8G8                  =40,
    ATC_RGB               =41,
    ATC_RGBA_E            =42,
    ATC_RGBA_I            =43,
    X24_G8                =44,
    ETC1                  =45,
    ETC2_RGB              =46,
    ETC2_RGBA             =47,
    R32G32B32A32_UINT     =48,
    R16G16_UINT           =49,
    ASTC_4x4              =50,
    ASTC_6x6              =51,
    ASTC_8x8              =52,
    ASTC_10x10            =53,
    ASTC_12x12            =54,
    BC6H                  =55,
    BC7                   =56,
    R8_UINT               =57,
    L8                    =58,
    XGXR8                 =59,
    R8G8B8A8_UINT         =60,
    R8G8B8A8_SNORM        =61,
    R16G16B16A16_UNORM    =62,
    R16G16B16A16_SNORM    =63,
    PLATFORM_HDR_0        =64,
    PLATFORM_HDR_1        =65,
    PLATFORM_HDR_2        =66,
    NV12                  =67,
    R32G32_UINT           =68,
    ETC2_R11_EAC          =69,
    ETC2_RG11_EAC         =70,
    PF_R8                 =71,
    PF_B5G5R5A1_UNORM     =72,
    PF_ASTC_4x4_HDR       =73,
    PF_ASTC_6x6_HDR       =74,
    PF_ASTC_8x8_HDR       =75,
    PF_ASTC_10x10_HDR     =76,
    PF_ASTC_12x12_HDR     =77,
    PF_G16R16_SNORM       =78,
    PF_R8G8_UINT          =79,
    PF_R32G32B32_UINT     =80,
    PF_R32G32B32_SINT     =81,
    PF_R32G32B32F         =82,
    PF_R8_SINT            =83,
    PF_R64_UINT           =84,
    PF_R9G9B9EXP5         =85,
    PF_MAX                =86,
}
