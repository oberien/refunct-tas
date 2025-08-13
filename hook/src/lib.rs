use std::{mem, slice};
use iced_x86::{BlockEncoder, BlockEncoderOptions, IcedError, Instruction, InstructionBlock};
use crate::function_decoder::FunctionDecoder;
use crate::hook_memory_page::HookMemoryPageBuilder;
use crate::isa_abi::Array;

mod args;
mod function_decoder;
mod trampoline;
mod isa_abi;
mod hook_memory_page;

pub use args::{ArgsRef, ArgsBoxed};
pub use isa_abi::{IsaAbi, X86_64_SystemV};
use crate::args::{Args, LoadFromArgs, StoreToArgs};

// +------------+
// | caller of  |    +-------------------+
// | now hooked |    | original function |
// | function   |    +-------------------+
// +------------+      • (0)
//   ^     | (1)       •  first few instructions get overwritten
//   |     | call      •  now immediately jumps to our interceptor
//   |     '----.      •  it becomes the overwritten function        .-------------.
//   |          |      •                                             |             |
//   |          v      v                                             |             v
//   |     +-------------+  (2) immediately jump to interceptor      |      +-------------+
//   |     | overwritten |-------------------------------------------'      | interceptor |
//   |     |  function   |<-.                                               +-------------+  (12)
//   |     +-------------+  |                                             (3) |   ^     '---------.
//   |            | (8)     |                    store registers & arguments  |   |               |
//   |            |         |                         create the Args-struct  |   |  return to    |
//   |            |         |        call the abi_fixer using extern "C" ABI  |   |  interceptor  |
//   |            |         |                                                 v   | (11)          |
//   |            |         |                                             +-----------+           |
//   |            |         |                                             | abi_fixer |           |
//   |            |         |                                             +-----------+           |
//   |            |         |                                             (4) |   ^               |
//   |            |         |              call hook using extern "Rust" ABI  |   |               |
//   |            |         |                                                 |   '----.          |
//   |            |         |                                                 |        |          |
//   |            |         |              can call the trampoline in order   |        |          |
//   |            |         |                 to call the original function   v        |          |
//   |            |         |                  +-----------------+       (5) +------+  |          |
//   |            |         |         .--------| call_trampoline |<----------| hook |  |          |
//   |            |         |         | (6)    +-----------------+---------->+------+  |          |
//   |            |         |         |  restore saved         ^   (9) ret      | (10) |          |
//   |            |         |         |  regs and args         |                '------'          |
//   |            |         |         |                        '----------.    return to          |
//   |            |         |         v                                   |    abi_fixer          |
//   |            |         |   +------------+ contains the overwritten   |                       |
//   |            |         |   | trampoline | instructions from the      |                       |
//   |            |         |   +------------+ original function          |                       |
//   |            |         |         | (7)                               |                       |
//   |            |         |         |  jump to the hooked function      |                       |
//   |            |         |         |  behind the hook-instructions     |                       |
//   |            |         '---------'                                   |                       |
//   |            |                                                       |                       |
//   |            | return to call_trampoline                             |                       |
//   |            '-------------------------------------------------------'                       |
//   |                                                                                            |
//   '--------------------------------------------------------------------------------------------'
//                                                                  return to the original caller

#[repr(C)]
pub struct Hook<IA: IsaAbi, T: 'static> {
    /// address of the original function that we hooked
    orig_addr: usize,
    /// address of the trampoline, which we can call to call the original function
    trampoline_addr: usize,
    /// address of the function we jump to from the original function, that
    /// calls the hook
    interceptor_addr: usize,
    /// `extern "C" fn(&Args)` to restore registers and args and call the trampoline
    call_trampoline_addr: usize,
    /// function pointer of the hook function that should be called instead of the original function
    hook_fn: for<'a> fn(&'static Hook<IA, T>, ArgsRef<'a, IA>),
    /// original bytes of the original function that are overwritten when enabling the hook
    orig_bytes: IA::JmpInterceptorBytesArray,
    /// argument-bytes passed to the original function via the stack
    orig_stack_arg_size: u16,
    user_context: T,
}

