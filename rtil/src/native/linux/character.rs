use super::AMYCHARACTER_TICK;
use native::CHARACTER;
use native::ue::FVector;

pub struct AMyCharacter;

impl AMyCharacter {
    pub fn location() -> (f32, f32, f32) {
        let root = AMyCharacter::root_component();
        unsafe {
            ((*root).location.x, (*root).location.y, (*root).location.z)
        }
    }
    pub fn set_location(x: f32, y: f32, z: f32) {
        let root = AMyCharacter::root_component();
        unsafe {
            (*root).location = FVector { x, y, z };
        }
    }
    pub fn velocity() -> (f32, f32, f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            let FVector { x, y, z } = (*movement).velocity;
            (x, y, z)
        }
    }
    pub fn set_velocity(x: f32, y: f32, z: f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            (*movement).velocity = FVector { x, y, z };
        }
    }
    pub fn acceleration() -> (f32, f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            let FVector { x, y, z: _ } = (*movement).acceleration;
            (x, y)
        }
    }
    fn root_component() -> *mut USceneComponent {
        unsafe { *((&*CHARACTER.get() + 0x168) as *const *mut USceneComponent) }
    }
    fn movement() -> *mut UCharacterMovementComponent {
        unsafe { *((&*CHARACTER.get() + 0x3f0) as *const *mut UCharacterMovementComponent) }
    }
}

#[repr(C, packed)]
struct USceneComponent {
    _pad: [u8; 0x1a0],
    location: FVector,
}

#[repr(C, packed)]
struct UCharacterMovementComponent {
    _pad: [u8; 0x104],
    velocity: FVector,
    _pad2: [u8; 0x178],
    acceleration: FVector,
}

hook! {
    "AMyCharacter::Tick",
    AMYCHARACTER_TICK,
    hook_character,
    unhook_character,
    get_character,
    true,
}

hook_fn_once! {
    get_character,
    save_character,
    unhook_character,
    AMYCHARACTER_TICK,
}

#[inline(never)]
extern fn save_character(this: usize) {
    CHARACTER.set(this);
    log!("Got AMyCharacter: {:#x}", this);
    log!("Got AMyCharacter::RootComponent: {:#x}", AMyCharacter::root_component() as usize);
    log!("Got AMyCharacter::Movement: {:#x}", AMyCharacter::movement() as usize);
}
