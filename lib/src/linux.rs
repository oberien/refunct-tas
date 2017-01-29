use std::slice;

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};

use consts;

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern fn() = ::initialize;

pub fn make_rw(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_WRITE); }
}

pub fn make_rx(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_EXEC); }
}

pub fn hook_slateapp() {
    make_rw(consts::FSLATEAPPLICATION_TICK);
    let hook_fn = get_slateapp as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut _, 13) }; 
    // mov r14, addr
    tick[..2].copy_from_slice(&[0x49, 0xbe]);
    for i in 0..8 {
        tick[2+i] = (hook_fn >> i*8) as u8;
    }
    // jmp r14
    tick[10..].copy_from_slice(&[0x41, 0xff, 0xe6]);
    log!("Injected Code: {:?}", tick);
    make_rx(consts::FSLATEAPPLICATION_TICK);
}

#[naked]
unsafe extern fn get_slateapp() -> ! {
    asm!("push rdi" :::: "intel");
    asm!("call r14" :: "{r14}"(save_slateapp as usize) :: "intel");
    asm!("push rax" :::: "intel");
    asm!(r"
        pop rax
        pop rdi
        jmp r14
    ":: "{r14}"(consts::FSLATEAPPLICATION_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_slateapp(this: usize) {
    make_rw(consts::FSLATEAPPLICATION_TICK);
    *::SLATEAPP.lock().unwrap() = this;
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut _, 13) }; 
    tick.copy_from_slice(&consts::FSLATEAPPLICATION_TICK_BEGIN);
    make_rx(consts::FSLATEAPPLICATION_TICK);
    log!("Got FSlateApplication: {:#x}", this);
}
