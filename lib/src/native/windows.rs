use std::slice;
use std::ptr::null;

use winapi::{self, c_void};
use winapi::winnt::{PAGE_READWRITE, PAGE_EXECUTE_READ};
use kernel32::{VirtualProtect, GetModuleHandleA};
use byteorder::{WriteBytesExt, LittleEndian};

use consts;
use native::SLATEAPP;
use statics::Static;

// https://www.unknowncheats.me/forum/general-programming-and-reversing/123333-demo-pure-rust-internal-coding.html
#[no_mangle]
// Entry Point
pub extern "stdcall" fn DllMain(module: u32, reason: u32, reserved: *mut c_void) {
    match reason {
        1 => ::initialize(),
        _ => ()
    }
}

lazy_static! {
    static ref SLATEAPP_START: Static<[u8; 7]> = Static::new();
    static ref UNCROUCH_START: Static<[u8; 7]> = Static::new();
    static ref BASE: usize = unsafe { GetModuleHandleA(null()) as usize };
}

pub static mut FSLATEAPPLICATION_TICK: usize = 0;
pub static mut AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0;
pub static mut FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0;
pub static mut APP_DELTATIME: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYDOWN: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYUP: usize = 0;
pub static mut FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0;

pub fn init() {
    let base = &*BASE;
    log!("Got Base address: {:#x}", base);
    unsafe {
        FSLATEAPPLICATION_TICK = base + consts::FSLATEAPPLICATION_TICK;
        AMYCHARACTER_EXECFORCEDUNCROUCH = base + consts::AMYCHARACTER_EXECFORCEDUNCROUCH;
        FENGINELOOP_TICK_AFTER_UPDATETIME = base + consts::FENGINELOOP_TICK_AFTER_UPDATETIME;
        APP_DELTATIME = base + consts::APP_DELTATIME;
        FSLATEAPPLICATION_ONKEYDOWN = base + consts::FSLATEAPPLICATION_ONKEYDOWN;
        FSLATEAPPLICATION_ONKEYUP = base + consts::FSLATEAPPLICATION_ONKEYUP;
        FSLATEAPPLICATION_ONRAWMOUSEMOVE = base + consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE;
    }
}

