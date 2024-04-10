use std::cell::Cell;
use std::ffi::c_void;
use crate::native::ue::{FVector, FRotator, FString, UeU64};
use crate::native::{ActorWrapper, ObjectWrapper, StructValueWrapper, BoolValueWrapper, UObject, UeScope};
use crate::native::reflection::UClass;
use crate::native::uworld::{CHARACTER_INDEX, MOVEMENT_COMP_INDEX, PLAYER_CONTROLLER_INDEX};

#[derive(Debug, PartialEq, Eq)]
pub struct AMyCharacter(*mut AMyCharacterUE);

// WARNING: somewhat unsound as some functions on AMyCharacter can only be called from
// UE's update-loop thread. However, currently there's no way to ensure that it's constructed
// from that thread, so there's also no reason not to allow it to be sent between threads
unsafe impl Send for AMyCharacter {}

impl AMyCharacter {
    pub(in crate::native) fn static_class() -> *const UClass {
        UeScope::with(|scope| {
            scope.get(CHARACTER_INDEX.get().unwrap()).class().as_ptr()
        })
    }

    pub(crate) fn character() -> *mut UObject {
        UeScope::with(|scope| {
            let controller = scope.get(PLAYER_CONTROLLER_INDEX.get().unwrap());
            controller.get_field("Character").unwrap::<ObjectWrapper>().as_ptr()
        })
    }

    pub fn controller() -> *mut UObject {
        UeScope::with(|scope| {
            let controller = scope.get(PLAYER_CONTROLLER_INDEX.get().unwrap());
            controller.as_ptr()
        })
    }
    fn movement() -> *mut UObject {
        UeScope::with(|scope| {
            let movement = scope.get(MOVEMENT_COMP_INDEX.get().unwrap());
            movement.as_ptr()
        })
    }
    fn player_state() -> *mut APlayerState {
        let controller = unsafe { ObjectWrapper::new(AMyCharacter::controller()) };
        controller.get_field("PlayerState").unwrap::<ObjectWrapper>().as_ptr() as *mut APlayerState
    }

    pub unsafe fn new(ptr: *mut AMyCharacterUE) -> AMyCharacter {
        AMyCharacter(ptr)
    }

    pub fn as_ptr(&self) -> *mut AMyCharacterUE {
        self.0
    }

    pub fn location() -> (f32, f32, f32) {
        UeScope::with(|scope| {
            let character = scope.get(CHARACTER_INDEX.get().unwrap());
            let fun = character.class().find_function("K2_GetActorLocation").unwrap();
            let params = fun.create_argument_struct();
            unsafe {
                fun.call(character.as_ptr(), &params);
            }
            let loc = params.get_field("ReturnValue").unwrap::<StructValueWrapper>();
            (loc.get_field("X").unwrap::<f32>(), loc.get_field("Y").unwrap::<f32>(), loc.get_field("Z").unwrap::<f32>())
        })
    }
    pub fn set_location(x: f32, y: f32, z: f32) {
        UeScope::with(|scope| {
            let character = scope.get(CHARACTER_INDEX.get().unwrap());
            let fun = character.class().find_function("K2_SetActorLocation").unwrap();
            let params = fun.create_argument_struct();
            params.get_field("NewLocation").field("X").unwrap::<&Cell<f32>>().set(x);
            params.get_field("NewLocation").field("Y").unwrap::<&Cell<f32>>().set(y);
            params.get_field("NewLocation").field("Z").unwrap::<&Cell<f32>>().set(z);
            unsafe {
                fun.call(character.as_ptr(), &params);
            }
        })
    }
    pub fn velocity() -> (f32, f32, f32) {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        let vel = movement.get_field("Velocity").unwrap::<StructValueWrapper>();
        (vel.get_field("X").unwrap::<f32>(), vel.get_field("Y").unwrap::<f32>(), vel.get_field("Z").unwrap::<f32>())
    }
    pub fn set_velocity( x: f32, y: f32, z: f32) {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        let vel = movement.get_field("Velocity").unwrap::<StructValueWrapper>();
        vel.get_field("X").unwrap::<&Cell<f32>>().set(x);
        vel.get_field("Y").unwrap::<&Cell<f32>>().set(y);
        vel.get_field("Z").unwrap::<&Cell<f32>>().set(z);
    }
    pub fn acceleration() -> (f32, f32, f32) {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        let vel = movement.get_field("Acceleration").unwrap::<StructValueWrapper>();
        (vel.get_field("X").unwrap::<f32>(), vel.get_field("Y").unwrap::<f32>(), vel.get_field("Z").unwrap::<f32>())
    }
    pub fn set_acceleration(x: f32, y: f32, z: f32) {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        let vel = movement.get_field("Acceleration").unwrap::<StructValueWrapper>();
        vel.get_field("X").unwrap::<&Cell<f32>>().set(x);
        vel.get_field("Y").unwrap::<&Cell<f32>>().set(y);
        vel.get_field("Z").unwrap::<&Cell<f32>>().set(z);
    }
    pub fn rotation() -> (f32, f32, f32) {
        let controller = unsafe { ObjectWrapper::new(AMyCharacter::controller()) };
        let rot = controller.get_field("ControlRotation").unwrap::<StructValueWrapper>();
        (rot.get_field("Pitch").unwrap::<f32>(), rot.get_field("Yaw").unwrap::<f32>(), rot.get_field("Roll").unwrap::<f32>())
    }
    pub fn set_rotation(pitch: f32, yaw: f32, roll: f32) {
        let controller = unsafe { ObjectWrapper::new(AMyCharacter::controller()) };
        let rot = controller.get_field("ControlRotation").unwrap::<StructValueWrapper>();
        rot.get_field("Pitch").unwrap::<&Cell<f32>>().set(pitch);
        rot.get_field("Yaw").unwrap::<&Cell<f32>>().set(yaw);
        rot.get_field("Roll").unwrap::<&Cell<f32>>().set(roll);
    }

