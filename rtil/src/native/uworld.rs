use std::ptr;

use libc::{c_void, c_int};

use native::ue::{FName, FVector, FRotator};
use native::{APAWN_STATICCLASS, APAWN_SPAWNDEFAULTCONTROLLER, GWORLD, UWORLD_SPAWNACTOR, UWORLD_DESTROYACTOR};

pub(in native) type AActor = c_void;
pub enum APawn {}
pub(in native) type ULevel = c_void;
pub(in native) type UClass = c_void;

#[derive(Debug)]
#[repr(u8)]
pub(in native) enum ESpawnActorCollisionHandlingMethod {
	Undefined,
	AlwaysSpawn,
	AdjustIfPossibleButAlwaysSpawn,
	AdjustIfPossibleButDontSpawnIfColliding,
	DontSpawnIfColliding,
}

#[derive(Debug)]
#[repr(C, packed)]
pub(in native) struct FActorSpawnParameters {
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
    fn static_class() -> *const UClass {
        let fun: extern_fn!(fn() -> *const UClass)
            = unsafe { ::std::mem::transmute(APAWN_STATICCLASS) };
        fun()
    }
    fn spawn_default_controller(this: *const APawn) {
        let fun: extern_fn!(fn(this: *const APawn))
            = unsafe { ::std::mem::transmute(APAWN_SPAWNDEFAULTCONTROLLER) };
        fun(this)
    }
}

pub struct UWorld;

impl UWorld {
    fn spawn_actor(
        class: *const UClass, location: *const FVector, rotation: *const FRotator,
        spawn_parameters: *const FActorSpawnParameters,
    ) -> *mut AActor {
        let fun: extern_fn!(fn(
            this: *const UWorld, class: *const UClass, location: *const FVector,
            rotation: *const FRotator, spawn_parameters: *const FActorSpawnParameters
        ) -> *mut AActor) = unsafe { ::std::mem::transmute(UWORLD_SPAWNACTOR) };
        let this = unsafe { *(GWORLD as *const _)};
        fun(this, class, location, rotation, spawn_parameters)
    }
    fn destroy_actor( actor: *const AActor, net_force: bool, should_modify_level: bool ) -> bool {
        let fun: extern_fn!(fn(
            this: *const UWorld, actor: *const AActor, net_force: bool, should_modify_level: bool
        ) -> c_int) = unsafe { ::std::mem::transmute(UWORLD_DESTROYACTOR) };
        let this = unsafe { *(GWORLD as *const _)};
        let res = fun(this, actor, net_force, should_modify_level);
        res != 0
    }

    pub fn spawn_pawn() -> *const APawn {
        let pawn = Self::spawn_actor(
            APawn::static_class(), &FVector { x: -1000.0, y: -1000.0, z: 732.0 },
            &FRotator { pitch: 0.0, yaw: 0.0, roll: 0.0}, &FActorSpawnParameters::default(),
        ) as *const APawn;
        APawn::spawn_default_controller(pawn);
        pawn
    }
    pub fn destroy_pawn(pawn: *const APawn) {
        let destroyed = Self::destroy_actor(pawn as *const AActor, false, true);
        assert!(destroyed, "pawn not destroyed");
    }
}