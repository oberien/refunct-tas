use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use consts;

pub fn hook_tick() {
    log!("Hooking FEngineLoop::Tick");
    super::make_rw(consts::FENGINELOOP_TICK_AFTER_UPDATETIME);
    let hook_fn = tick as *const () as usize;
    let mut bytes = unsafe { slice::from_raw_parts_mut(consts::FENGINELOOP_TICK_AFTER_UPDATETIME as *mut u8, 15) };
    // mov rax, addr
    bytes[..2].copy_from_slice(&[0x48, 0xb8]);
    (&mut bytes[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    // jmp rax
    bytes[10..12].copy_from_slice(&[0xff, 0xe0]);
    // nop
    bytes[12] = 0x90;
    bytes[13] = 0x90;
    bytes[14] = 0x90;
    log!("Injected Code: {:?}", bytes);
    super::make_rx(consts::FENGINELOOP_TICK_AFTER_UPDATETIME);
    log!("FEngineLoop::Tick hooked successfully");
}

#[naked]
unsafe extern fn tick() -> ! {
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

    alignstack_pre!();
    // call our function
    asm!("call rax" :: "{rax}"(::native::tick_intercept as usize) :: "intel");
    alignstack_post!();

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
        movsd xmm0, [$1]
        cvtsd2ss xmm0, xmm0
    " :: "i"(consts::GENGINE), "i"(consts::APP_DELTATIME) :: "intel");
    // jump to original tick function after our hook
    asm!(r"
        mov rax, $0
        jmp rax
    " :: "i"(consts::FENGINELOOP_TICK_AFTER_UPDATETIME + 15) :: "intel");
    ::std::intrinsics::unreachable()
}
