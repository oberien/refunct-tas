use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use consts;
use statics::Static;


lazy_static! {
    static ref START: Static<[u8; 12]> = Static::new();
}

pub fn hook_newgame() {
    log!("Hooking AMyCharacter::execForcedUnCrouch");
    super::make_rw(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    let hook_fn = new_game as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::AMYCHARACTER_EXECFORCEDUNCROUCH as *mut u8, 12) }; 
    let mut saved = [0u8; 12];
    saved[..].copy_from_slice(tick);
    START.set(saved);
    log!("orig execforceduncrouch: {:?}", tick);
    // mov rax, addr
    tick[..2].copy_from_slice(&[0x48, 0xb8]);
    (&mut tick[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    // jmp rax
    tick[10..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    super::make_rx(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    log!("AMyCharacter::execForcedUnCrouch successfully hooked");
}

fn restore_newgame() {
    log!("Restoring AMyCharacter::execForcedUnCrouch");
    super::make_rw(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::AMYCHARACTER_EXECFORCEDUNCROUCH as *mut u8, 12) }; 
    tick[..].copy_from_slice(&*START.get());
    super::make_rx(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    log!("AMyCharacter::execForcedUnCrouch successfully restored");
}

#[naked]
unsafe extern fn new_game() -> ! {
    // push arguments
    asm!("push rdi" :::: "intel");
    asm!("push rsi" :::: "intel");
    alignstack_pre!();
    // call interceptor
    asm!("call rax" :: "{rax}"(::native::new_game as usize) :: "intel");
    // restore original function
    asm!("call rax" :: "{rax}"(restore_newgame as usize) :: "intel");
    alignstack_post!();
    // restore registers
    asm!(r"
        pop rsi
        pop rdi
    " :::: "intel");
    alignstack_pre!();
    // call original function
    asm!("call rax" :: "{rax}"(consts::AMYCHARACTER_EXECFORCEDUNCROUCH) :: "intel");
    alignstack_post!();
    // save rax (return value of original function)
    asm!("push rax" :::: "intel");
    alignstack_pre!();
    // hook method again
    asm!("call rax" :: "{rax}"(hook_newgame as usize) :: "intel");
    alignstack_post!();
    // restore rax
    asm!("pop rax" :::: "intel");
    // return to the original caller
    asm!("ret" :::: "intel");
    ::std::intrinsics::unreachable()
}

