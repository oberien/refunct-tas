macro_rules! alignstack_pre {
    () => {{
        asm!(r"
            push rbp
            mov rbp, rsp
            and rsp, 0xfffffffffffffff0
        " :::: "intel");
    }}
}
macro_rules! alignstack_post {
    () => {{
        asm!(r"
            mov rsp, rbp
            pop rbp
        " :::: "intel");
    }}
}

macro_rules! pushall {
    () => {{
        asm!(r"
            push rax
            push rbx
            push rcx
            push rdx
            push rsi
            push rdi
            push rbp
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
    }}
}
macro_rules! popall {
    () => {{
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
            pop rbp
            pop rdi
            pop rsi
            pop rdx
            pop rcx
            pop rbx
            pop rax
        " :::: "intel");
    }}
}

macro_rules! hook_beginning {
    ($hook_name:ident, $restore_name:ident, $hook_fn:path, $name:expr, $addr:expr,) => {
        use std::slice;
        use byteorder::{WriteBytesExt, LittleEndian};
        use statics::Static;

        lazy_static! {
            static ref ORIGINAL: Static<[u8; 12]> = Static::new();
        }

        pub fn $hook_name() {
            log!("Hooking {}", $name);
            let addr = unsafe { $addr };
            super::make_rw(addr);
            let hook_fn = $hook_fn as *const () as usize;
            let mut slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 12) };
            let mut saved = [0u8; 12];
            saved[..].copy_from_slice(slice);
            ORIGINAL.set(saved);
            log!("Original {}: {:?}", $name, slice);
            // mov rax, addr
            slice[..2].copy_from_slice(&[0x48, 0xb8]);
            (&mut slice[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
            // jmp rax
            slice[10..].copy_from_slice(&[0xff, 0xe0]);
            log!("Injected Code: {:?}", slice);
            super::make_rx(addr);
            log!("{} successfully hooked", $name);
        }

        fn $restore_name() {
            log!("Restoring {}", $name);
            let addr = unsafe { $addr };
            super::make_rw(addr);
            let mut slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 12) };
            slice[..].copy_from_slice(&*ORIGINAL.get());
            super::make_rx(addr);
            log!("{} successfully restored", $name);
        }
    }
}

macro_rules! hook_fn_once {
    ($hook_fn:ident, $interceptor:path, $restore_name:path, $addr:expr,) => {
        #[naked]
        unsafe extern fn $hook_fn() -> ! {
            // push arguments
            pushall!();
            alignstack_pre!();
            // call interceptor
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel");
            // restore original function
            asm!("call rax" :: "{rax}"($restore_name as usize) :: "intel");
            alignstack_post!();
            // restore register
            popall!();
            // jump to original function
            asm!("jmp rax" :: "{rax}"($addr) :: "intel");
            ::std::intrinsics::unreachable()
        }
    }
}

macro_rules! hook_fn_always {
    ($hook_fn:ident, $interceptor:path, $hook_name:path, $restore_name:path, $addr:expr,) => {
        hook_fn_always! {
            $hook_fn,
            $interceptor,
            $hook_name,
            $restore_name,
            $addr,
            intercept before original
        }
    };
    ($hook_fn:ident, $interceptor:path, $hook_name:path, $restore_name:path, $addr:expr, intercept before original) => {
        #[naked]
        unsafe extern fn $hook_fn() -> ! {
            pushall!();
            alignstack_pre!();
            // call interceptor
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel");
            // restore original function
            asm!("call rax" :: "{rax}"($restore_name as usize) :: "intel");
            alignstack_post!();
            popall!();

            // call original function
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($addr) :: "intel");
            alignstack_post!();

            // save rax (return value of original function
            asm!("push rax" :::: "intel");

            // hook method again
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($hook_name as usize) :: "intel");
            alignstack_post!();

            // restore rax
            asm!("pop rax" :::: "intel");

            // return to original caller
            asm!("ret" :::: "intel");
            ::std::intrinsics::unreachable()
        }
    };
    ($hook_fn:ident, $interceptor:path, $hook_name:path, $restore_name:path, $addr:expr, intercept after original) => {
        #[naked]
        unsafe extern fn $hook_fn() -> ! {
            // restore original function
            pushall!();
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($restore_name as usize) :: "intel");
            alignstack_post!();
            popall!();

            // call original function
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($addr) :: "intel");
            alignstack_post!();

            // save rax (return value of original function
            asm!("push rax" :::: "intel");

            alignstack_pre!();
            // hook method again
            asm!("call rax" :: "{rax}"($hook_name as usize) :: "intel");
            // call interceptor
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel");
            alignstack_post!();

            // restore rax
            asm!("pop rax" :::: "intel");

            // return to original caller
            asm!("ret" :::: "intel");
            ::std::intrinsics::unreachable()
        }
    }
}