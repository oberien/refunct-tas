use native::character::{CHARACTER, AMyCharacter};
use native::ue::FVector;

impl AMyCharacter {
    pub(in native) fn root_component() -> *mut USceneComponent {
        unsafe { *((&*CHARACTER.get() + 0x11c) as *const *mut USceneComponent) }
    }
    pub(in native) fn movement() -> *mut UCharacterMovementComponent {
        unsafe { *((&*CHARACTER.get() + 0x2fc) as *const *mut UCharacterMovementComponent) }
    }
}

#[repr(C, packed)]
pub(in native) struct USceneComponent {
    _pad: [u8; 0x140],
    pub location: FVector,
}

#[repr(C, packed)]
pub(in native) struct UCharacterMovementComponent {
    _pad: [u8; 0xb4],
    pub velocity: FVector,
    _pad2: [u8; 0x14c],
    pub acceleration: FVector,
}

#[inline(never)]
pub(in native) extern "thiscall" fn save(this: usize) {
    CHARACTER.set(this);
    log!("Got AMyCharacter: {:#x}", this);
    log!("Got AMyCharacter::RootComponent: {:#x}", AMyCharacter::root_component() as usize);
    log!("Got AMyCharacter::Movement: {:#x}", AMyCharacter::movement() as usize);
}
