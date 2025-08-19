use std::arch::naked_asm;
use hook::{ArgsRef, RawHook, IsaAbi, TypedHook};
#[cfg(target_pointer_width = "64")]
use hook::X86_64_SystemV;
#[cfg(target_pointer_width = "32")]
use hook::I686_MSVC_Thiscall;

#[cfg(target_pointer_width = "64")]
type CurrentIsaAbi = X86_64_SystemV;
#[cfg(target_pointer_width = "32")]
type CurrentIsaAbi = I686_MSVC_Thiscall;

fn main() {
    // let hook = unsafe { Hook::create(test_function as usize, custom_raw_hook::<CurrentIsaAbi, ()>) };
    let hook = unsafe { TypedHook::create(test_function as usize, custom_typed_hook::<CurrentIsaAbi>) };

    test_function(1337);
    hook.enable();
    unsafe { hook.call_original_function(69) };
    test_function(42);
    test_function(21);
}

fn custom_typed_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(u32), ()>, arg: u32) {
    println!("from inside the hook; original argument: {arg}");
    unsafe { hook.call_original_function(arg) };
    println!("calling with argument 314");
    unsafe { hook.call_original_function(314) };
    println!("disabling the hook within the hook");
    hook.disable();
}

fn custom_raw_hook<IA: IsaAbi, T>(hook: &'static RawHook<IA, T>, mut args: ArgsRef<'_, IA>) {
    let arg = args.load::<u32>();
    println!("from inside the hook; original argument: {arg}");
    unsafe { hook.call_original_function(&args) };
    let arg = args.load::<u32>();
    println!("setting argument to 314");
    *arg = 314;
    unsafe { hook.call_original_function(&args) };
    println!("disabling the hook within the hook");
    hook.disable();
}
#[cfg(target_pointer_width = "64")]
extern "C" fn print(val: u64) {
    println!("from inside hooked function: {val}");
}
#[cfg(target_pointer_width = "32")]
extern "fastcall" fn print(val: u32) {
    println!("from inside hooked function: {val}");
}
#[cfg(target_pointer_width = "64")]
#[unsafe(link_section = ".custom_section")]
#[unsafe(naked)]
extern "C" fn test_function(_arg: u32) {
    naked_asm!(
        "push rbp",
        "jmp 5f",
        "mov rax,[rip-12]",
        "mov rbp, rsp",
        "call {print}",
        "mov rax, [rip+12]",
        "add rax, 0x50000000",
        "add rax, 0x50000000",
        "5:",
        "call {print}",
        "add rax, 0x50000000",
        "mov rax, [rip+12]",
        "pop rbp",
        "ret",
        print = sym print,
    )
}
#[cfg(target_pointer_width = "32")]
#[unsafe(link_section = ".custom_section")]
#[unsafe(naked)]
extern "thiscall" fn test_function(_arg: u32) {
    naked_asm!(
        "push eax",
        "jmp 5f",
        "mov ecx, [eax+12]",
        "5:",
        "call {print}",
        "pop eax",
        "ret",
        print = sym print,
    )
}
