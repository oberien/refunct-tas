use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use super::{FENGINELOOP_TICK_AFTER_UPDATETIME, FAPP_DELTATIME, make_rw, make_rx};

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
        add eax, 8
        jmp eax
    " :: "{eax}"(FENGINELOOP_TICK_AFTER_UPDATETIME) :: "intel");
    ::std::intrinsics::unreachable()
}
