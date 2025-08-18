use std::{mem, ptr, cell::Cell};
use std::sync::{OnceLock, atomic::Ordering};
use crate::native::{ArrayWrapper, BoolValueWrapper, ObjectIndex, ObjectWrapper, ObjectWrapperType, StructValueWrapper, UeScope};

#[cfg(unix)] use libc::{c_void, c_int};
use hook::{ArgsRef, IsaAbi, RawHook};
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
pub static ENGINE_INDEX: OnceLock<ObjectIndex<ObjectWrapperType>> = OnceLock::new();
pub static CAMERA_INDEX: OnceLock<ObjectIndex<ObjectWrapperType>> = OnceLock::new();

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

#[derive(rebo::ExternalType)]
pub enum TimeOfDay {
    Day,
    Night,
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

    pub fn set_sun_redness(redness: f32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let keys = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("SunColor")
            .field("FloatCurves")
            .field("Keys")
            .unwrap::<ArrayWrapper<StructValueWrapper>>();
        for key in keys.into_iter() {
            key.get_field("Value").unwrap::<&Cell<f32>>().set(redness);
        }
    }

    pub fn set_cloud_redness(red: f32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let keys = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("CloudColor")
            .field("FloatCurves")
            .field("Keys")
            .unwrap::<ArrayWrapper<StructValueWrapper>>();
        for key in keys.into_iter() {
            key.get_field("Value").unwrap::<&Cell<f32>>().set(red);
        }
    }
    pub fn set_stars_brightness(time: TimeOfDay, brightness: f32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let keys = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("StarsBrightness")
            .field("FloatCurve")
            .field("Keys")
            .unwrap::<ArrayWrapper<StructValueWrapper>>();
        match time {
            TimeOfDay::Day => keys.get(0).unwrap().get_field("Value").unwrap::<&Cell<f32>>().set(brightness),
            TimeOfDay::Night => keys.get(1).unwrap().get_field("Value").unwrap::<&Cell<f32>>().set(brightness),
        }
    }

