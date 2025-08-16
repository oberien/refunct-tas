use std::sync::atomic::{AtomicPtr, Ordering};
use hook::{IsaAbi, TypedHook};
use crate::native::{FSLATEAPPLICATION_TICK, FSLATEAPPLICATION_ONKEYDOWN, FSLATEAPPLICATION_ONKEYUP, FSLATEAPPLICATION_ONRAWMOUSEMOVE, REBO_DOESNT_START_SEMAPHORE, RefunctIsaAbi};

static SLATEAPP: AtomicPtr<FSlateApplicationUE> = AtomicPtr::new(std::ptr::null_mut());

pub enum FSlateApplicationUE {}

pub struct FSlateApplication {
    tick: TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE), ()>,
    onkeydown: TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE, i32, u32, bool), ()>,
    onkeyup: TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE, i32, u32, bool), ()>,
    onrawmousemove: TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE, i32, i32), ()>,
}

impl FSlateApplication {
    pub fn hook() -> FSlateApplication {
        unsafe {
            Self {
                tick: TypedHook::with_this_pointer(FSLATEAPPLICATION_TICK.load(Ordering::Relaxed), tick_hook::<RefunctIsaAbi>).enabled(),
                onkeydown: TypedHook::with_this_pointer(FSLATEAPPLICATION_ONKEYDOWN.load(Ordering::Relaxed), on_key_down_hook::<RefunctIsaAbi>).enabled(),
                onkeyup: TypedHook::with_this_pointer(FSLATEAPPLICATION_ONKEYUP.load(Ordering::Relaxed), on_key_up_hook::<RefunctIsaAbi>).enabled(),
                onrawmousemove: TypedHook::with_this_pointer(FSLATEAPPLICATION_ONRAWMOUSEMOVE.load(Ordering::Relaxed), on_raw_mouse_move_hook::<RefunctIsaAbi>).enabled(),
            }
        }
    }

    fn get_this_pointer(fn_name: &str) -> *mut FSlateApplicationUE {
        let slateapp = SLATEAPP.load(Ordering::SeqCst);
        if slateapp.is_null() {
            let msg = format!("called FSlateApplication::{fn_name} while FSlateApplication-pointer wasn't initialized yet");
            log!("{}", msg);
            panic!("{}", msg);
        }
        slateapp
    }

    pub fn press_key(&self, key: i32, code: u32, repeat: bool) {
        unsafe { self.onkeydown.call_original_function((FSlateApplication::get_this_pointer("on_key_down"), key, code, repeat)); }
    }

    pub fn release_key(&self, key: i32, code: u32, repeat: bool) {
        unsafe { self.onkeyup.call_original_function((FSlateApplication::get_this_pointer("on_key_up"), key, code, repeat)); }
    }

    pub fn move_mouse(&self, x: i32, y: i32) {
        unsafe { self.onrawmousemove.call_original_function((FSlateApplication::get_this_pointer("on_raw_mouse_move"), x, y)); }
    }
}

pub fn tick_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut FSlateApplicationUE), ()>, this: *mut FSlateApplicationUE) {
    #[cfg(unix)] { SLATEAPP.store(this, Ordering::SeqCst); }
    #[cfg(windows)] {
        let this_addr = this as usize;
        // don't ask why this offset is needed, it's there since Feb 2017
        // introduced in 882dc51a5345deb50f3166a4ce4855133c993fb8
        // and it works, so don't touch it
        let this_fixed_addr = this_addr + 0x3c;
        SLATEAPP.store(this_fixed_addr as *mut _, Ordering::SeqCst);
    }
    log!("Got FSlateApplication: {:#x}", this as usize);
    hook.disable();
    unsafe { hook.call_original_function(this); }
    REBO_DOESNT_START_SEMAPHORE.release();
}

pub fn on_key_down_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut FSlateApplicationUE,i32,u32,bool), ()>, this: *mut FSlateApplicationUE, key_code: i32, character_code: u32, is_repeat: bool) {
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        crate::threads::ue::key_down(key_code & !(1<<30), character_code, is_repeat);
    }
    #[cfg(windows)] {
        crate::threads::ue::key_down(key_code, character_code, is_repeat);
    }
    unsafe { hook.call_original_function((this, key_code, character_code, is_repeat)); }
}

pub fn on_key_up_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut FSlateApplicationUE, i32, u32, bool), ()>, this: *mut FSlateApplicationUE, key_code: i32, character_code: u32, is_repeat: bool) {
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        crate::threads::ue::key_up(key_code & !(1 << 30), character_code, is_repeat);
    }
    #[cfg(windows)] {
        crate::threads::ue::key_up(key_code, character_code, is_repeat);
    }
    unsafe { hook.call_original_function((this, key_code, character_code, is_repeat)); }
}

fn on_raw_mouse_move_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut FSlateApplicationUE, i32, i32), ()>, this: *mut FSlateApplicationUE, x: i32, y: i32) {
    crate::threads::ue::mouse_move(x, y);
    unsafe { hook.call_original_function((this, x, y)); }
}
