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
    static ref SLATEAPP_START: Static<[u8; 12]> = Static::new();
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
    log!("Hooking FSlateApplication::Tick");
    make_rw(consts::FSLATEAPPLICATION_TICK);
    let hook_fn = get_slateapp as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut u8, 12) }; 
    let mut saved = [0u8; 12];
    saved[..].copy_from_slice(tick);
    SLATEAPP_START.set(saved);
    log!("orig tick: {:?}", tick);
    // mov rax, addr
    tick[..2].copy_from_slice(&[0x48, 0xb8]);
    (&mut tick[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    // jmp rax
    tick[10..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    make_rx(consts::FSLATEAPPLICATION_TICK);
    log!("FSlateApplication::Tick successfully hooked");
}

#[naked]
unsafe extern fn get_slateapp() -> ! {
    asm!("push rdi" :::: "intel");
    asm!("push rbp" :::: "intel");
    // align stack
    asm!("mov rbp, rsp" :::: "intel");
    asm!("and rsp, 0xfffffffffffffff0" :::: "intel");
    asm!("call rax" :: "{rax}"(save_slateapp as usize) :: "intel");
    asm!(r"
        mov rsp, rbp
        pop rbp
        pop rdi
        jmp rax
    ":: "{rax}"(consts::FSLATEAPPLICATION_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_slateapp(this: usize) {
    make_rw(consts::FSLATEAPPLICATION_TICK);
    SLATEAPP.set(this);
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut _, 12) }; 
    tick.copy_from_slice(&*SLATEAPP_START.get());
    make_rx(consts::FSLATEAPPLICATION_TICK);
    log!("Got FSlateApplication: {:#x}", this);
}

pub fn hook_tick() {
    ::std::thread::sleep(::std::time::Duration::from_secs(5));
    log!("Hooking FEngineLoop::Tick");
    make_rw(consts::FENGINELOOP_TICK_AFTER_UPDATETIME);
    let hook_fn = tick as *const () as usize;
    let mut bytes = unsafe { slice::from_raw_parts_mut(consts::FENGINELOOP_TICK_AFTER_UPDATETIME as *mut u8, 14) };
    // mov rax, addr
    bytes[..2].copy_from_slice(&[0x48, 0xb8]);
    (&mut bytes[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    // jmp rax
    bytes[10..12].copy_from_slice(&[0xff, 0xe0]);
    // nop
    bytes[12] = 0x90;
    bytes[13] = 0x90;
    log!("Injected Code: {:?}", bytes);
    make_rx(consts::FENGINELOOP_TICK_AFTER_UPDATETIME);
    log!("FEngineLoop::Tick hooked successfully");
}

#[naked]
unsafe extern fn tick() {
    // we are inside a function, so we need to push everything
    asm!(r"
        push rax
        push rbx
        push rcx
        push rdx
        push rsi
        push rdi
        push rbp
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15
        sub rsp, 0x80
        movdqu [rsp+0x70], xmm0
        movdqu [rsp+0x60], xmm1
        movdqu [rsp+0x50], xmm2
        movdqu [rsp+0x40], xmm3
        movdqu [rsp+0x30], xmm4
        movdqu [rsp+0x20], xmm5
        movdqu [rsp+0x10], xmm6
        movdqu [rsp], xmm7
    " :::: "intel");

    // align stack and call our function
    asm!(r"
        mov rbp, rsp
        and rsp, 0xfffffffffffffff0
        call rax
        mov rsp, rbp
    " :: "{rax}"(::native::tick_intercept as usize) :: "intel");

    // restore all registers
    asm!(r"
        movdqu xmm7, [rsp]
        movdqu xmm6, [rsp+0x10]
        movdqu xmm5, [rsp+0x20]
        movdqu xmm4, [rsp+0x30]
        movdqu xmm3, [rsp+0x40]
        movdqu xmm2, [rsp+0x50]
        movdqu xmm1, [rsp+0x60]
        movdqu xmm0, [rsp+0x70]
        add rsp, 0x80
        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rbp
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rbx
        pop rax
    " :::: "intel");
    // execute the 3 instructions which we overwrote
    asm!(r"
        mov rdi, [$0]
        mov rax, [rdi]
        call [rax+0x60]
    " :: "i"(consts::GMALLOC) :: "intel");
    // jump to original tick function after our hook
    asm!(r"
        mov rax, $0
        jmp rax
    " :: "i"(consts::FENGINELOOP_TICK_AFTER_UPDATETIME + 14) :: "intel");
}
