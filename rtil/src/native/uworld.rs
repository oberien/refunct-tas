use std::{mem, ptr};
use std::borrow::Borrow;
use std::cell::Cell;
use std::sync::{OnceLock, atomic::Ordering};
use crate::native::{Args, Level, LEVELS, LevelWrapper, LiftWrapper, ObjectIndex, ObjectWrapper, ObjectWrapperType, PipeWrapper, SpringpadWrapper, UeScope};

#[cfg(unix)] use libc::{c_void, c_int};
use serde::de::Unexpected::Option;
use serde_json::Value::Object;
#[cfg(windows)] use winapi::ctypes::{c_void, c_int};

use crate::native::ue::{FName, FVector, FRotator};
use crate::native::{APAWN_SPAWNDEFAULTCONTROLLER, AACTOR_SETACTORENABLECOLLISION, GWORLD, UWORLD_SPAWNACTOR, UWORLD_DESTROYACTOR, AMyCharacter, UGAMEPLAYSTATICS_GETACCURATEREALTIME, UMATERIALINSTANCEDYNAMIC_SETSCALARPARAMETERVALUE};
use crate::native::character::AMyCharacterUE;
use crate::native::gameinstance::UMyGameInstance;
use crate::native::reflection::{AActor, UClass, UObject};

pub enum APawn {}
pub enum UGameplayStatics {}
pub(in crate::native) type ULevel = c_void;

pub static CLOUDS_INDEX: OnceLock<ObjectIndex<ObjectWrapperType>> = OnceLock::new();
pub static JUMP6_INDEX: OnceLock<ObjectIndex<ObjectWrapperType>> = OnceLock::new();

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
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
#[allow(unused)]
enum ESpawnActorNameMode {
    RequiredFatal,
    RequiredErrorAndReturnNull,
    RequiredReturnNull,
    Requested,
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
    bitfield: u8,
    name_node: ESpawnActorNameMode,
    object_flags: c_int,
}
#[allow(unused)]
impl FActorSpawnParameters {
    const B_REMOTE_OWNED: u8 = 0b0000_0001;
    const B_NO_FAIL: u8 = 0b0000_0010;
    const B_DEFER_CONSTRUCTION: u8 = 0b0000_0100;
    const B_ALLOW_DURING_CONSTRUCTION_SCRIPT: u8 = 0b0000_1000;
}

impl APawn {
    fn spawn_default_controller(this: *const APawn) {
        let fun: extern_fn!(fn(this: *const APawn))
            = unsafe { ::std::mem::transmute(APAWN_SPAWNDEFAULTCONTROLLER.load(Ordering::SeqCst)) };
        fun(this)
    }
}
impl AActor {
    pub fn set_actor_enable_collision(this: *const AActor, enable: bool) {
        let fun: extern_fn!(fn(this: *const AActor, enable: u32))
            = unsafe { ::std::mem::transmute(AACTOR_SETACTORENABLECOLLISION.load(Ordering::SeqCst)) };
        fun(this, enable as u32)
    }
}

impl UGameplayStatics {
    pub fn get_accurate_real_time() -> f64 {
        let fun: extern "C" fn(world_context_object: *const UObject, seconds: *mut i32, partial_seconds: *mut f32)
            = unsafe { ::std::mem::transmute(UGAMEPLAYSTATICS_GETACCURATEREALTIME.load(Ordering::SeqCst)) };
        let my_character = AMyCharacter::get_player();
        let mut rt_seconds = 0_i32;
        let mut rt_partial_seconds = 0_f32;
        fun(&my_character as *const _ as *const _, &mut rt_seconds, &mut rt_partial_seconds);
        rt_seconds as f64 + rt_partial_seconds as f64
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

    pub fn get_global() -> *mut UWorld {
        unsafe { *(GWORLD.load(Ordering::SeqCst) as *mut *mut UWorld)}
    }

    pub fn get_umygameinstance() -> *mut UMyGameInstance {
        unsafe {
            (*Self::get_global()).game_instance
        }
    }

    pub fn spawn_amycharacter(x: f32, y: f32, z: f32, pitch: f32, yaw: f32, roll: f32) -> AMyCharacter {
        unsafe {
            let location = FVector { x, y, z };
            let rotation = FRotator { pitch, yaw, roll };
            let spawn_parameters = FActorSpawnParameters {
                name: FName::NAME_None,
                template: ptr::null(),
                owner: ptr::null(),
                instigator: ptr::null(),
                override_level: ptr::null(),
                spawn_collision_handling_override: ESpawnActorCollisionHandlingMethod::AlwaysSpawn,
                bitfield: FActorSpawnParameters::B_NO_FAIL,
                name_node: ESpawnActorNameMode::RequiredFatal,
                object_flags: 0x00000000,
            };
            let ptr = Self::spawn_actor(
                AMyCharacter::static_class(), &location, &rotation, &spawn_parameters,
            ) as *mut AMyCharacterUE;
            assert!(!ptr.is_null(), "UWorld::SpawnActor returned null");
            APawn::spawn_default_controller(ptr as *const APawn);
            AActor::set_actor_enable_collision(ptr as *const AActor, true);
            let my_character = AMyCharacter::new(ptr);
            my_character
        }
    }
    pub fn destroy_amycharaccter(my_character: AMyCharacter) {
        unsafe {
            let destroyed = Self::destroy_actor(my_character.as_ptr() as *const AActor, true, true);
            // assert!(destroyed, "amycharacter not destroyed");
            if !destroyed {
                log!("amycharacter {:p} not destroyed", my_character.as_ptr());
            }
        }
    }

    pub fn set_cloud_speed(speed: f32) {
        UeScope::with(|scope| {
            let object = scope.get(CLOUDS_INDEX.get().unwrap());
            let fun: extern_fn!(fn( this: *const c_void, name: FName, value: f32 ))
                = unsafe { mem::transmute(UMATERIALINSTANCEDYNAMIC_SETSCALARPARAMETERVALUE.load(Ordering::SeqCst)) };
            fun(object.as_ptr() as *const c_void, FName::from("Cloud speed"), speed);
        })
    }

    pub fn set_outro_time_dilation(dilation: f32) {
        UeScope::with(|scope| {
            let object = scope.get(JUMP6_INDEX.get().unwrap());
            object.get_field("OutroDilation").unwrap::<&Cell<f32>>().set(dilation);
        })
    }

    pub fn set_outro_dilated_duration(duration: f32) {
        UeScope::with(|scope| {
            let object = scope.get(JUMP6_INDEX.get().unwrap());
            object.get_field("OutroDilatedDuration").unwrap::<&Cell<f32>>().set(duration);
        })
    }
}

#[rtil_derive::hook_before(UUserWidget::AddToScreen)]
fn add_to_screen(_args: &mut Args) {
    crate::threads::ue::add_to_screen();
}

pub fn init() {
    UeScope::with(|scope| {
        for item in scope.iter_global_object_array() {
            let object = item.object();
            let name = object.name();
            let class_name = object.class().name();
            if class_name == "MaterialInstanceDynamic" && name != "Default__MaterialInstanceDynamic" {
                CLOUDS_INDEX.set(scope.object_index(&object)).unwrap();
            }
            if class_name == "jump6_C" && name != "Default__jump6_C" {
                JUMP6_INDEX.set(scope.object_index(&object)).unwrap();
            }
        }
    })
}