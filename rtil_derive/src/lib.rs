#![recursion_limit="256"]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, Ident, token::Colon2, Result, Abi, LitStr, token::Extern};
use syn::parse::{Parse, ParseStream};

struct StrTokens<'a>(&'a str);

impl<'a> ToTokens for StrTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(self.0.parse::<TokenStream2>().unwrap())
    }
}

struct Attrs {
    /// demangled name of the original function
    original_function_name: String,
    /// name of the variable in which the address of the original function address is stored
    original_function_address: Ident,
    /// name of the static backup of the original overwritten bytes for unhooking
    original_bytes_backup_name: Ident,
    /// name of function hooking the original function
    hook_function_name: Ident,
    /// name of function unhooking the original function
    unhook_function_name: Ident,
    /// name of the internal interceptor called by the hook, which calls the function_to_call
    interceptor_name: Ident,
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let class = input.parse::<Ident>()?;
        let _colon = input.parse::<Colon2>()?;
        let function = input.parse::<Ident>()?;
        let class_lower = class.to_string().to_lowercase();
        let function_lower = function.to_string().to_lowercase();
        let class_upper = class.to_string().to_uppercase();
        let function_upper = function.to_string().to_uppercase();
        fn ident(s: String) -> Ident {
            let s = Box::leak(Box::new(s));
            Ident::new(s, Span::call_site())
        }
        Ok(Attrs {
            original_function_name: format!("{}::{}", class, function),
            original_function_address: ident(format!("{}_{}", class_upper, function_upper)),
            original_bytes_backup_name: ident(format!("ORIGINAL_{}_{}_BYTES", class_upper, function_upper)),
            hook_function_name: ident(format!("hook_{}_{}", class_lower, function_lower)),
            unhook_function_name: ident(format!("unhook_{}_{}", class_lower, function_lower)),
            interceptor_name: ident(format!("intercept_{}_{}", class_lower, function_lower)),
        })
    }
}

#[proc_macro_attribute]
pub fn hook_once(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = parse_macro_input!(attr as Attrs);
    let function_to_call = &input.sig.ident;
    let item = generate_item(&input);
    let hook_unhook = generate_hook_unhook(&attrs, true);
    let hook_once = generate_hook_once(&attrs, function_to_call);

    (quote! {
        #item
        #hook_once
        #hook_unhook
    }).into()
}

#[proc_macro_attribute]
pub fn hook_before(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = parse_macro_input!(attr as Attrs);
    let function_to_call = &input.sig.ident;
    let item = generate_item(&input);
    let hook_unhook = generate_hook_unhook(&attrs, false);
    let hook_before = generate_hook_before(&attrs, function_to_call);

    (quote! {
        #item
        #hook_before
        #hook_unhook
    }).into()
}

#[proc_macro_attribute]
pub fn hook_after(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = parse_macro_input!(attr as Attrs);
    let function_to_call = &input.sig.ident;
    let item = generate_item(&input);
    let hook_unhook = generate_hook_unhook(&attrs, false);
    let hook_after = generate_hook_after(&attrs, function_to_call);

    (quote! {
        #item
        #hook_after
        #hook_unhook
    }).into()
}

fn generate_item(input: &ItemFn) -> TokenStream2 {
    fn abi(abi: &str) -> Abi {
        Abi {
            extern_token: Extern { span: Span::call_site() },
            name: Some(LitStr::new(abi, Span::call_site())),
        }
    }
    let mut input_linux = input.clone();
    let mut input_windows = input.clone();
    input_linux.sig.abi = Some(abi("C"));
    input_windows.sig.abi = Some(abi("thiscall"));
    quote! {
        #[cfg(unix)]
        #input_linux

        #[cfg(windows)]
        #input_windows
    }
}

