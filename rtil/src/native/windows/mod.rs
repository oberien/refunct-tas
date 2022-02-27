pub mod consts;

use std::ptr;
use std::mem;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
use winapi::um::winnt::{PAGE_READWRITE, PAGE_EXECUTE_READ, HANDLE, THREAD_ALL_ACCESS};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next};
use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
use winapi::um::processthreadsapi::{GetCurrentThreadId, GetCurrentProcessId, OpenThread, SuspendThread, ResumeThread};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::libloaderapi::GetModuleHandleA;

// https://www.unknowncheats.me/forum/general-programming-and-reversing/123333-demo-pure-rust-internal-coding.html
// Entry Point
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "stdcall" fn DllMain(module: u32, reason: u32, reserved: *mut c_void) {
    match reason {
        1 => ::initialize(),
        _ => ()
    }
}

pub struct ThreadHandles(Vec<HANDLE>);

impl Drop for ThreadHandles {
    fn drop(&mut self) {
        log!("closing thread handles");
        for handle in self.0.drain(..) {
            unsafe { CloseHandle(handle); }
        }
    }
}

pub fn suspend_threads() -> ThreadHandles {
    let handles = get_thread_handles_except_current();
    log!("Suspend threads");
    unsafe {
        for thread in handles.0.iter().copied() {
            log!("Suspending thread {:p}", thread);
            SuspendThread(thread);
        }
    }
    handles
}
pub fn resume_threads(handles: ThreadHandles) {
    log!("Resume threads");
    unsafe {
        for thread in handles.0.iter().copied() {
            log!("Resuming thread {:p}", thread);
            ResumeThread(thread);
        }
    }
}

// https://stackoverflow.com/a/16684288
fn get_thread_handles_except_current() -> ThreadHandles {
    log!("Getting thread handles");
    unsafe {
        //void DoSuspendThread(DWORD targetProcessId, DWORD targetThreadId)
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if handle == INVALID_HANDLE_VALUE {
            log!("Couldn't get snapshot-handle. No thread handle acquired.");
            return ThreadHandles(Vec::new());
        }
        let mut thread_handles = Vec::new();

        let mut te = THREADENTRY32::default();
        te.dwSize = mem::size_of::<THREADENTRY32>() as u32;
        if Thread32First(handle, &mut te) == FALSE {
            log!("Couldn't get first thread. No thread handle acquired.");
            CloseHandle(handle);
            return ThreadHandles(Vec::new());
        }

        let current_thread_id = GetCurrentThreadId();
        let current_process_id = GetCurrentProcessId();
        let offset = memoffset::offset_of!(THREADENTRY32, th32OwnerProcessID);
        let size = mem::size_of_val(&te.th32OwnerProcessID);

        loop {
            if te.dwSize >= (offset + size) as u32 {
                // we MUST check the processid because for some reason
                // > The TH32CS_SNAPTHREAD value always creates a system-wide snapshot even if a
                // > process identifier is passed to CreateToolhelp32Snapshot.
                if te.th32OwnerProcessID == current_process_id && te.th32ThreadID != current_thread_id {
                    let thread = OpenThread(THREAD_ALL_ACCESS, FALSE, te.th32ThreadID);
                    if !thread.is_null() {
                        thread_handles.push(thread);
                    }
                }
            }
            te.dwSize = mem::size_of_val(&te) as u32;
            if Thread32Next(handle, &mut te) == FALSE {
                break;
            }
        }
        CloseHandle(handle);
        ThreadHandles(thread_handles)
    }
}

pub fn base_address() -> usize {
    unsafe { GetModuleHandleA(ptr::null()) as usize }
}

macro_rules! find {
    ($($name:ident,)*) => {
        $(
            pub(in native) static mut $name: usize = 0;
        )*
        pub(in native) fn init() {
            let base = base_address();
            log!("Got Base address: {:#x}", base);
            unsafe {
                $(
                    $name = base + self::consts::$name;
                )*
            }
        }
    }
}

find! {
    FSLATEAPPLICATION_TICK,
    AMYCHARACTER_TICK,
    AMYCHARACTER_FORCEDUNCROUCH,
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
    FAPP_DELTATIME,
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
    FMEMORY_MALLOC,
    FMEMORY_FREE,
    FNAME_FNAME,
    AMYHUD_DRAWHUD,
    AHUD_DRAWLINE,
    AHUD_DRAWTEXT,
    AHUD_PROJECT,
    GWORLD,
    UWORLD_SPAWNACTOR,
    UWORLD_DESTROYACTOR,
    AMYCHARACTER_STATICCLASS,
    APAWN_SPAWNDEFAULTCONTROLLER,
}

pub(in native) fn make_rw(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut std::ffi::c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_READWRITE, &mut out); }
}

pub(in native) fn make_rx(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut std::ffi::c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_EXECUTE_READ, &mut out); }
}
