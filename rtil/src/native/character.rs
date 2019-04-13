use native::ue::FVector;
use statics::Static;

lazy_static! {
    static ref CHARACTER: Static<usize> = Static::new();
}

pub struct AMyCharacter;

impl AMyCharacter {
    fn root_component() -> *mut USceneComponent {
        #[cfg(unix)] unsafe { *((&*CHARACTER.get() + 0x168) as *const *mut USceneComponent) }
        #[cfg(windows)] unsafe { *((&*CHARACTER.get() + 0x11c) as *const *mut USceneComponent) }
    }
    fn movement() -> *mut UCharacterMovementComponent {
        #[cfg(unix)] unsafe { *((&*CHARACTER.get() + 0x3f0) as *const *mut UCharacterMovementComponent) }
        #[cfg(windows)] unsafe { *((&*CHARACTER.get() + 0x2fc) as *const *mut UCharacterMovementComponent) }
    }

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

#[repr(C, packed)]
struct USceneComponent {
    #[cfg(unix)] _pad: [u8; 0x1a0],
    #[cfg(windows)] _pad: [u8; 0x140],
    location: FVector,
}

#[repr(C, packed)]
struct UCharacterMovementComponent {
    #[cfg(unix)] _pad: [u8; 0x104],
    #[cfg(windows)] _pad: [u8; 0xb4],
    velocity: FVector,
    #[cfg(unix)] _pad2: [u8; 0x178],
    #[cfg(windows)] _pad2: [u8; 0x14c],
    acceleration: FVector,
}

#[rtil_derive::hook_once(AMyCharacter::Tick)]
fn save(this: usize) {
    CHARACTER.set(this);
    log!("Got AMyCharacter: {:#x}", this);
    log!("Got AMyCharacter::RootComponent: {:#x}", AMyCharacter::root_component() as usize);
    log!("Got AMyCharacter::Movement: {:#x}", AMyCharacter::movement() as usize);
}
