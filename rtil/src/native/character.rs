use std::cell::Cell;
use std::ffi::c_void;
use std::mem;
use std::sync::atomic::{AtomicPtr, Ordering};
use crate::native::ue::{FVector, FRotator, FString, UeU64};
use crate::native::{AMYCHARACTER_STATICCLASS, Args, REBO_DOESNT_START_SEMAPHORE, APLAYERCONTROLLER_GETVIEWPORTSIZE, ActorWrapper, ObjectWrapper, StructValueWrapper, BoolValueWrapper, AMYCHARACTER_UNDERWATERCHANGED};
use crate::native::reflection::UClass;

static CURRENT_PLAYER: AtomicPtr<AMyCharacterUE> = AtomicPtr::new(std::ptr::null_mut());

#[derive(Debug, PartialEq, Eq)]
pub struct AMyCharacter(*mut AMyCharacterUE);

// WARNING: somewhat unsound as some functions on AMyCharacter can only be called from
// UE's update-loop thread. However, currently there's no way to ensure that it's constructed
// from that thread, so there's also no reason not to allow it to be sent between threads
unsafe impl Send for AMyCharacter {}

impl AMyCharacter {
    pub(in crate::native) fn static_class() -> *const UClass {
        let fun: extern "C" fn() -> *const UClass
            = unsafe { ::std::mem::transmute(AMYCHARACTER_STATICCLASS.load(Ordering::SeqCst)) };
        fun()
    }
    fn root_component(&self) -> *mut USceneComponent {
        unsafe { (*self.0).root_component }
    }
    fn controller(&self) -> *mut APlayerController {
        unsafe { (*self.0).controller }
    }
    fn movement(&self) -> *mut UCharacterMovementComponent {
        unsafe { (*self.0).movement }
    }
    fn player_state(&self) -> *mut APlayerState {
        unsafe { (*self.controller()).player_state }
    }

    pub unsafe fn new(ptr: *mut AMyCharacterUE) -> AMyCharacter {
        AMyCharacter(ptr)
    }

    pub fn as_ptr(&self) -> *mut AMyCharacterUE {
        self.0
    }

    pub fn get_player() -> AMyCharacter {
        let current_player = CURRENT_PLAYER.load(Ordering::SeqCst);
        if current_player.is_null() {
            let msg = concat!("called AMyCharacter::get_player while current player's AMyCharacter-pointer wasn't initialized yet");
            log!("{}", msg);
            panic!("{}", msg);
        }
        AMyCharacter(current_player)
    }

    pub fn location(&self) -> (f32, f32, f32) {
        let FVector { x, y, z } = unsafe { (*self.root_component()).location };
        (x, y, z)
    }
    pub fn set_location(&mut self, x: f32, y: f32, z: f32) {
        unsafe { (*self.root_component()).location = FVector { x, y, z } };
    }
    pub fn velocity(&self) -> (f32, f32, f32) {
        let FVector { x, y, z } = unsafe { (*self.movement()).velocity };
        (x, y, z)
    }
    pub fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        unsafe { (*self.movement()).velocity = FVector { x, y, z } };
    }
    pub fn acceleration(&self) -> (f32, f32, f32) {
        let FVector { x, y, z } = unsafe { (*self.movement()).acceleration };
        (x, y, z)
    }
    pub fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        unsafe { (*self.movement()).acceleration = FVector { x, y, z } };
    }
    pub fn rotation(&self) -> (f32, f32, f32) {
        let FRotator { pitch, yaw, roll } = unsafe { (*self.controller()).rotation };
        (pitch, yaw, roll)
    }
    pub fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) {
        unsafe { (*self.controller()).rotation = FRotator { pitch, yaw, roll } };
    }

    pub fn get_player_name(&self) -> String {
        unsafe { (*self.player_state()).player_name.to_string_lossy() }
    }
    pub fn get_steamid(&self) -> u64 {
        let ptr = unsafe { (*self.player_state()).unique_id.unique_id };
        assert!(!ptr.is_null());
        unsafe { (*ptr).steamid.get() }
    }

    pub fn movement_mode(&self) -> u8 {
        unsafe { (*self.movement()).movement_mode }
    }
    pub fn set_movement_mode(&mut self, value: u8) {
        unsafe { (*self.movement()).movement_mode = value };
    }
    pub fn max_fly_speed(&self) -> f32 {
        unsafe { (*self.movement()).max_fly_speed }
    }
    pub fn set_max_fly_speed(&mut self, value: f32) {
        unsafe { (*self.movement()).max_fly_speed = value };
    }

    pub fn get_viewport_size(&self) -> (i32, i32) {
        let mut width: i32 = -1;
        let mut height: i32 = -1;
        let fun: extern_fn!(fn(
            this: *mut APlayerController, size_x: &mut i32, size_y: &mut i32
        )) = unsafe { mem::transmute(APLAYERCONTROLLER_GETVIEWPORTSIZE.load(Ordering::SeqCst)) };
        fun(self.controller(), &mut width, &mut height);
        (width, height)
    }
    pub fn exit_water() {
        unsafe {
            let fun: extern_fn!(fn(this: *mut AMyCharacterUE, value: bool))
                = ::std::mem::transmute(AMYCHARACTER_UNDERWATERCHANGED.load(Ordering::SeqCst));
            fun(AMyCharacter::get_player().as_ptr(), false);
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
        let location: StructValueWrapper = params.get_field("NewLocation").unwrap();
        location.get_field("X").unwrap::<&Cell<f32>>().set(loc.x);
        location.get_field("Y").unwrap::<&Cell<f32>>().set(loc.y);
        location.get_field("Z").unwrap::<&Cell<f32>>().set(loc.z);
        let rotation: StructValueWrapper = params.get_field("NewRotation").unwrap();
        rotation.get_field("Pitch").unwrap::<&Cell<f32>>().set(rot.pitch);
        rotation.get_field("Yaw").unwrap::<&Cell<f32>>().set(rot.yaw);
        rotation.get_field("Roll").unwrap::<&Cell<f32>>().set(rot.roll);
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
        let s: StructValueWrapper = params.get_field("NewScale").unwrap();
        s.get_field("X").unwrap::<&Cell<f32>>().set(scale.x);
        s.get_field("Y").unwrap::<&Cell<f32>>().set(scale.y);
        s.get_field("Z").unwrap::<&Cell<f32>>().set(scale.z);

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

#[rtil_derive::hook_once(AMyCharacter::Tick)]
fn save(args: &mut Args) {
    let this = unsafe { args.nth_integer_arg(0) } as *mut AMyCharacterUE;
    CURRENT_PLAYER.store(this, Ordering::SeqCst);
    let my_character = AMyCharacter::get_player();
    log!("Got AMyCharacter: {:p}", this);
    log!("Got AMyCharacter::RootComponent: {:p}", my_character.root_component());
    log!("Got AMyCharacter::Controller: {:p}", my_character.controller());
    log!("Got AMyCharacter::Movement: {:p}", my_character.movement());
    log!("Got AMyCharacter::Movement::MovementMode: {:p}", unsafe { &(*my_character.movement()).movement_mode });
    log!("Got AMyCharacter::Movement::Acceleration: {:p}", unsafe { &(*my_character.movement()).acceleration });
    log!("Got AMyCharacter::Movement::MaxFlySpeed : {:p}", unsafe { &(*my_character.movement()).max_fly_speed });
    REBO_DOESNT_START_SEMAPHORE.release();
}