    pub fn set_sky_light_intensity(intensity: f32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let light_component = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("Light")
            .field("LightComponent")
            .unwrap::<ObjectWrapper>();
        let fun = light_component.class().find_function("SetIntensity").unwrap();
        let params = fun.create_argument_struct();
        params.get_field("NewIntensity").unwrap::<&Cell<f32>>().set(intensity);
        unsafe {
            fun.call(light_component.as_ptr(), &params);
        }
    }
    pub fn set_sky_light_brightness(brightness: f32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let light = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("Light")
            .unwrap::<ObjectWrapper>();
        let fun = light.class().find_function("SetBrightness").unwrap();
        let params = fun.create_argument_struct();
        params.get_field("NewBrightness").unwrap::<&Cell<f32>>().set(brightness);
        unsafe {
            fun.call(light.as_ptr(), &params);
        }
    }
    pub fn set_sky_time_speed(speed: f32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("TimeSpeed")
            .unwrap::<&Cell<f32>>().set(speed);
    }
    pub fn set_sky_light_enabled(enabled: bool) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let light_component  = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("Light")
            .unwrap::<ObjectWrapper>();
        let fun = light_component.class().find_function("SetEnabled").unwrap();
        let params = fun.create_argument_struct();
        params.get_field("bSetEnabled").unwrap::<BoolValueWrapper>().set(enabled);
        unsafe {
            fun.call(light_component.as_ptr(), &params);
        }
    }
    pub fn set_lighting_casts_shadows(value: bool) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let light_component = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("Light")
            .field("LightComponent")
            .unwrap::<ObjectWrapper>();
        let fun = light_component.class().find_function("SetCastShadows").unwrap();
        let params = fun.create_argument_struct();
        params.get_field("bNewValue").unwrap::<BoolValueWrapper>().set(value);
        unsafe {
            fun.call(light_component.as_ptr(), &params);
        }
    }

    pub fn set_time_dilation(dilation: f32) {
        let obj = unsafe { ObjectWrapper::new(UWorld::get_global() as *mut UObject) };
        obj.get_field("PersistentLevel")
            .field("WorldSettings")
            .field("TimeDilation")
            .unwrap::<&Cell<f32>>()
            .set(dilation);
    }

    pub fn set_gravity(gravity: f32) {
        let obj = unsafe { ObjectWrapper::new(UWorld::get_global() as *mut UObject) };
        let world_settings = obj.get_field("PersistentLevel")
            .field("WorldSettings").unwrap::<ObjectWrapper>();
        world_settings.get_field("bWorldGravitySet").unwrap::<BoolValueWrapper>().set(true);
        world_settings.get_field("WorldGravityZ").unwrap::<&Cell<f32>>().set(gravity);
    }

    pub fn get_time_of_day() -> f32 {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("CurrentMinute")
            .unwrap::<f32>()
    }

    pub fn set_time_of_day(time: f32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("CurrentMinute")
            .unwrap::<&Cell<f32>>()
            .set(time);
    }

    pub fn set_cloud_speed(speed: f32) {
        UeScope::with(|scope| {
            let object = scope.get(CLOUDS_INDEX.get().unwrap());
            let fun: extern_fn!(fn(this: *const c_void, name: FName, value: f32))
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

    pub fn set_kill_z(kill_z: f32) {
        let obj = unsafe { ObjectWrapper::new(UWorld::get_global() as *mut UObject) };
        obj.get_field("PersistentLevel")
            .field("WorldSettings")
            .field("KillZ")
            .unwrap::<&Cell<f32>>()
            .set(kill_z);
    }

    pub fn set_reflection_render_scale(render_scale: i32) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        obj.get_field("WorldReferences")
            .field("Water")
            .field("PlanarReflectionComponent")
            .field("ScreenPercentage")
            .unwrap::<&Cell<i32>>()
            .set(render_scale);
    }

    pub fn set_fog_enabled(enabled: bool) {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let component = obj.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("Fog")
            .field("Component")
            .unwrap::<ObjectWrapper>();
        let new_value = if enabled { 0.05 } else { 0. };
        component.get_field("FogDensity").unwrap::<&Cell<f32>>().set(new_value);
        component.get_field("DirectionalInscatteringExponent").unwrap::<&Cell<f32>>().set(1.);
    }

    pub fn set_screen_percentage(percentage: f32) {
        let game_instance = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let volumes = game_instance.get_field("WorldReferences")
            .field("TimeOfDay")
            .field("Volumes")
            .unwrap::<ArrayWrapper<ObjectWrapper>>();
        for volume in volumes.into_iter() {
            let settings = volume.get_field("Settings").unwrap::<StructValueWrapper>();
            settings.get_field("bOverride_ScreenPercentage").unwrap::<BoolValueWrapper>().set(true);
            settings.get_field("ScreenPercentage").unwrap::<&Cell<f32>>().set(percentage);
        }
    }
}

/// TODO: Create a new `ObjectWrapper` from the `rdi` register of `_args` and get the name, check it is the menu widget,
/// and only execute the function if it is.
pub fn add_to_screen_hook<IA: IsaAbi>(hook: &RawHook<IA, ()>, args: ArgsRef<'_, IA>) {
    crate::threads::ue::add_to_screen();
    unsafe { hook.call_original_function(args); }
}

pub fn init() {
    UeScope::with(|scope| {
        for item in scope.iter_global_object_array() {
            let object = item.object();
            let name = object.name();
            let class_name = object.class().name();
            if class_name == "MaterialInstanceDynamic" && name != "Default__MaterialInstanceDynamic" {
                CLOUDS_INDEX.set(scope.object_index(&object)).ok().unwrap();
            }
            if class_name == "jump6_C" && name != "Default__jump6_C" {
                JUMP6_INDEX.set(scope.object_index(&object)).ok().unwrap();
            }
            if class_name == "GameEngine" && name != "Default__GameEngine" {
                ENGINE_INDEX.set(scope.object_index(&object)).ok().unwrap();
            }
            if class_name == "CameraComponent" && name == "FirstPersonCamera" {
                let fun = object.class().find_function("GetOwner").unwrap();
                let params = fun.create_argument_struct();
                unsafe {
                    fun.call(object.as_ptr(), &params);
                    let owner = params.get_field("ReturnValue").unwrap::<ObjectWrapper>();
                    if owner.name() != "Default__MyCharacter" && owner.name() != "Default__BP_MyCharacter_C" {
                        CAMERA_INDEX.set(scope.object_index(&object)).ok().unwrap();
                    }
                }
            }
        }
    })
}
