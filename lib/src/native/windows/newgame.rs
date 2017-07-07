use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use super::{AMYCHARACTER_EXECFORCEDUNCROUCH_END, make_rw, make_rx};

pub extern fn hook_newgame() {
    log!("Hooking AMyCharacter::execForcedUnCrouch");
    let addr = unsafe { AMYCHARACTER_EXECFORCEDUNCROUCH_END };
    make_rw(addr);
    let hook_fn = new_game as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) }; 
    // mov eax, addr
    tick[0] = 0xb8;
    (&mut tick[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
    // jmp eax
    tick[5..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    make_rx(addr);
    log!("AMyCharacter::execForcedUnCrouch successfully hooked");
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

