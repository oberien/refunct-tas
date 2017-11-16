use native::character::{CHARACTER, AMyCharacter};
use native::ue::FVector;

impl AMyCharacter {
    pub(in native) fn root_component() -> *mut USceneComponent {
        unsafe { *((&*CHARACTER.get() + 0x168) as *const *mut USceneComponent) }
    }
    pub(in native) fn movement() -> *mut UCharacterMovementComponent {
        unsafe { *((&*CHARACTER.get() + 0x3f0) as *const *mut UCharacterMovementComponent) }
    }
}

#[repr(C, packed)]
pub(in native) struct USceneComponent {
    _pad: [u8; 0x1a0],
    pub location: FVector,
}

#[repr(C, packed)]
pub(in native) struct UCharacterMovementComponent {
    _pad: [u8; 0x104],
    pub velocity: FVector,
    _pad2: [u8; 0x178],
    pub acceleration: FVector,
}

#[inline(never)]
pub(in native) extern "C" fn save(this: usize) {
    CHARACTER.set(this);
    log!("Got AMyCharacter: {:#x}", this);
    log!("Got AMyCharacter::RootComponent: {:#x}", AMyCharacter::root_component() as usize);
    log!("Got AMyCharacter::Movement: {:#x}", AMyCharacter::movement() as usize);
}