macro_rules! alignstack_pre {
    () => {{
        asm!(r"
            push ebp
            mov ebp, esp
            and esp, 0xfffffffffffffff0
        " :::: "intel");
    }}
}
macro_rules! alignstack_post {
    () => {{
        asm!(r"
            mov esp, ebp
            pop ebp
        " :::: "intel");
    }}
}
macro_rules! pushall {
    () => {{
        asm!(r"
            push eax
            push ebx
            push ecx
            push edx
            push esi
            push edi
            push ebp
            sub esp, 0x80
            movdqu [esp+0x70], xmm0
            movdqu [esp+0x60], xmm1
            movdqu [esp+0x50], xmm2
            movdqu [esp+0x40], xmm3
            movdqu [esp+0x30], xmm4
            movdqu [esp+0x20], xmm5
            movdqu [esp+0x10], xmm6
            movdqu [esp], xmm7
        " :::: "intel");
    }}
}
macro_rules! popall {
    () => {{
        asm!(r"
            movdqu xmm7, [esp]
            movdqu xmm6, [esp+0x10]
            movdqu xmm5, [esp+0x20]
            movdqu xmm4, [esp+0x30]
            movdqu xmm3, [esp+0x40]
            movdqu xmm2, [esp+0x50]
            movdqu xmm1, [esp+0x60]
            movdqu xmm0, [esp+0x70]
            add esp, 0x80
            pop ebp
            pop edi
            pop esi
            pop edx
            pop ecx
            pop ebx
            pop eax
        " :::: "intel");
    }}
}

pub fn make_rw(addr: usize) {
    log!("make_rw: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_READWRITE, &mut out); }
}

pub fn make_rx(addr: usize) {
    log!("make_rx: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_EXECUTE_READ, &mut out); }
}

pub fn hook_slateapp() {
    log!("Hooking FSlateApplication::Tick");
    let addr = unsafe { FSLATEAPPLICATION_TICK };
    make_rw(addr);
    let hook_fn = get_slateapp as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) }; 
    let mut saved = [0u8; 7];
    saved[..].copy_from_slice(tick);
    SLATEAPP_START.set(saved);
    log!("orig tick: {:?}", tick);
    // mov eax, addr
    tick[0] = 0xb8;
    (&mut tick[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
    // jmp eax
    tick[5..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    make_rx(addr);
    log!("FSlateApplication::Tick successfully hooked");
}

#[inline(never)]
#[no_mangle]
#[naked]
unsafe extern fn get_slateapp() -> ! {
    // push argument
    asm!("push ecx" :::: "intel");
    // call interceptor
    asm!("call $0
    " :: "i"(save_slateapp as usize) :: "intel");
    // restore everything and jump to original function
    asm!(r"
        pop ecx
        jmp eax
    ":: "{eax}"(FSLATEAPPLICATION_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
#[no_mangle]
extern fn save_slateapp(this: usize) {
    log!("save_slateapp");
    let addr = unsafe { FSLATEAPPLICATION_TICK };
    make_rw(addr);
    SLATEAPP.set(this);
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut _, 7) }; 
    tick.copy_from_slice(&*SLATEAPP_START.get());
    make_rx(addr);
    log!("Got FSlateApplication: {:#x}", this);
}

pub extern fn hook_newgame() {
    log!("Hooking AMyCharacter::execForcedUnCrouch");
    let addr = unsafe { AMYCHARACTER_EXECFORCEDUNCROUCH };
    make_rw(addr);
    let hook_fn = new_game as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) }; 
    let mut saved = [0u8; 7];
    saved[..].copy_from_slice(tick);
    UNCROUCH_START.set(saved);
    log!("orig execforceduncrouch: {:?}", tick);
    // mov eax, addr
    tick[0] = 0xb8;
    (&mut tick[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
    // jmp eax
    tick[5..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    make_rx(addr);
    log!("AMyCharacter::execForcedUnCrouch successfully hooked");
}

extern fn restore_newgame() {
    log!("Restoring AMyCharacter::execForcedUnCrouch");
    let addr = unsafe { AMYCHARACTER_EXECFORCEDUNCROUCH };
    make_rw(addr);
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) }; 
    tick[..].copy_from_slice(&*UNCROUCH_START.get());
    make_rx(addr);
    log!("AMyCharacter::execForcedUnCrouch successfully restored");
}

extern fn print0() {
    log!("0");
}
extern fn print1() {
    log!("1");
}
extern fn print2() {
    log!("2");
}
extern fn print_arg(val: usize) {
    log!("arg: {:#x}", val);
    //::std::thread::sleep(::std::time::Duration::from_secs(5000));
}
unsafe extern fn print_more(ecx: usize) {
    log!("ecx: {:#x}", ecx);
    log!("[ecx]: {:#x}", *(ecx as *const usize));
    log!("[ecx+0x2fc]: {:#x}", *((ecx+0x2fc) as *const usize));
    log!("[[ecx+0x2fc]]: {:#x}", **((ecx+0x2fc) as *const *const usize));
    log!("[[[ecx+0x2fc]]+0x364]: {:#x}", *((**((ecx+0x2fc) as *const *const usize) + 0x364) as *const usize));
    //::std::thread::sleep(::std::time::Duration::from_secs(5000));
}

#[naked]
unsafe extern fn new_game() -> ! {
    // The original function just called UnCrouch
    // save eax (return value of original function)
    asm!("push eax" :::: "intel");
    // call interceptor
    asm!("call $0" :: "i"(::native::new_game as usize) :: "intel");
    // restore eax
    asm!("pop eax" :::: "intel");
    // execute what we overwrote
    asm!(r"
        pop esi
        ret 0x0008
    " :::: "intel");
    ::std::intrinsics::unreachable()
}

pub fn hook_tick() {
    log!("Hooking FEngineLoop::Tick");
    let addr = unsafe { FENGINELOOP_TICK_AFTER_UPDATETIME };
    make_rw(addr);
    let hook_fn = tick as *const () as usize;
    let mut bytes = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 8) };
    // mov eax, addr
    bytes[0] = 0xb8;
    (&mut bytes[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
    // jmp rax
    bytes[5..7].copy_from_slice(&[0xff, 0xe0]);
    // nop
    bytes[7] = 0x90;
    log!("Injected Code: {:?}", bytes);
    make_rx(addr);
    log!("FEngineLoop::Tick hooked successfully");
}

#[naked]
unsafe extern fn tick() -> ! {
    // we are inside a function, so we need to push everything
    pushall!();
    // call our function
    asm!("call $0" :: "i"(::native::tick_intercept as usize) :: "intel");
    // restore all registers
    popall!();
    // execute the instruction which we overwrote
    asm!("movsd xmm0, [eax]" :: "{eax}"(APP_DELTATIME) :: "intel");
    // jump to original tick function after our hook
    asm!("
        add eax, 7
        jmp eax
    " :: "{eax}"(FENGINELOOP_TICK_AFTER_UPDATETIME) :: "intel");
    ::std::intrinsics::unreachable()
}
