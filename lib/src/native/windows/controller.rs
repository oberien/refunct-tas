use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use statics::Static;
use native::CONTROLLER;
use super::ACONTROLLER_GETCONTROLROTATION;

lazy_static! {
    static ref START: Static<[u8; 7]> = Static::new();
}

pub struct AController;

impl AController {
    pub fn rotation() -> (f32, f32, f32) {
        let pitch = unsafe { *AController::pitch_ptr() };
        let yaw = unsafe { *AController::yaw_ptr() };
        let roll = unsafe { *AController::roll_ptr() };
        (pitch, yaw, roll)
    }

    pub fn set_rotation(pitch: f32, yaw: f32, roll: f32) {
        unsafe {
            *AController::pitch_ptr() = pitch;
            *AController::yaw_ptr() = yaw;
            *AController::roll_ptr() = roll;
        }
    }

    unsafe fn pitch_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x2d0) as *mut f32
    }
    unsafe fn yaw_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x2d4) as *mut f32
    }
    unsafe fn roll_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x2d8) as *mut f32
    }
}

pub fn hook_controller() {
    log!("Hooking AController::GetControlRotation");
    let addr = unsafe { ACONTROLLER_GETCONTROLROTATION };
    super::make_rw(addr);
    let hook_fn = get_controller as *const () as usize;
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
    log!("AController::GetControlRotation successfully hooked");
}

#[naked]
unsafe extern fn get_controller() -> ! {
    // push argument
    asm!("push ecx" :::: "intel");
    // call interceptor
    asm!("call eax" :: "{eax}"(save_controller as usize) :: "intel");
    // restore everything and jump to original function
    asm!(r"
        pop ecx
        jmp eax
    ":: "{eax}"(ACONTROLLER_GETCONTROLROTATION) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_controller(this: usize) {
    let addr = unsafe { ACONTROLLER_GETCONTROLROTATION };
    super::make_rw(addr);
    CONTROLLER.set(this);
    let mut code = unsafe { slice::from_raw_parts_mut(addr as *mut _, 7) }; 
    code.copy_from_slice(&*START.get());
    super::make_rx(addr);
    log!("Got AController: {:#x}", this);
}

