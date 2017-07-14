use super::AMYCHARACTER_TICK;
use native::CHARACTER;
use statics::Static;
use std::slice;
use byteorder::{WriteBytesExt, LittleEndian};

lazy_static! {
    static ref START: Static<[u8; 7]> = Static::new();
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
    pub fn acceleration() -> (f32, f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            let FVector { x, y, z: _ } = (*movement).acceleration;
            (x, y)
        }
    }
    fn root_component() -> *mut USceneComponent {
        unsafe { *((&*CHARACTER.get() + 0x11c) as *const *mut USceneComponent) }
    }
    fn movement() -> *mut UCharacterMovementComponent {
        unsafe { *((&*CHARACTER.get() + 0x2fc) as *const *mut UCharacterMovementComponent) }
    }
}

#[repr(C, packed)]
struct FVector {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C, packed)]
struct USceneComponent {
    _pad: [u8; 0x108],
    location: FVector,
}

#[repr(C, packed)]
struct UCharacterMovementComponent {
    _pad: [u8; 0xb4],
    velocity: FVector,
    _pad2: [u8; 0x14c],
    acceleration: FVector,
}

pub fn hook_character() {
    log!("Hooking AMyCharacter::Tick");
    let addr = unsafe { AMYCHARACTER_TICK };
    super::make_rw(addr);
    let hook_fn = get_character as *const () as usize;
    let mut code = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) };
    let mut saved = [0u8; 7];
    saved[..].copy_from_slice(code);
    START.set(saved);
    log!("Original: {:?}", code);
    // mov eax, addr
    code[0] = 0xb8;
    (&mut code[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
    // jmp rax
    code[5..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected: {:?}", code);
    super::make_rx(addr);
    log!("AMyCharacter::Tick successfully hooked");
}

#[naked]
unsafe extern fn get_character() -> ! {
    // push argument
    asm!("push ecx" :::: "intel");
    // call interceptor
    asm!("call eax" :: "{eax}"(save_character as usize) :: "intel");
    // restore everything and jump to original function
    asm!(r"
        pop ecx
        jmp eax
    ":: "{eax}"(AMYCHARACTER_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_character(this: usize) {
    let addr = unsafe { AMYCHARACTER_TICK };
    super::make_rw(addr);
    CHARACTER.set(this);
    let mut code = unsafe { slice::from_raw_parts_mut(addr as *mut _, 7) };
    code.copy_from_slice(&*START.get());
    super::make_rx(addr);
    log!("Got AMyCharacter: {:#x}", this);
    log!("Got AMyCharacter::RootComponent: {:#x}", AMyCharacter::root_component() as usize);
    log!("Got AMyCharacter::Movement: {:#x}", AMyCharacter::movement() as usize);
}
