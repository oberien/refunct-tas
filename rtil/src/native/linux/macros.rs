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

/// Generates functions to hook the beginning of a function and restore it
///
/// # Parameters
///
/// * **hook_name**: Name of the hook function
/// * **restore_name**: Name of the restore function
/// * **hook_fn**: Function to call from within the hook
/// * **name**: Name of the original function to hook (for logging purposes)
/// * **addr**: Address of the original function to hook
/// * **log**: `true` if logging should be enabled (Optional)
macro_rules! hook_beginning {
    ($hook_name:ident, $restore_name:ident, $hook_fn:path, $name:expr, $addr:expr,) => {
        hook_beginning! {
            $hook_name,
            $restore_name,
            $hook_fn,
            $name,
            $addr,
            true,
        }
    };
    ($hook_name:ident, $restore_name:ident, $hook_fn:path, $name:expr, $addr:expr, $log:expr,) => {
        use std::slice;
        use byteorder::{WriteBytesExt, LittleEndian};
        use statics::Static;

        lazy_static! {
            static ref ORIGINAL: Static<[u8; 12]> = Static::new();
        }

        pub fn $hook_name() {
            if $log { log!("Hooking {}", $name); }
            let addr = unsafe { $addr };
            super::make_rw(addr);
            let hook_fn = $hook_fn as *const () as usize;
            let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 12) };
            let mut saved = [0u8; 12];
            saved[..].copy_from_slice(slice);
            ORIGINAL.set(saved);
            if $log { log!("Original {}: {:?}", $name, slice); }
            // mov rax, addr
            slice[..2].copy_from_slice(&[0x48, 0xb8]);
            (&mut slice[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
            // jmp rax
            slice[10..].copy_from_slice(&[0xff, 0xe0]);
            if $log { log!("Injected Code: {:?}", slice); }
            super::make_rx(addr);
            if $ log { log!("{} successfully hooked", $name); }
        }

        fn $restore_name() {
            if $log { log!("Restoring {}", $name); }
            let addr = unsafe { $addr };
            super::make_rw(addr);
            let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 12) };
            slice[..].copy_from_slice(&*ORIGINAL.get());
            super::make_rx(addr);
            if $log { log!("{} successfully restored", $name); }
        }
    }
}

/// Generates a hook function with given name, which executes the the interceptor once and unhooks afterwards
///
/// # Parameters
///
/// * **hook_fn**: Name of the hook function that is generated
/// * **interceptor**: Interceptor function which is called from within the hook
/// * **restore_name**: Name of the restore function
/// * **addr**: Address of the original function
macro_rules! hook_fn_once {
    ($hook_fn:ident, $interceptor:path, $restore_name:path, $addr:expr,) => {
        #[naked]
        unsafe extern fn $hook_fn() -> ! {
            // push arguments
            pushall!();
            alignstack_pre!();
            // call interceptor
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel","volatile");
            // restore original function
            asm!("call rax" :: "{rax}"($restore_name as usize) :: "intel","volatile");
            alignstack_post!();
            // restore register
            popall!();
            // jump to original function
            asm!("jmp rax" :: "{rax}"($addr) :: "intel","volatile");
            ::std::intrinsics::unreachable()
        }
    }
}

/// Keeps the passed function always hooked, calling the interceptor every time the original function is called
///
/// # Parameters
///
/// * **hook_fn**: Name of the hook function that is generated
/// * **interceptor**: Interceptor function to call every time the original function is called
/// * **hook_name**: Name of the function which hooks the original function
/// * **restore_name**: Name of the restore function
/// * **addr**: Address of the original function
/// * **order**: `intercept before original` or `intercept after original`, defaults to the former
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
            asm!("call rax" :: "{rax}"($interceptor as usize) :: "intel","volatile");
            // restore original function
            asm!("call rax" :: "{rax}"($restore_name as usize) :: "intel","volatile");
            alignstack_post!();
            popall!();

            // call original function
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($addr) :: "intel","volatile");
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
    ($hook_fn:ident, $interceptor:path, $hook_name:path, $restore_name:path, $addr:expr, intercept after original) => {
        #[naked]
        unsafe extern fn $hook_fn() -> ! {
            // restore original function
            pushall!();
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($restore_name as usize) :: "intel","volatile");
            alignstack_post!();
            popall!();

            // call original function
            alignstack_pre!();
            asm!("call rax" :: "{rax}"($addr) :: "intel","volatile");
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