impl<IA: IsaAbi> Hook<IA, ()> {
    #[must_use]
    pub unsafe fn create(orig_addr: usize, hook_fn: for<'a> fn(&'static Hook<IA, ()>, ArgsRef<'a, IA>)) -> &'static Hook<IA, ()> {
        unsafe { Self::with_context(orig_addr, hook_fn, ()) }
    }
}
impl<IA: IsaAbi, T> Hook<IA, T> {
    #[must_use]
    pub unsafe fn with_context(orig_addr: usize, hook_fn: for<'a> fn(&'static Hook<IA, T>, ArgsRef<'a, IA>), user_context: T) -> &'static Hook<IA, T> {
        let orig_stack_arg_size = unsafe { FunctionDecoder::<IA>::new(orig_addr) }.stack_argument_size();

        let builder = HookMemoryPageBuilder::<IA, T>::new();

        let trampoline = unsafe { trampoline::create_trampoline::<IA>(orig_addr) };
        let builder = builder.trampoline(trampoline);

        let interceptor = unsafe { IA::create_interceptor::<T>(builder.hook_struct_addr(), orig_stack_arg_size) };
        let builder = builder.interceptor(interceptor);

        let call_trampoline = IA::create_call_trampoline(builder.trampoline_addr());
        let builder = builder.call_trampoline(call_trampoline);

        let orig_bytes = unsafe { get_orig_bytes::<IA>(orig_addr) };
        let hook = Hook {
            orig_addr,
            trampoline_addr: builder.trampoline_addr(),
            interceptor_addr: builder.interceptor_addr(),
            call_trampoline_addr: builder.call_trampoline_addr(),
            hook_fn,
            orig_bytes,
            orig_stack_arg_size,
            user_context,
        };
        builder.finalize(hook)
    }

    pub fn enable(&self) {
        let jmp = IA::create_jmp_to_interceptor(self.interceptor_addr);
        unsafe { IA::make_rw(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
        let slice = unsafe { slice::from_raw_parts_mut(self.orig_addr as *mut u8, IA::JmpInterceptorBytesArray::LEN) };
        jmp.store_to(slice);
        unsafe { IA::make_rx(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
    }
    pub fn enabled(&self) -> &Self {
        self.enable();
        self
    }
    pub fn disable(&self) {
        let slice = unsafe { slice::from_raw_parts_mut(self.orig_addr as *mut u8, IA::JmpInterceptorBytesArray::LEN) };
        unsafe { IA::make_rw(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
        slice.copy_from_slice(self.orig_bytes.as_slice());
        unsafe { IA::make_rx(self.orig_addr, IA::JmpInterceptorBytesArray::LEN) };
    }
    pub fn call_original_function(&self, args: impl AsRef<IA::Args>) {
        unsafe {
            let function: extern "C" fn(&IA::Args) = mem::transmute(self.call_trampoline_addr);
            function(args.as_ref())
        }
    }
    pub fn trampoline(&self) -> *const () {
        self.call_trampoline_addr as *const ()
    }
    pub fn user_context(&self) -> &T {
        &self.user_context
    }
}

struct Interceptor {
    pub instructions: Vec<Instruction>,
}
struct CallTrampoline {
    pub instructions: Vec<Instruction>,
}

fn assemble<IA: IsaAbi>(instructions: &[Instruction], ip: u64) -> Result<Vec<u8>, IcedError> {
    let block = InstructionBlock::new(&instructions, ip);
    BlockEncoder::encode(IA::BITNESS, block, BlockEncoderOptions::NONE)
        .map(|res| res.code_buffer)
}

unsafe fn get_orig_bytes<IA: IsaAbi>(orig_addr: usize) -> IA::JmpInterceptorBytesArray {
    let slice = unsafe { slice::from_raw_parts(orig_addr as *const u8, IA::JmpInterceptorBytesArray::LEN) };
    IA::JmpInterceptorBytesArray::load_from(slice)
}

pub struct SafeHook<IA: IsaAbi, F: HookableFunction<IA> + 'static> {
    hook: &'static Hook<IA, (F, bool)>,
    has_this_pointer: bool,
}
impl<IA: IsaAbi, F: HookableFunction<IA>> SafeHook<IA, F> {
    #[must_use]
    pub unsafe fn create_with_this_pointer(orig_addr: usize, hook_fn: F) -> SafeHook<IA, F> {
        unsafe { Self::create(orig_addr, hook_fn, true) }
    }
    #[must_use]
    pub unsafe fn create_without_this_pointer(orig_addr: usize, hook_fn: F) -> SafeHook<IA, F> {
        unsafe { Self::create(orig_addr, hook_fn, false) }
    }
    #[must_use]
    unsafe fn create(orig_addr: usize, hook_fn: F, has_this_pointer: bool) -> SafeHook<IA, F> {
        Self {
            hook: unsafe { Hook::with_context(orig_addr, hook_fn_for_hookable_function::<IA, F>, (hook_fn, has_this_pointer)) },
            has_this_pointer,
        }
    }

    pub fn enable(&self) {
        self.hook.enable()
    }
    pub fn enabled(&self) -> &Self {
        self.hook.enable();
        self
    }
    pub fn disable(&self) {
        self.hook.disable()
    }
    pub fn call_original_function<R: HookableFunctionRet>(&self, args: F::Args) -> R {
        let mut a = IA::Args::new();
        let mut a_ref = ArgsRef::<IA>::new(&mut a);
        if self.has_this_pointer {
            a_ref.store_with_this_pointer(args);
        } else {
            a_ref.store_without_this_pointer(args);
        }
        self.hook.call_original_function(a_ref);
        R::from_usize(a.return_value())
    }
}

fn hook_fn_for_hookable_function<IA: IsaAbi, F: HookableFunction<IA>>(hook: &'static Hook<IA, (F, bool)>, mut args: ArgsRef<IA>) {
    let (hook_fn, has_this_pointer) = hook.user_context();
    let args = if *has_this_pointer {
        args.with_this_pointer::<F::Args>()
    } else {
        args.without_this_pointer::<F::Args>()
    };
    let safe_hook = SafeHook {
        hook,
        has_this_pointer: *has_this_pointer,
    };
    hook_fn.call(&safe_hook, F::Args::convert_output_to_owned(args));
}

pub trait HookableFunction<IA: IsaAbi> {
    type Args: LoadFromArgs + StoreToArgs;
    fn call(&self, hook: &SafeHook<IA, Self>, args: Self::Args);
}
macro_rules! impl_hookable_function {
    (fn($($args:ident),*)) => {
        #[allow(unused_parens)]
        impl<IA: IsaAbi, $($args),*> HookableFunction for fn(&SafeHook<IA, Self>, $($args),*)
        where
            ($($args),*): LoadFromArgs,
            ($($args),*): StoreToArgs,
        {
            type Args = ($($args),*);
            fn call(&self, hook: &SafeHook<IA, Self>, args: Self::Args) {
                #[allow(non_snake_case)]
                let ($($args),*) = args;
                (self)($($args,)*);
            }
        }
    };
}
impl_hookable_function!(fn());
impl_hookable_function!(fn(A));
impl_hookable_function!(fn(A, B));
impl_hookable_function!(fn(A, B, C));
impl_hookable_function!(fn(A, B, C, D));
impl_hookable_function!(fn(A, B, C, D, E));
impl_hookable_function!(fn(A, B, C, D, E, F));
impl_hookable_function!(fn(A, B, C, D, E, F, G));
impl_hookable_function!(fn(A, B, C, D, E, F, G, H));
impl_hookable_function!(fn(A, B, C, D, E, F, G, H, I));
impl_hookable_function!(fn(A, B, C, D, E, F, G, H, I, J));
impl_hookable_function!(fn(A, B, C, D, E, F, G, H, I, J, K));

pub trait HookableFunctionRet {
    fn from_usize(res: usize) -> Self;
}
macro_rules! impl_hookable_function_ret {
    ($res:ident: $typ:ty => $conv:expr) => {
        impl HookableFunctionRet for $typ {
            fn from_usize($res: usize) -> Self {
                $conv
            }
        }
    };
}
impl_hookable_function_ret!(_res: () => ());
impl_hookable_function_ret!(res: usize => res);
impl_hookable_function_ret!(res: u8 => res as u8);
impl_hookable_function_ret!(res: i8 => res as i8);
impl_hookable_function_ret!(res: u16 => res as u16);
impl_hookable_function_ret!(res: i16 => res as i16);
impl_hookable_function_ret!(res: u32 => res as u32);
impl_hookable_function_ret!(res: i32 => res as i32);
