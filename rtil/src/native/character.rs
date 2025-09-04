use std::cell::Cell;
use std::ffi::c_void;
use std::mem;
use std::sync::atomic::{AtomicPtr, Ordering};
use hook::{ArgsRef, RawHook, IsaAbi};
use iced::mouse::Interaction;
use crate::native::ue::{FVector, FRotator, FString, UeU64};
use crate::native::{AMYCHARACTER_STATICCLASS, REBO_DOESNT_START_SEMAPHORE, APLAYERCONTROLLER_GETVIEWPORTSIZE, ActorWrapper, ObjectWrapper, StructValueWrapper, BoolValueWrapper, AMYCHARACTER_UNDERWATERCHANGED, UObject, UeScope, APLAYERCONTROLLER_FLUSHPRESSEDKEYS, APLAYERCONTROLLER_GETMOUSEPOSITION};
use crate::native::reflection::UClass;
use crate::native::uworld::CAMERA_INDEX;

static CURRENT_PLAYER: AtomicPtr<AMyCharacterUE> = AtomicPtr::new(std::ptr::null_mut());

#[derive(Debug, PartialEq, Eq)]
pub struct AMyCharacter(*mut AMyCharacterUE);

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
#[allow(unused)]
pub enum EMouseCursorType {
    None,
    Default,
    TextEditBeam,
    ResizeLeftRight,
    ResizeUpDown,
    ResizeSouthEast,
    ResizeSouthWest,
    CardinalCross,
    Crosshairs,
    Hand,
    GrabHand,
    GrabHandClosed,
    SlashedCircle,
    EyeDropper,
    Custom,
    TotalCursorCount,
}
impl From<Interaction> for EMouseCursorType {
    fn from(interaction: Interaction) -> Self {
        match interaction {
            Interaction::None => EMouseCursorType::Default,
            Interaction::Idle => EMouseCursorType::Default,
            Interaction::Pointer => EMouseCursorType::Hand,
            Interaction::Grab => EMouseCursorType::GrabHand,
            Interaction::Text => EMouseCursorType::TextEditBeam,
            Interaction::Crosshair => EMouseCursorType::Crosshairs,
            Interaction::Working => EMouseCursorType::SlashedCircle,
            Interaction::Grabbing => EMouseCursorType::GrabHandClosed,
            Interaction::ResizingHorizontally => EMouseCursorType::ResizeLeftRight,
            Interaction::ResizingVertically => EMouseCursorType::ResizeUpDown,
            Interaction::ResizingDiagonallyUp => EMouseCursorType::ResizeSouthEast,
            Interaction::ResizingDiagonallyDown => EMouseCursorType::ResizeSouthWest,
            Interaction::NotAllowed => EMouseCursorType::Default,
            Interaction::ZoomIn => EMouseCursorType::Default,
            Interaction::ZoomOut => EMouseCursorType::Default,
            Interaction::Cell => EMouseCursorType::Default,
            Interaction::Move => EMouseCursorType::Default,
            Interaction::Copy => EMouseCursorType::Default,
            Interaction::Help => EMouseCursorType::Default,
        }
    }
}

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
    pub fn controller(&self) -> *mut APlayerController {
        unsafe { (*self.0).controller }
    }
    pub fn movement(&self) -> *mut UCharacterMovementComponent {
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

    pub fn get_max_walk_speed() -> f32 {
        unsafe {
            let movement = ObjectWrapper::new(AMyCharacter::get_player().movement() as *mut UObject);
            movement.get_field("MaxWalkSpeed").unwrap::<f32>()
        }
    }
    pub fn get_base_speed() -> f32 {
        unsafe {
            let movement = ObjectWrapper::new(AMyCharacter::get_player().as_ptr() as *mut UObject);
            movement.get_field("BaseSpeed").unwrap::<f32>()
        }
    }
    pub fn get_max_bonus_speed() -> f32 {
        unsafe {
            let movement = ObjectWrapper::new(AMyCharacter::get_player().as_ptr() as *mut UObject);
            movement.get_field("MaxBonusSpeed").unwrap::<f32>()
        }
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
            let obj = ObjectWrapper::new(AMyCharacter::get_player().as_ptr() as *mut UObject);
            let fun = obj.class().find_function("StopSwimEffect").unwrap();
            let params = fun.create_argument_struct();
            fun.call(obj.as_ptr(), &params);
            let fun = obj.class().find_function("StopFootstepEffect").unwrap();
            let params = fun.create_argument_struct();
            fun.call(obj.as_ptr(), &params);
        }
    }
    pub fn camera_mode() -> u8 {
        UeScope::with(|scope| {
            let cam = scope.get(CAMERA_INDEX.get().unwrap());
            cam.get_field("ProjectionMode").unwrap::<u8>()
        })
    }
    pub fn set_camera_mode(mode: u8) {
        unsafe {
            UeScope::with(|scope| {
                let cam = scope.get(CAMERA_INDEX.get().unwrap());
                let fun = cam.class().find_function("SetProjectionMode").unwrap();
                let params = fun.create_argument_struct();
                params.get_field("InProjectionMode").unwrap::<&Cell<u8>>().set(mode);
                fun.call(cam.as_ptr(), &params);
            });
        }
    }
    pub fn flush_pressed_keys() {
        let fun: extern_fn!(fn(this: *mut APlayerController))
            = unsafe { ::std::mem::transmute(APLAYERCONTROLLER_FLUSHPRESSEDKEYS.load(Ordering::SeqCst)) };
        fun(AMyCharacter::get_player().controller());
    }
    pub fn set_mouse_cursor(cursor: EMouseCursorType) {
        let controller = unsafe { ObjectWrapper::new(AMyCharacter::get_player().controller() as *mut UObject) };
        controller.get_field("CurrentMouseCursor").unwrap::<&Cell<u8>>().set(cursor as u8);
    }
    pub fn get_mouse_position() -> (f32, f32) {
        let fun: extern_fn!(fn(this: *mut APlayerController, x: &mut f32, y: &mut f32)) = unsafe {
            mem::transmute(APLAYERCONTROLLER_GETMOUSEPOSITION.load(Ordering::SeqCst))
        };
        let mut x = 0.;
        let mut y = 0.;
        fun(AMyCharacter::get_player().controller(), &mut x, &mut y);
        (x, y)
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
pub struct UCharacterMovementComponent {
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
pub struct APlayerController {
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

pub fn tick_hook<IA: IsaAbi>(hook: &'static RawHook<IA, ()>, mut args: ArgsRef<'_, IA>) {
    let this = args.load::<*mut AMyCharacterUE>();
    CURRENT_PLAYER.store(this, Ordering::SeqCst);
    let my_character = AMyCharacter::get_player();
    log!("Got AMyCharacter: {:p}", this);
    log!("Got AMyCharacter::RootComponent: {:p}", my_character.root_component());
    log!("Got AMyCharacter::Controller: {:p}", my_character.controller());
    log!("Got AMyCharacter::Movement: {:p}", my_character.movement());
    log!("Got AMyCharacter::Movement::MovementMode: {:p}", unsafe { &(*my_character.movement()).movement_mode });
    log!("Got AMyCharacter::Movement::Acceleration: {:p}", unsafe { &(*my_character.movement()).acceleration });
    log!("Got AMyCharacter::Movement::MaxFlySpeed : {:p}", unsafe { &(*my_character.movement()).max_fly_speed });
    hook.disable();
    unsafe { hook.call_original_function(args) };
    REBO_DOESNT_START_SEMAPHORE.release();
}