fn generate_hook_once(attrs: &Attrs, function_to_call: &Ident) -> TokenStream2 {
    let original_function_address = &attrs.original_function_address;
    let unhook_function_name = &attrs.unhook_function_name;
    let interceptor_name = &attrs.interceptor_name;

    quote! {
        #[cfg(unix)]
        #[naked]
        unsafe extern "C" fn #interceptor_name() -> ! {
            std::arch::asm!(
                // push arguments
                #PUSHALL_LINUX,
                #ALIGNSTACK_PRE_LINUX,
                // call function_to_call
                concat!("call {", stringify!(#function_to_call), "}"),
                // restore original function
                concat!("call {", stringify!(#unhook_function_name), "}"),
                #ALIGNSTACK_POST_LINUX,
                // restore register
                #POPALL_LINUX,
                // jump to original function
                concat!("mov rax, [rip+{", stringify!(#original_function_address), "}]"),
                "jmp rax",

                #function_to_call = sym #function_to_call,
                #unhook_function_name = sym #unhook_function_name,
                #original_function_address = sym crate::native::#original_function_address,
                options(noreturn),
            )
        }

        // only allows inspection of first argument (this*)
        #[cfg(windows)]
        #[naked]
        unsafe extern "thiscall" fn #interceptor_name() -> ! {
            std::arch::asm!(
                #PUSHALL_WINDOWS,
                concat!("call {", stringify!(#function_to_call), "}"),
                // unhook original function
                concat!("call {", stringify!(#unhook_function_name), "}"),
                // restore registers
                #POPALL_WINDOWS,
                // jump to original function
                concat!("mov eax, [{", stringify!(#original_function_address), "}]"),
                "jmp eax",

                #function_to_call = sym #function_to_call,
                #unhook_function_name = sym #unhook_function_name,
                #original_function_address = sym crate::native::#original_function_address,
                options(noreturn),
            )
        }
    }
}

fn generate_hook_before(attrs: &Attrs, function_to_call: &Ident) -> TokenStream2 {
    let original_function_address = &attrs.original_function_address;
    let hook_function_name = &attrs.hook_function_name;
    let unhook_function_name = &attrs.unhook_function_name;
    let interceptor_name = &attrs.interceptor_name;

    quote! {
        #[cfg(unix)]
        #[naked]
        unsafe extern "C" fn #interceptor_name() -> ! {
            std::arch::asm!(
                #PUSHALL_LINUX,
                #ALIGNSTACK_PRE_LINUX,
                // call function_to_call
                concat!("call {", stringify!(#function_to_call), "}"),
                // restore original function
                concat!("call {", stringify!(#unhook_function_name), "}"),
                #ALIGNSTACK_POST_LINUX,
                #POPALL_LINUX,

                // call original function
                #ALIGNSTACK_PRE_LINUX,
                concat!("mov rax, [rip+{", stringify!(#original_function_address), "}]"),
                "call rax",
                #ALIGNSTACK_POST_LINUX,

                // save rax (return value of original function)
                "push rax",

                // hook method again
                #ALIGNSTACK_PRE_LINUX,
                concat!("call {", stringify!(#hook_function_name), "}"),
                #ALIGNSTACK_POST_LINUX,

                // restore rax
                "pop rax",
                // return to original caller
                "ret",

                #function_to_call = sym #function_to_call,
                #unhook_function_name = sym #unhook_function_name,
                #original_function_address = sym crate::native::#original_function_address,
                #hook_function_name = sym #hook_function_name,
                options(noreturn),
            )
        }

//        #[cfg(windows)]
//        unsafe extern "thiscall" fn print_stack(esp: usize) {
//            let stack = ::std::slice::from_raw_parts(esp as *const u8, 0x100);
//            log!("{:#x}:", esp);
//            log!("    xmm7: {:?}", &stack[0..0x10]);
//            log!("    xmm6: {:?}", &stack[0x10..0x20]);
//            log!("    xmm5: {:?}", &stack[0x20..0x30]);
//            log!("    xmm4: {:?}", &stack[0x30..0x40]);
//            log!("    xmm3: {:?}", &stack[0x40..0x50]);
//            log!("    xmm2: {:?}", &stack[0x50..0x60]);
//            log!("    xmm1: {:?}", &stack[0x60..0x70]);
//            log!("    xmm0: {:?}", &stack[0x70..0x80]);
//            log!("    ebp : {:?}", &stack[0x80..0x84]);
//            log!("    edi : {:?}", &stack[0x84..0x88]);
//            log!("    esi : {:?}", &stack[0x88..0x8c]);
//            log!("    edx : {:?}", &stack[0x8c..0x90]);
//            log!("    ecx : {:?}", &stack[0x90..0x94]);
//            log!("    ebx : {:?}", &stack[0x94..0x98]);
//            log!("    eax : {:?}", &stack[0x98..0x9c]);
//            log!("    ret : {:?}", &stack[0x9c..0xa0]);
//            log!("    arg1: {:?}", &stack[0xa0..0xa4]);
//            log!("    arg2: {:?}", &stack[0xa4..0xa8]);
//            log!("    arg3: {:?}", &stack[0xa8..0xac]);
//            log!("    arg4: {:?}", &stack[0xac..0xb0]);
//            log!("    rest: {:?}", &stack[0xb0..]);
//        }

        #[cfg(windows)]
        #[naked]
        unsafe extern "thiscall" fn #interceptor_name() -> ! {
            std::arch::asm!(
                // We need to duplicate the arguments and delete the return address for ours to
                // be located correctly when using `call`.
                // Stack Layout:
                // esp    xmm7    10         \
                // +10    xmm6    10          |
                // +20    xmm5    10          |
                // +30    xmm4    10          |
                // +40    xmm3    10          |
                // +50    xmm2    10          |
                // +60    xmm1    10          |
                // +70    xmm0    10           > 0xa0
                // +80    ebp      4          |
                // +84    edi      4          |
                // +88    esi      4          |
                // +8c    edx      4          |
                // +90    ecx      4          |
                // +94    ebx      4          |
                // +98    eax      4          |
                // +9c    ret      4         /
                // +a0    args
                //        caller stack frame
                // We assume that there aren't more than 0x100-0xa0 = 0x60 bytes of arguments.

                // save all registers
                #PUSHALL_WINDOWS,
                // Reserve some stack which we copy everything into.
                "sub esp, 0x100",
                "mov ecx, 0x100",
                "lea esi, [esp + 0x100]",
                "mov edi, esp",
                "rep movsb",
                // restore copied registers
                #POPALL_WINDOWS,
                // remove old return address, which will be replaced by our `call`
                "pop eax",
                // save current stack pointer in non-volatile register to find out
                // how many arguments are cleared, which we use to adjust the stack back
                "mov ebx, esp",


                // call function_to_call
                concat!("call {", stringify!(#function_to_call), "}"),
                // get consumed stack (negative value)
                "sub ebx, esp",

                // restore original function
                concat!("call {", stringify!(#unhook_function_name), "}"),
                // restore stack
                "add esp, 0x60",
                "add esp, ebx",


                // copy stack again and do the same with the original function
                "sub esp, 0x100",
                "mov ecx, 0x100",
                "lea esi, [esp + 0x100]",
                "mov edi, esp",
                "rep movsb",
                // restore registers
                #POPALL_WINDOWS,
                // pop return address
                "pop eax",
                // save stack pointer
                "mov ebx, esp",
                // call original function
                concat!("mov eax, [{", stringify!(#original_function_address), "}]"),
                "call eax",

                // get consumed stack (negative value)
                "sub ebx, esp",
                // restore stack
                "add esp, 0x60",
                "add esp, ebx",

                // save eax (return value of original function) to pushed registers
                "mov [esp + 0x98], eax",
                // save consumed stack to ecx in the pushed registers, so we can consume as much
                // after popping the registers before returning
                "mov [esp + 0x90], ebx",
                // move original return address to correct position after arg-consumption
                "mov eax, [esp + 0x9c]",
                "lea edx, [esp + 0x9c]",
                "sub edx, ebx",
                "mov [edx], eax",

                // hook method again
                concat!("call {", stringify!(#hook_function_name), "}"),

                // restore all registers
                #POPALL_WINDOWS,
                // do not pop old return address, because we wrote the return address to the last argument
                // consume arguments
                "sub esp, ecx",

                // return to original caller
                "ret",

                #function_to_call = sym #function_to_call,
                #unhook_function_name = sym #unhook_function_name,
                #original_function_address = sym crate::native::#original_function_address,
                #hook_function_name = sym #hook_function_name,
                options(noreturn),
            )
        }
    }
}

fn generate_hook_after(attrs: &Attrs, function_to_call: &Ident) -> TokenStream2 {
    let original_function_address = &attrs.original_function_address;
    let hook_function_name = &attrs.hook_function_name;
    let unhook_function_name = &attrs.unhook_function_name;
    let interceptor_name = &attrs.interceptor_name;

    quote! {
        #[cfg(unix)]
        #[naked]
        unsafe extern "C" fn #interceptor_name() -> ! {
            std::arch::asm!(
                // restore original function
                #PUSHALL_LINUX,
                #ALIGNSTACK_PRE_LINUX,
                concat!("call {", stringify!(#unhook_function_name), "}"),
                #ALIGNSTACK_POST_LINUX,
                #POPALL_LINUX,

                // call original function
                #ALIGNSTACK_PRE_LINUX,
                concat!("mov rax, [rip+{", stringify!(#original_function_address), "}]"),
                "call rax",
                #ALIGNSTACK_POST_LINUX,

                // save rax (return value of original function
                "push rax",

                #ALIGNSTACK_PRE_LINUX,
                // hook method again
                concat!("call {", stringify!(#hook_function_name), "}"),
                // call function_to_call
                concat!("call {", stringify!(#function_to_call), "}"),
                #ALIGNSTACK_POST_LINUX,

                // restore rax
                "pop rax",

                // return to original caller
                "ret",

                #unhook_function_name = sym #unhook_function_name,
                #original_function_address = sym crate::native::#original_function_address,
                #hook_function_name = sym #hook_function_name,
                #function_to_call = sym #function_to_call,
                options(noreturn),
            )
        }

        #[cfg(windows)]
        #[naked]
        unsafe extern "thiscall" fn #interceptor_name() -> ! {
            std::arch::asm!(
                // restore original function
                #PUSHALL_WINDOWS,
                concat!("call {", stringify!(#unhook_function_name), "}"),
                #POPALL_WINDOWS,

                // call original function
                concat!("mov eax, [{", stringify!(#original_function_address), "}]"),
                "call eax",

                // save eax (return value of original function)
                "push eax",

                // hook method again
                concat!("call {", stringify!(#hook_function_name), "}"),
                // call function_to_call
                concat!("call {", stringify!(#function_to_call), "}"),

                // restore eax
                "pop eax",

                // return to original caller
                "ret",

                #unhook_function_name = sym #unhook_function_name,
                #original_function_address = sym crate::native::#original_function_address,
                #hook_function_name = sym #hook_function_name,
                #function_to_call = sym #function_to_call,
                options(noreturn),
            )
        }
    }
}

fn generate_hook_unhook(attrs: &Attrs, log: bool) -> TokenStream2 {
    let original_function_name = &attrs.original_function_name;
    let original_function_address = &attrs.original_function_address;
    let original_bytes_backup_name = &attrs.original_bytes_backup_name;
    let hook_function_name = &attrs.hook_function_name;
    let unhook_function_name = &attrs.unhook_function_name;
    let interceptor_name = &attrs.interceptor_name;

    quote! {
        #[cfg(unix)]
        static #original_bytes_backup_name: once_cell::sync::Lazy<crate::statics::Static<[u8; 12]>> = once_cell::sync::Lazy::new(|| crate::statics::Static::new());
        #[cfg(windows)]
        static #original_bytes_backup_name: once_cell::sync::Lazy<crate::statics::Static<[u8; 7]>> = once_cell::sync::Lazy::new(|| crate::statics::Static::new());

        #[cfg(unix)]
        pub extern "C" fn #hook_function_name() {
            use byteorder::{WriteBytesExt, LittleEndian};
            if #log { log!("Hooking {}", #original_function_name); }
            let addr = unsafe { crate::native::#original_function_address };
            crate::native::make_rw(addr);
            let interceptor_address = #interceptor_name as *const () as usize;
            let slice = unsafe { std::slice::from_raw_parts_mut(addr as *mut u8, 12) };
            let mut saved = [0u8; 12];
            saved[..].copy_from_slice(slice);
            #original_bytes_backup_name.set(saved);
            if #log { log!("Original {}: {:?}", #original_function_name, slice); }
            // mov rax, addr
            slice[..2].copy_from_slice(&[0x48, 0xb8]);
            (&mut slice[2..10]).write_u64::<LittleEndian>(interceptor_address as u64).unwrap();
            // jmp rax
            slice[10..].copy_from_slice(&[0xff, 0xe0]);
            if #log { log!("Injected Code: {:?}", slice); }
            crate::native::make_rx(addr);
            if #log { log!("{} successfully hooked", #original_function_name); }
        }

        #[cfg(windows)]
        pub extern "thiscall" fn #hook_function_name() {
            use byteorder::{WriteBytesExt, LittleEndian};
            if #log { log!("Hooking {}", #original_function_name); }
            let addr = unsafe { crate::native::#original_function_address };
            crate::native::make_rw(addr);
            let interceptor_address = #interceptor_name as *const () as usize;
            let slice = unsafe { std::slice::from_raw_parts_mut(addr as *mut u8, 7) };
            let mut saved = [0u8; 7];
            saved[..].copy_from_slice(slice);
            #original_bytes_backup_name.set(saved);
            if #log { log!("Original {}: {:?}", #original_function_name, slice); }
            // mov eax, addr
            slice[0] = 0xb8;
            (&mut slice[1..5]).write_u32::<LittleEndian>(interceptor_address as u32).unwrap();
            // jmp rax
            slice[5..].copy_from_slice(&[0xff, 0xe0]);
            if #log { log!("Injected {:?}", slice); }
            crate::native::make_rx(addr);
            if #log { log!("{} hooked successfully", #original_function_name); }
        }

        #[cfg(unix)]
        pub extern "C" fn #unhook_function_name() {
            if #log { log!("Restoring {}", #original_function_name); }
            let addr = unsafe { crate::native::#original_function_address };
            crate::native::make_rw(addr);
            let slice = unsafe { std::slice::from_raw_parts_mut(addr as *mut u8, 12) };
            slice[..].copy_from_slice(&*#original_bytes_backup_name.get());
            crate::native::make_rx(addr);
            if #log { log!("{} successfully restored", #original_function_name); }
        }

        #[cfg(windows)]
        pub extern "thiscall" fn #unhook_function_name() {
            if #log { log!("Unhooking {}", #original_function_name); }
            let addr = unsafe { crate::native::#original_function_address };
            crate::native::make_rw(addr);
            let slice = unsafe { std::slice::from_raw_parts_mut(addr as *mut u8, 7) };
            slice[..].copy_from_slice(&*#original_bytes_backup_name.get());
            crate::native::make_rx(addr);
            if #log { log!("{} unhooked successfully", #original_function_name) }
        }
    }
}


const PUSHALL_LINUX: &str = r#"
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
"#;
const POPALL_LINUX: &str = r#"
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
"#;
const ALIGNSTACK_PRE_LINUX: &str = r#"
    push rbp
    mov rbp, rsp
    and rsp, 0xfffffffffffffff0
"#;
const ALIGNSTACK_POST_LINUX: &str = r#"
    mov rsp, rbp
    pop rbp
"#;
const PUSHALL_WINDOWS: &str = r#"
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
"#;
const POPALL_WINDOWS: &str = r#"
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
"#;
