use std::ptr;
use std::sync::atomic::Ordering;

#[cfg(unix)] use libc::{c_void, c_int};
#[cfg(windows)] use winapi::ctypes::{c_void, c_int};

use crate::native::ue::{FName, FVector, FRotator};
use crate::native::{APAWN_SPAWNDEFAULTCONTROLLER, GWORLD, UWORLD_SPAWNACTOR, UWORLD_DESTROYACTOR, AMyCharacter};
use crate::native::gameinstance::UMyGameInstance;

pub(in crate::native) type AActor = c_void;
pub enum APawn {}
pub(in crate::native) type ULevel = c_void;
pub(in crate::native) type UClass = c_void;

#[derive(Debug)]
#[repr(u8)]
#[allow(unused)]
enum ESpawnActorCollisionHandlingMethod {
    Undefined,
    AlwaysSpawn,
    AdjustIfPossibleButAlwaysSpawn,
    AdjustIfPossibleButDontSpawnIfColliding,
    DontSpawnIfColliding,
}

#[derive(Debug)]
#[repr(C)]
struct FActorSpawnParameters {
    name: FName,
    template: *const AActor,
    owner: *const AActor,
    instigator: *const APawn,
    override_level: *const ULevel,
    spawn_collision_handling_override: ESpawnActorCollisionHandlingMethod,
    // bRemoteOwned, bNoFail, bDeferConstruction, bAllowDuringConstructionScript
    some_flags: u16,
    object_flags: c_int,
}

impl Default for FActorSpawnParameters {
    fn default() -> Self {
        Self {
            name: FName::NAME_None,
            template: ptr::null(),
            owner: ptr::null(),
            instigator: ptr::null(),
            override_level: ptr::null(),
            spawn_collision_handling_override: ESpawnActorCollisionHandlingMethod::Undefined,
            some_flags: 0,
            object_flags: 0x00000008,
        }
    }
}

impl APawn {
    fn spawn_default_controller(this: *const APawn) {
        let fun: extern_fn!(fn(this: *const APawn))
            = unsafe { ::std::mem::transmute(APAWN_SPAWNDEFAULTCONTROLLER.load(Ordering::SeqCst)) };
        fun(this)
    }
}

#[repr(C)]
pub struct UWorld {
    #[cfg(windows)]
    pad: [u8; 0xc0],
    #[cfg(unix)]
    pad: [u8; 0x138],
    game_instance: *mut UMyGameInstance,
}

impl UWorld {
    unsafe fn spawn_actor(
        class: *const UClass, location: *const FVector, rotation: *const FRotator,
        spawn_parameters: *const FActorSpawnParameters,
    ) -> *mut AActor {
        let fun: extern_fn!(fn(
            this: *const UWorld, class: *const UClass, location: *const FVector,
            rotation: *const FRotator, spawn_parameters: *const FActorSpawnParameters
        ) -> *mut AActor) = ::std::mem::transmute(UWORLD_SPAWNACTOR.load(Ordering::SeqCst));
        let this = Self::get_global();
        fun(this, class, location, rotation, spawn_parameters)
    }
    unsafe fn destroy_actor(actor: *const AActor, net_force: bool, should_modify_level: bool) -> bool {
        let fun: extern_fn!(fn(
            this: *const UWorld, actor: *const AActor, net_force: bool, should_modify_level: bool
        ) -> c_int) = ::std::mem::transmute(UWORLD_DESTROYACTOR.load(Ordering::SeqCst));
        let this = Self::get_global();
        let res = fun(this, actor, net_force, should_modify_level);
        res != 0
    }

    pub(in crate::native) fn get_global() -> *mut UWorld {
        unsafe { *(GWORLD.load(Ordering::SeqCst) as *mut *mut UWorld)}
    }

    pub fn get_umygameinstance() -> *mut UMyGameInstance {
        unsafe {
            (*Self::get_global()).game_instance
        }
    }

    pub fn spawn_amycharacter() -> AMyCharacter {
        unsafe {
            let ptr = Self::spawn_actor(
                AMyCharacter::static_class(), &FVector { x: -500.0, y: -1125.0, z: 90.0 },
                &FRotator { pitch: 0.0, yaw: 0.0, roll: 0.0 }, &FActorSpawnParameters::default(),
            ) as *mut AMyCharacter;
            let my_character = AMyCharacter::new(ptr);
            APawn::spawn_default_controller(my_character.as_ptr() as *const APawn);
            my_character
        }
    }
    pub fn destroy_amycharaccter(my_character: AMyCharacter) {
        unsafe {
            let destroyed = Self::destroy_actor(my_character.as_ptr() as *const AActor, false, true);
            assert!(destroyed, "amycharacter not destroyed");
        }
    }
}