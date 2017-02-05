use std::slice;

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};
use byteorder::{WriteBytesExt, LittleEndian};

use consts;
use native::SLATEAPP;
use statics::Static;

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern fn() = ::initialize;

lazy_static! {
    static ref SLATEAPP_START: Static<[u8; 13]> = Static::new();
}

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
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut u8, 13) }; 
    let mut saved = [0u8; 13];
    saved[..].copy_from_slice(tick);
    SLATEAPP_START.set(saved);
    log!("orig tick: {:?}", tick);
    // push rax
    tick[0] = 0x50;
    // mov rax, addr
    tick[1..3].copy_from_slice(&[0x48, 0xb8]);
    (&mut tick[3..11]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    // jmp rax
    tick[11..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    make_rx(consts::FSLATEAPPLICATION_TICK);
}

#[naked]
unsafe extern fn get_slateapp() -> ! {
    asm!("push rdi" :::: "intel");
    asm!("call r14" :: "{r14}"(save_slateapp as usize) :: "intel");
    asm!(r"
        pop rdi
        pop rax
        jmp r14
    ":: "{r14}"(consts::FSLATEAPPLICATION_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_slateapp(this: usize) {
    log!("before sleep");
    ::std::thread::sleep(::std::time::Duration::from_secs(5));
    log!("after sleep");
    make_rw(consts::FSLATEAPPLICATION_TICK);
    SLATEAPP.set(this);
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut _, 13) }; 
    tick.copy_from_slice(&*SLATEAPP_START.get());
    make_rx(consts::FSLATEAPPLICATION_TICK);
    log!("Got FSlateApplication: {:#x}", this);
}


