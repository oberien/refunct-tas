/// Push all registers including xmm0-7
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
        " :::: "intel","volatile");
    }}
}

/// Pop all registers including xmm0-7
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
            static ref ORIGINAL: Static<[u8; 7]> = Static::new();
        }

        pub(in native) fn $hook_name() {
            if $log { log!("Hooking {}", $orig_name); }
            let addr = unsafe { $orig_addr };
            ::native::make_rw(addr);
            let hook_fn = $hook_fn as *const () as usize;
            let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) };
            let mut saved = [0u8; 7];
            saved[..].copy_from_slice(slice);
            ORIGINAL.set(saved);
            if $log { log!("Original {}: {:?}", $orig_name, slice); }
            // mov eax, addr
            slice[0] = 0xb8;
            (&mut slice[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
            // jmp rax
            slice[5..].copy_from_slice(&[0xff, 0xe0]);
            if $log { log!("Injected {:?}", slice); }
            ::native::make_rx(addr);
            if $log { log!("{} hooked successfully", $orig_name); }
        }

        pub(in native) fn $unhook_name() {
            if $log { log!("Unhooking {}", $orig_name); }
            let addr = unsafe { $orig_addr };
            ::native::make_rw(addr);
            let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) };
            slice[..].copy_from_slice(&*ORIGINAL.get());
            ::native::make_rx(addr);
            if $log { log!("{} unhooked successfully", $orig_name) }
        }
    };
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
        unsafe extern "thiscall" fn $hook_fn() -> ! {
            // save registers
            pushall!();
            // call interceptor
            asm!("call eax" :: "{eax}"($interceptor as usize) :: "intel","volatile");
            // unhook original function
            asm!("call eax" :: "{eax}"($unhook_name as usize) :: "intel","volatile");
            // restore registers
            popall!();
            // jump to original function
            asm!("jmp eax" :: "{eax}"($orig_addr) :: "intel","volatile");
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
        unsafe extern "thiscall" fn $hook_fn() -> ! {
            pushall!();
            // call interceptor
            asm!("call $0" :: "i"($interceptor as usize) :: "intel","volatile");
            // restore original function
            asm!("call $0" :: "i"($unhook_name as usize) :: "intel","volatile");
            popall!();

            // call original function
            asm!("call eax" :: "{eax}"($orig_addr) :: "intel","volatile");

            // save eax (return value of original function)
            asm!("push eax" :::: "intel","volatile");

            // hook method again
            asm!("call $0" :: "i"($hook_name as usize) :: "intel","volatile");

            // restore eax
            asm!("pop eax" :::: "intel","volatile");

            // return to original caller
            asm!("ret" :::: "intel","volatile");
            ::std::intrinsics::unreachable()
        }
    };
    ($hook_fn:ident, $interceptor:path, $hook_name:path, $unhook_name:path, $orig_addr:expr, intercept after original,) => {
        #[naked]
        unsafe extern "thiscall" fn $hook_fn() -> ! {
            // restore original function
            pushall!();
            asm!("call $0" :: "i"($unhook_name as usize) :: "intel","volatile");
            popall!();

            // call original function
            asm!("call eax" :: "{eax}"($orig_addr) :: "intel","volatile");

            // save eax (return value of original function)
            asm!("push eax" :::: "intel","volatile");

            // hook method again
            asm!("call $0" :: "i"($hook_name as usize) :: "intel","volatile");
            // call interceptor
            asm!("call $0" :: "i"($interceptor as usize) :: "intel","volatile");

            // restore eax
            asm!("pop eax" :::: "intel","volatile");

            // return to original caller
            asm!("ret" :::: "intel","volatile");
            ::std::intrinsics::unreachable()
        }
    }
}