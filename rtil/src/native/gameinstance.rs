use std::sync::atomic::Ordering;
use crate::native::{UWorld, LevelState};
use crate::native::linux::{AACTOR_PROCESSEVENT, UOBJECT_FINDFUNCTION, UOBJECT_PROCESSEVENT};
use crate::native::ue::FName;

#[repr(C)]
pub struct UMyGameInstance {
    #[cfg(windows)]
    pad: [u8; 0x9c],
    #[cfg(unix)]
    pad: [u8; 0x134],
    level_state: LevelState,
}
impl UMyGameInstance {
    pub fn get_umygameinstance() -> *mut UMyGameInstance {
        UWorld::get_umygameinstance()
    }
    pub fn get_levelstate() -> *mut LevelState {
        unsafe {
            &mut (*Self::get_umygameinstance()).level_state as *mut LevelState
        }
    }
    pub fn trigger_new_game() {
        let fun: extern_fn!(fn(this: *mut UMyGameInstance, name: FName) -> *const ()) =
            unsafe { ::std::mem::transmute(UOBJECT_FINDFUNCTION.load(Ordering::SeqCst)) };
        let ufunction = fun(UMyGameInstance::get_umygameinstance(), FName::from("NewGame"));
        struct Foo { x: i32 }
        let foo = Foo { x: 1337 };
        log!("{ufunction:p}");
        let fun: extern_fn!(fn(this: *mut UMyGameInstance, function: *const (), args: *const ())) =
            unsafe { ::std::mem::transmute(AACTOR_PROCESSEVENT.load(Ordering::SeqCst)) };
        fun(UMyGameInstance::get_umygameinstance(), ufunction, &foo as *const _ as *const ());
    }
}

