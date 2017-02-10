use std::process::Command;
use std::ffi::CString;
use std::ptr::null_mut;
use std::mem;

use kernel32::{
    CreateRemoteThread,
    OpenProcess,
    GetProcAddress,
    GetModuleHandleA,
    VirtualAllocEx,
    WriteProcessMemory,
    CloseHandle,
};
use winapi::winnt::{PROCESS_ALL_ACCESS, MEM_RESERVE, MEM_COMMIT, PAGE_READWRITE};

// http://resources.infosecinstitute.com/using-createremotethread-for-dll-injection-on-windows/
pub fn inject() {
    unsafe {
        let lib = CString::new("C:\\Users\\black\\workspace\\refunct-tas\\lib\\target\\i686-pc-windows-msvc\\debug\\rtil.dll").unwrap();
        let pid = pidof();
        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        if handle.is_null() {
            panic!("Could not open process");
        }
        let kernel = CString::new("kernel32.dll").unwrap();
        let load = CString::new("LoadLibraryA").unwrap();
        let addr = GetProcAddress(GetModuleHandleA(kernel.as_ptr()), load.as_ptr());
        if addr.is_null() {
            panic!("Cannot find LoadLibraryA in kernel32.dll");
        }
        println!("Found LoadLibraryA at {:?}", addr);
        let arg = VirtualAllocEx(handle, null_mut(), lib.to_bytes().len() as u32, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
        if arg.is_null() {
            panic!("Cannot allocate memory in Refunct");
        }
        println!("Alloc'ed at {:?}", arg);
        let n = WriteProcessMemory(handle, arg, lib.as_ptr() as *const _, lib.to_bytes().len() as u32, null_mut());
        if n == 0 {
            panic!("Cannot write to Refunct");
        }
        let thread_id = CreateRemoteThread(handle, null_mut(), 0, Some(mem::transmute(addr)), arg, 0, null_mut());
        if thread_id.is_null() {
            panic!("Could not start remote thread");
        }
        println!("thread_id: {:?}", thread_id);
        CloseHandle(handle);
    }
}

fn pidof() -> u32 {
    let output = Command::new("wmic")
        .args(&["process", "where", "Name='Refunct-Win32-Shipping.exe'", "get", "ProcessId"])
        .output()
        .expect("Cannot get pid of Refunct");
    let s = String::from_utf8(output.stdout).expect("Output of pidof is not utf8");
    let mut lines = s.lines();
    assert_eq!(lines.next().map(|s| s.trim()), Some("ProcessId"), "could not get pid of Refunct");
    lines.next().expect("No line containing pid").trim().parse().expect("Pidof returned non-number")
}
