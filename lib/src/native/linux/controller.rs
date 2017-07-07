use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use statics::Static;
use native::CONTROLLER;
use consts;

lazy_static! {
    static ref START: Static<[u8; 12]> = Static::new();
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
        (&*CONTROLLER.get() + 0x3b8) as *mut f32
    }
    unsafe fn yaw_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3bc) as *mut f32
    }
    unsafe fn roll_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3c0) as *mut f32
    }
}

pub fn hook_controller() {
    log!("Hooking AController::GetControlRotation");
    super::make_rw(consts::ACONTROLLER_GETCONTROLROTATION);
    let hook_fn = get_controller as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::ACONTROLLER_GETCONTROLROTATION as *mut u8, 12) }; 
    let mut saved = [0u8; 12];
    saved[..].copy_from_slice(tick);
    START.set(saved);
    log!("Original: {:?}", tick);
    // mov rax, addr
    tick[..2].copy_from_slice(&[0x48, 0xb8]);
    (&mut tick[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    // jmp rax
    tick[10..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected: {:?}", tick);
    super::make_rx(consts::ACONTROLLER_GETCONTROLROTATION);
    log!("AController::GetControlRotation successfully hooked");
}

#[naked]
unsafe extern fn get_controller() -> ! {
    // push argument
    asm!("push rdi" :::: "intel");
    alignstack_pre!();
    // call interceptor
    asm!("call rax" :: "{rax}"(save_controller as usize) :: "intel");
    alignstack_post!();
    // restore everything and jump to original function
    asm!(r"
        pop rdi
        jmp rax
    ":: "{rax}"(consts::ACONTROLLER_GETCONTROLROTATION) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_controller(this: usize) {
    super::make_rw(consts::ACONTROLLER_GETCONTROLROTATION);
    CONTROLLER.set(this);
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::ACONTROLLER_GETCONTROLROTATION as *mut _, 12) }; 
    tick.copy_from_slice(&*START.get());
    super::make_rx(consts::ACONTROLLER_GETCONTROLROTATION);
    log!("Got AController: {:#x}", this);
}

