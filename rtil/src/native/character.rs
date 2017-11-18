use native::AMYCHARACTER_TICK;
use native::ue::FVector;
#[cfg(unix)] use native::linux::character::save;
#[cfg(windows)] use native::windows::character::save;

lazy_static! {
    pub(in native) static ref CHARACTER: Static<usize> = Static::new();
}

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
    pub fn acceleration() -> (f32, f32, f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            let FVector { x, y, z } = (*movement).acceleration;
            (x, y, z)
        }
    }
    pub fn set_acceleration(x: f32, y: f32, z: f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            (*movement).acceleration = FVector { x, y, z };
        }
    }
}

hook! {
    "AMyCharacter::Tick",
    AMYCHARACTER_TICK,
    hook,
    unhook,
    get,
    true,
}

hook_fn_once! {
    get,
    save,
    unhook,
    AMYCHARACTER_TICK,
}

