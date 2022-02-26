use crate::native::ue::{FVector, FRotator};
use crate::native::uworld::UClass;
use crate::native::AMYCHARACTER_STATICCLASS;
use crate::statics::Static;

lazy_static::lazy_static! {
    static ref CHARACTER: Static<usize> = Static::new();
}

#[derive(Debug, PartialEq, Eq)]
pub struct AMyCharacter(usize);

impl AMyCharacter {
    pub(in crate::native) fn static_class() -> *const UClass {
        let fun: extern "C" fn() -> *const UClass
            = unsafe { ::std::mem::transmute(AMYCHARACTER_STATICCLASS) };
        let res = fun();
        res
    }
    fn as_ue(&self) -> *mut AMyCharacterUE {
        self.0 as *mut AMyCharacterUE
    }
    fn root_component(&self) -> &USceneComponent {
        unsafe { &*(*self.as_ue()).root_component }
    }
    fn root_component_mut(&mut self) -> &mut USceneComponent {
        unsafe { &mut *(*self.as_ue()).root_component }
    }
    fn controller(&self) -> &APlayerController {
        unsafe { &*(*self.as_ue()).controller }
    }
    fn controller_mut(&mut self) -> &mut APlayerController {
        unsafe { &mut *(*self.as_ue()).controller }
    }
    fn movement(&self) -> &UCharacterMovementComponent {
        unsafe { &*(*self.as_ue()).movement }
    }
    fn movement_mut(&mut self) -> &mut UCharacterMovementComponent {
        unsafe { &mut *(*self.as_ue()).movement }
    }

    pub unsafe fn new(ptr: *mut AMyCharacter) -> AMyCharacter {
        AMyCharacter(ptr as usize)
    }

    pub fn as_ptr(&self) -> *mut AMyCharacter {
        self.0 as *mut AMyCharacter
    }

    pub fn get_player() -> AMyCharacter {
        AMyCharacter(*CHARACTER.get())
    }

    pub fn location(&self) -> (f32, f32, f32) {
        let FVector { x, y, z } = self.root_component().location;
        (x, y, z)
    }
    pub fn set_location(&mut self, x: f32, y: f32, z: f32) {
        self.root_component_mut().location = FVector { x, y, z };
    }
    pub fn velocity(&self) -> (f32, f32, f32) {
        let FVector { x, y, z } = self.movement().velocity;
        (x, y, z)
    }
    pub fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.movement_mut().velocity = FVector { x, y, z };
    }
    pub fn acceleration(&self) -> (f32, f32, f32) {
        let FVector { x, y, z } = self.movement().acceleration;
        (x, y, z)
    }
    pub fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        self.movement_mut().acceleration = FVector { x, y, z };
    }
    pub fn rotation(&self) -> (f32, f32, f32) {
        let FRotator { pitch, yaw, roll } = self.controller().rotation;
        (pitch, yaw, roll)
    }
    pub fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.controller_mut().rotation = FRotator { pitch, yaw, roll };
    }
}

#[repr(C)]
struct AMyCharacterUE {
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
struct USceneComponent {
    #[cfg(unix)] _pad: [u8; 0x1a0],
    #[cfg(windows)] _pad: [u8; 0x140],
    location: FVector,
}

#[repr(C)]
struct UCharacterMovementComponent {
    #[cfg(unix)] _pad: [u8; 0x104],
    #[cfg(windows)] _pad: [u8; 0xb4],
    velocity: FVector,
    #[cfg(unix)] _pad2: [u8; 0x178],
    #[cfg(windows)] _pad2: [u8; 0x14c],
    acceleration: FVector,
}

#[repr(C)]
struct APlayerController {
    #[cfg(unix)] _pad: [u8; 0x3b8],
    #[cfg(windows)] _pad: [u8; 0x2d0],
    rotation: FRotator,
}

#[rtil_derive::hook_once(AMyCharacter::Tick)]
fn save(this: usize) {
    CHARACTER.set(this);
    let my_character = AMyCharacter::get_player();
    log!("Got AMyCharacter: {:#x}", this);
    log!("Got AMyCharacter::RootComponent: {:#x}", my_character.root_component() as *const _ as usize);
    log!("Got AMyCharacter::Controller: {:#x}", my_character.controller() as *const _ as usize);
    log!("Got AMyCharacter::Movement: {:#x}", my_character.movement() as *const _ as usize);
}
