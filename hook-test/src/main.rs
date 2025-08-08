use std::arch::naked_asm;
use hook_test::{ArgsRef, Hook, IsaAbi, X86_64_SystemV};

fn main() {
    let hook = unsafe { Hook::create(test_function as usize, custom_hook::<X86_64_SystemV>) };

    test_function(1337);
    hook.enable();
    test_function(42);
    test_function(21);
}

fn custom_hook<IA: IsaAbi>(hook: &'static Hook<IA>, mut args: ArgsRef<'_, IA>) {
    let arg = args.without_this_pointer::<u32>();
    println!("from inside the hook; original argument: {arg}");
    hook.call_original_function(args.as_args());
    let arg = args.without_this_pointer::<u32>();
    println!("setting argument to 314");
    *arg = 314;
    hook.call_original_function(args.as_args());
    println!("disabling the hook within the hook");
    hook.disable();
}
#[cfg(target_pointer_width = "64")]
extern "C" fn print(val: u64) {
    println!("from inside hooked function: {val}");
}
#[unsafe(link_section = ".custom_section")]
#[cfg(target_pointer_width = "64")]
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
extern "thiscall" fn thiscall_function(this: *const (), _: u8, _: u16, _: u32) {

}

#[cfg(target_pointer_width = "32")]
extern "C" fn print(val: u32) {
    println!("{val:x}");
}

#[cfg(target_pointer_width = "32")]
#[unsafe(naked)]
extern "C" fn test_function() {
    naked_asm!(
        "push eax",
        "mov edi, [{test_function}+1-12]",
        "call {print}",
        "pop eax",
        "ret",
        test_function = sym test_function,
        print = sym print,
    )
}