    pub fn get_player_name() -> String {
        unsafe { (*Self::player_state().as_ref().unwrap()).player_name.to_string_lossy() }
    }
    pub fn get_steamid() -> u64 {
        let ptr = unsafe { (*Self::player_state()).unique_id.unique_id };
        assert!(!ptr.is_null());
        unsafe { (*ptr).steamid.get() }
    }

    pub fn movement_mode() -> u8 {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        movement.get_field("MovementMode").unwrap::<u8>()
    }
    pub fn set_movement_mode(value: u8) {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        movement.get_field("MovementMode").unwrap::<&Cell<u8>>().set(value);
    }
    pub fn max_fly_speed() -> f32 {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        movement.get_field("MaxFlySpeed").unwrap::<f32>()
    }
    pub fn set_max_fly_speed(value: f32) {
        let movement = unsafe { ObjectWrapper::new(AMyCharacter::movement()) };
        movement.get_field("MaxFlySpeed").unwrap::<&Cell<f32>>().set(value);
    }

    pub fn get_viewport_size() -> (i32, i32) {
        let controller = unsafe { ObjectWrapper::new(AMyCharacter::controller()) };
        let fun = controller.class().find_function("GetViewportSize").unwrap();
        let params = fun.create_argument_struct();
        unsafe {
            fun.call(controller.as_ptr(), &params);
            (params.get_field("SizeX").unwrap::<i32>(), params.get_field("SizeY").unwrap::<i32>())
        }
    }
    pub fn exit_water() {
        unsafe {
            let obj = ObjectWrapper::new(AMyCharacter::character());
            let fun = obj.class().find_function("UnderwaterChanged").unwrap();
            let params = fun.create_argument_struct();
            params.get_field("Value").unwrap::<BoolValueWrapper>().set(false);
            fun.call(obj.as_ptr(), &params);
            let fun = obj.class().find_function("StopSwimEffect").unwrap();
            let params = fun.create_argument_struct();
            fun.call(obj.as_ptr(), &params);
            let fun = obj.class().find_function("StopFootstepEffect").unwrap();
            let params = fun.create_argument_struct();
            fun.call(obj.as_ptr(), &params);
        }
    }
}

#[repr(C)]
pub struct AMyCharacterUE {
    #[cfg(unix)] _pad: [u8; 0x168],
    #[cfg(windows)] _pad: [u8; 0x11c],
    root_component: *mut USceneComponent,
    #[cfg(unix)] _pad2: [u8; 0x250],
    #[cfg(windows)] _pad2: [u8; 0x1b4],
    controller: *mut APlayerController,
    #[cfg(unix)] _pad3: [u8; 0x28],
    #[cfg(windows)] _pad3: [u8; 0x24],
    movement: *mut UCharacterMovementComponent,
}

