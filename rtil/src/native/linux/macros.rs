/// Preamble code to align the stack to 16 bytes
macro_rules! alignstack_pre {
    () => {{
        asm!(r"
            push rbp
            mov rbp, rsp
            and rsp, 0xfffffffffffffff0
        " :::: "intel","volatile");
    }}
}

/// Epilogue code to restore the aligned stack to its original state
macro_rules! alignstack_post {
    () => {{
        asm!(r"
            mov rsp, rbp
            pop rbp
        " :::: "intel","volatile");
    }}
}

/// Push all registers including xmm0-7
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
        " :::: "intel","volatile");
    }}
}
/// Pop all registers including xmm0-7
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
        " :::: "intel","volatile");
    }}
}

/// Generates functions to hook and unhook the function at given address
///
/// # Parameters
///
/// * `orig_name`: Name of the original function to hook (for logging purposes)
/// * `orig_addr`: Address of the original function to hook
/// * `hook_name`: Name of the function hooking the original function
/// * `unhook_name`: Name of the function unhooking the original function
/// * `hook_fn`: Function to call when the hook triggers.
///      Can be generated with `hook_fn_once!` or `hook_fn_always!`.
/// * `log`: Indicates whether to log messages or not
macro_rules! hook {
    ($orig_name:expr, $orig_addr:expr, $hook_name:ident, $unhook_name:ident, $hook_fn:path, $log:expr,) => {
        use std::slice;
        use byteorder::{WriteBytesExt, LittleEndian};
        use statics::Static;

        lazy_static! {
            static ref ORIGINAL: Static<[u8; 12]> = Static::new();
        }

        pub fn $hook_name() {
            if $log { log!("Hooking {}", $orig_name); }
            let addr = unsafe { $orig_addr };
            super::make_rw(addr);
            let hook_fn = $hook_fn as *const () as usize;
            let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 12) };
            let mut saved = [0u8; 12];
            saved[..].copy_from_slice(slice);
            ORIGINAL.set(saved);
            if $log { log!("Original {}: {:?}", $orig_name, slice); }
            // mov rax, addr
            slice[..2].copy_from_slice(&[0x48, 0xb8]);
            (&mut slice[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
            // jmp rax
            slice[10..].copy_from_slice(&[0xff, 0xe0]);
            if $log { log!("Injected Code: {:?}", slice); }
            super::make_rx(addr);
            if $ log { log!("{} successfully hooked", $orig_name); }
        }

        fn $unhook_name() {
            if $log { log!("Restoring {}", $orig_name); }
            let addr = unsafe { $orig_addr };
            super::make_rw(addr);
            let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 12) };
            slice[..].copy_from_slice(&*ORIGINAL.get());
            super::make_rx(addr);
            if $log { log!("{} successfully restored", $orig_name); }
        }
    }
}

/// Generates a hook-function which calls the interceptor on first execution of the hook and
/// unhooks the original function afterwards forever.
///
/// # Parameters
///
/// * `hook_fn`: Name of hook-function
/// * `interceptor`: Interceptor function to be called whenever the hook is triggered
/// * `unhook_name`: Name of the unhooking function to restore the original function
/// * `orig_addr`: Address of the original function
macro_rules! hook_fn_once {
    ($hook_fn:ident, $interceptor:path, $unhook_name:path, $orig_addr:expr,) => {
        #[naked]
        unsafe extern "C" fn $hook_fn() -> ! {
            // push arguments
            pushall!();
            alignstack_pre!();
            // call interceptor
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel","volatile");
            // restore original function
            asm!("call rax" :: "{rax}"($unhook_name as usize) :: "intel","volatile");
            alignstack_post!();
            // restore register
            popall!();
            // jump to original function
            asm!("jmp rax" :: "{rax}"($orig_addr) :: "intel","volatile");
            ::std::intrinsics::unreachable()
        }
    }
}

/// Generates a hook-function, which call the interceptor on every execution of the hook and
/// keeps the original function hooked.
///
/// # Parameters
///
/// * `hook_fn`: Name of hook-function
/// * `interceptor`: Interceptor function to be called whenever the hook is triggered
/// * `hook_name`: Name of the hooking function to hook the original function
/// * `unhook_name`: Name of the unhooking function to restore the original function
/// * `orig_addr`: Address of the original function
/// * `order`: `intercept before original` or `intercept after original`
macro_rules! hook_fn_always {
    ($hook_fn:ident, $interceptor:path, $hook_name:path, $unhook_name:path, $orig_addr:expr, intercept before original,) => {
        #[naked]
        unsafe extern "C" fn $hook_fn() -> ! {
            pushall!();
            alignstack_pre!();
            // call interceptor
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel","volatile");
            // restore original function
            asm!("call rax" :: "{rax}"($unhook_name as usize) :: "intel","volatile");
            alignstack_post!();
            popall!();

            // call original function
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($orig_addr) :: "intel","volatile");
            alignstack_post!();

            // save rax (return value of original function
            asm!("push rax" :::: "intel","volatile");

            // hook method again
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($hook_name as usize) :: "intel","volatile");
            alignstack_post!();

            // restore rax
            asm!("pop rax" :::: "intel","volatile");

            // return to original caller
            asm!("ret" :::: "intel","volatile");
            ::std::intrinsics::unreachable()
        }
    };
    ($hook_fn:ident, $interceptor:path, $hook_name:path, $unhook_name:path, $orig_addr:expr, intercept after original,) => {
        #[naked]
        unsafe extern "C" fn $hook_fn() -> ! {
            // restore original function
            pushall!();
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($unhook_name as usize) :: "intel","volatile");
            alignstack_post!();
            popall!();

            // call original function
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($orig_addr) :: "intel","volatile");
            alignstack_post!();

            // save rax (return value of original function
            asm!("push rax" :::: "intel","volatile");

            alignstack_pre!();
            // hook method again
            asm!("call rax" :: "{rax}"($hook_name as usize) :: "intel","volatile");
            // call interceptor
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel","volatile");
            alignstack_post!();

            // restore rax
            asm!("pop rax" :::: "intel","volatile");

            // return to original caller
            asm!("ret" :::: "intel","volatile");
            ::std::intrinsics::unreachable()
        }
    }
}