use std::sync::atomic::{AtomicPtr, Ordering};
use hook::{ArgsRef, IsaAbi, RawHook, TypedHook};
use crate::native::{FSLATEAPPLICATION_TICK, FSLATEAPPLICATION_ONKEYDOWN, FSLATEAPPLICATION_ONKEYUP, FSLATEAPPLICATION_ONRAWMOUSEMOVE, REBO_DOESNT_START_SEMAPHORE, RefunctIsaAbi, FSLATEAPPLICATION_ONMOUSEDOUBLECLICK, FSLATEAPPLICATION_ONMOUSEDOWN, FSLATEAPPLICATION_ONMOUSEMOVE, FSLATEAPPLICATION_ONMOUSEUP, FSLATEAPPLICATION_ONMOUSEWHEEL};

static SLATEAPP: AtomicPtr<FSlateApplicationUE> = AtomicPtr::new(std::ptr::null_mut());

pub enum FSlateApplicationUE {}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum EMouseButtonsType {
    Left,
    Middle,
    Right,
    Thumb01,
    Thumb02,
    Invalid,
}
impl EMouseButtonsType {
    pub fn from_u32(button: u32) -> Self {
        match button {
            0 => EMouseButtonsType::Left,
            1 => EMouseButtonsType::Middle,
            2 => EMouseButtonsType::Right,
            3 => EMouseButtonsType::Thumb01,
            4 => EMouseButtonsType::Thumb02,
            _ => EMouseButtonsType::Invalid,
        }
    }
    pub fn to_iced_button(self) -> iced::mouse::Button {
        match self {
            EMouseButtonsType::Left => iced::mouse::Button::Left,
            EMouseButtonsType::Middle => iced::mouse::Button::Middle,
            EMouseButtonsType::Right => iced::mouse::Button::Right,
            EMouseButtonsType::Thumb01 => iced::mouse::Button::Forward,
            EMouseButtonsType::Thumb02 => iced::mouse::Button::Back,
            EMouseButtonsType::Invalid => iced::mouse::Button::Other(u16::MAX),
        }
    }
}

pub struct FSlateApplication {
    _tick: &'static TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE), ()>,
    onkeydown: &'static TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE, i32, u32, bool), ()>,
    onkeyup: &'static TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE, i32, u32, bool), ()>,
    onrawmousemove: &'static TypedHook<RefunctIsaAbi, fn(*mut FSlateApplicationUE, i32, i32), ()>,
    _onmousemove: &'static RawHook<RefunctIsaAbi, ()>,
    _onmousedown: &'static RawHook<RefunctIsaAbi, ()>,
    _onmousedoubleclick: &'static RawHook<RefunctIsaAbi, ()>,
    _onmouseup: &'static RawHook<RefunctIsaAbi, ()>,
    _onmousewheel: &'static RawHook<RefunctIsaAbi, ()>,
}

impl FSlateApplication {
    pub(super) fn hook() -> FSlateApplication {
        unsafe {
            Self {
                _tick: TypedHook::create(FSLATEAPPLICATION_TICK.load(Ordering::Relaxed), tick_hook).enabled(),
                onkeydown: TypedHook::create(FSLATEAPPLICATION_ONKEYDOWN.load(Ordering::Relaxed), on_key_down_hook).enabled(),
                onkeyup: TypedHook::create(FSLATEAPPLICATION_ONKEYUP.load(Ordering::Relaxed), on_key_up_hook).enabled(),
                onrawmousemove: TypedHook::create(FSLATEAPPLICATION_ONRAWMOUSEMOVE.load(Ordering::Relaxed), on_raw_mouse_move_hook).enabled(),
                _onmousemove: RawHook::create(FSLATEAPPLICATION_ONMOUSEMOVE.load(Ordering::Relaxed), on_mouse_move_hook).enabled(),
                _onmousedown: RawHook::create(FSLATEAPPLICATION_ONMOUSEDOWN.load(Ordering::Relaxed), on_mouse_down_hook).enabled(),
                _onmousedoubleclick: RawHook::create(FSLATEAPPLICATION_ONMOUSEDOUBLECLICK.load(Ordering::Relaxed), on_mouse_double_click_hook).enabled(),
                _onmouseup: RawHook::create(FSLATEAPPLICATION_ONMOUSEUP.load(Ordering::Relaxed), on_mouse_up_hook).enabled(),
                _onmousewheel: RawHook::create(FSLATEAPPLICATION_ONMOUSEWHEEL.load(Ordering::Relaxed), on_mouse_wheel_hook).enabled(),
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

fn tick_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut FSlateApplicationUE), ()>, this: *mut FSlateApplicationUE) {
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

fn on_key_down_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut FSlateApplicationUE,i32,u32,bool), ()>, this: *mut FSlateApplicationUE, key_code: i32, character_code: u32, is_repeat: bool) {
    #[cfg(unix)] {
        crate::threads::ue::key_down(key_code, character_code, is_repeat);
    }
    #[cfg(windows)] {
        crate::threads::ue::key_down(key_code, character_code, is_repeat);
    }
    unsafe { hook.call_original_function((this, key_code, character_code, is_repeat)); }
}

fn on_key_up_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut FSlateApplicationUE, i32, u32, bool), ()>, this: *mut FSlateApplicationUE, key_code: i32, character_code: u32, is_repeat: bool) {
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
fn on_mouse_move_hook<IA: IsaAbi>(hook: &RawHook<IA, ()>, args: ArgsRef<'_, IA>) {
    crate::threads::ue::mouse_move(0, 0);
    unsafe { hook.call_original_function(args); }
}
fn on_mouse_down_hook<IA: IsaAbi>(hook: &RawHook<IA, ()>, mut args: ArgsRef<'_, IA>) {
    let (_this, _window, button) = args.load::<(*mut FSlateApplicationUE, *mut (), u32)>();
    let button = EMouseButtonsType::from_u32(*button);
    crate::threads::ue::mouse_button_down(button);
    unsafe { hook.call_original_function(args); }
}
fn on_mouse_double_click_hook<IA: IsaAbi>(hook: &RawHook<IA, ()>, mut args: ArgsRef<'_, IA>) {
    let (_this, _window, button) = args.load::<(*mut FSlateApplicationUE, *mut (), u32)>();
    let button = EMouseButtonsType::from_u32(*button);
    crate::threads::ue::mouse_button_down(button);
    unsafe { hook.call_original_function(args); }
}
fn on_mouse_up_hook<IA: IsaAbi>(hook: &RawHook<IA, ()>, mut args: ArgsRef<'_, IA>) {
    let (_this, button) = args.load::<(*mut FSlateApplicationUE, u32)>();
    let button = EMouseButtonsType::from_u32(*button);
    crate::threads::ue::mouse_button_up(button);
    unsafe { hook.call_original_function(args); }
}
fn on_mouse_wheel_hook<IA: IsaAbi>(hook: &RawHook<IA, ()>, mut args: ArgsRef<'_, IA>) {
    let (_this, delta) = args.load::<(*mut FSlateApplicationUE, f32)>();
    crate::threads::ue::mouse_wheel(*delta);
    unsafe { hook.call_original_function(args); }
}