#[repr(C)]
pub(crate) struct USceneComponent {
    #[cfg(unix)] _pad: [u8; 0x1a0],
    #[cfg(windows)] _pad: [u8; 0x140],
    location: FVector,
}

impl USceneComponent {
    pub fn set_world_location_and_rotation(loc: FVector, rot: FRotator, object: &ActorWrapper) {
        let root_component: ObjectWrapper = object.get_field("RootComponent").unwrap();
        let set_world_location_and_rotation = root_component.class().find_function("K2_SetWorldLocationAndRotation").unwrap();
        let params = set_world_location_and_rotation.create_argument_struct();
        params.get_field("NewLocation").field("X").unwrap::<&Cell<f32>>().set(loc.x);
        params.get_field("NewLocation").field("Y").unwrap::<&Cell<f32>>().set(loc.y);
        params.get_field("NewLocation").field("Z").unwrap::<&Cell<f32>>().set(loc.z);
        params.get_field("NewRotation").field("Pitch").unwrap::<&Cell<f32>>().set(rot.pitch);
        params.get_field("NewRotation").field("Yaw").unwrap::<&Cell<f32>>().set(rot.yaw);
        params.get_field("NewRotation").field("Roll").unwrap::<&Cell<f32>>().set(rot.roll);
        params.get_field("bSweep").unwrap::<BoolValueWrapper>().set(false);
        params.get_field("bTeleport").unwrap::<BoolValueWrapper>().set(true);
        unsafe {
            set_world_location_and_rotation.call(root_component.as_ptr(), &params);
        }
    }

    pub fn set_world_scale(scale: FVector, object: &ActorWrapper) {
        let root_component: ObjectWrapper = object.get_field("RootComponent").unwrap();
        let set_world_scale = root_component.class().find_function("SetWorldScale3D").unwrap();
        let params = set_world_scale.create_argument_struct();
        params.get_field("NewScale").field("X").unwrap::<&Cell<f32>>().set(scale.x);
        params.get_field("NewScale").field("Y").unwrap::<&Cell<f32>>().set(scale.y);
        params.get_field("NewScale").field("Z").unwrap::<&Cell<f32>>().set(scale.z);
        unsafe { set_world_scale.call(root_component.as_ptr(), &params); }
    }
}

#[repr(C)]
struct UCharacterMovementComponent {
    #[cfg(unix)] _pad: [u8; 0x104],
    #[cfg(windows)] _pad: [u8; 0xb4],
    velocity: FVector,
    #[cfg(unix)] _pad2: [u8; 0x90],
    #[cfg(windows)] _pad2: [u8; 0x7c],
    movement_mode: u8,
    #[cfg(unix)] _pad3: [u8; 0x2c],
    #[cfg(windows)] _pad3: [u8; 0x31],
    max_fly_speed: f32,
    #[cfg(unix)] _pad4: [u8; 0x9c],
    #[cfg(windows)] _pad4: [u8; 0x98],
    acceleration: FVector,
}

#[repr(C)]
struct APlayerController {
    #[cfg(unix)] _pad: [u8; 0x3a8],
    #[cfg(windows)] _pad: [u8; 0x2c8],
    player_state: *mut APlayerState,
    _pad2: *mut c_void,
    rotation: FRotator,
}

#[repr(C)]
struct APlayerState {
    #[cfg(unix)] _pad: [u8; 0x390],
    #[cfg(windows)] _pad: [u8; 0x2bc],
    player_name: FString,
    old_name: FString,
    #[cfg(unix)] _pad2: [u8; 0x30],
    #[cfg(windows)] _pad2: [u8; 0x20],
    unique_id: FUniqueNetIdRepl,
}

struct FUniqueNetIdRepl {
    _vtable: *const c_void,
    unique_id: *const FUniqueNetIdSteam,
}

#[repr(C)]
struct FUniqueNetIdSteam {
    _vtable: *const c_void,
    _self: *const c_void,
    _shared_ptr: *const c_void,
    steamid: UeU64,
}
