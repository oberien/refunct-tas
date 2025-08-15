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
    pub fn context(&self) -> &T {
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

struct SafeHookContext<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static> {
    safe_hook_function: F::BoxedFn,
    has_this_pointer: bool,
    user_context: T,
}

pub struct SafeHook<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static> {
    hook: &'static Hook<IA, SafeHookContext<IA, F, T>>,
    has_this_pointer: bool,
}
impl<IA: IsaAbi> SafeHook<IA, fn(), ()> {
    #[must_use]
    pub unsafe fn with_this_pointer<Args, HF: HookableFunction<IA, (), Args>>(orig_addr: usize, hook_fn: HF) -> SafeHook<IA, HF::RawFnWithoutHook, ()> {
        unsafe { Self::create(orig_addr, hook_fn, true, ()) }
    }
    #[must_use]
    pub unsafe fn without_this_pointer<Args, HF: HookableFunction<IA, (), Args>>(orig_addr: usize, hook_fn: HF) -> SafeHook<IA, HF::RawFnWithoutHook, ()> {
        unsafe { Self::create(orig_addr, hook_fn, false, ()) }
    }
    #[must_use]
    pub unsafe fn with_this_pointer_and_context<Args, HF: HookableFunction<IA, T, Args>, T: 'static>(orig_addr: usize, hook_fn: HF, context: T) -> SafeHook<IA, HF::RawFnWithoutHook, T> {
        unsafe { Self::create(orig_addr, hook_fn, true, context) }
    }
    #[must_use]
    pub unsafe fn without_this_pointer_and_context<Args, HF: HookableFunction<IA, T, Args>, T: 'static>(orig_addr: usize, hook_fn: HF, context: T) -> SafeHook<IA, HF::RawFnWithoutHook, T> {
        unsafe { Self::create(orig_addr, hook_fn, false, context) }
    }
    #[must_use]
    unsafe fn create<Args, HF: HookableFunction<IA, T, Args>, T: 'static>(orig_addr: usize, hook_fn: HF, has_this_pointer: bool, user_context: T) -> SafeHook<IA, HF::RawFnWithoutHook, T> {
        let context = SafeHookContext {
            safe_hook_function: hook_fn.into_boxed_fn(),
            has_this_pointer,
            user_context,
        };
        SafeHook {
            hook: unsafe { Hook::with_context(orig_addr, hook_fn_for_hookable_function::<IA, HF::RawFnWithoutHook, T>, context) },
            has_this_pointer,
        }
    }
}

impl<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static> SafeHook<IA, F, T> {
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
    pub fn context(&self) -> &T {
        &self.hook.user_context.user_context
    }
    pub fn call_original_function(&self, args: F::Args) -> usize {
        let mut a = IA::Args::new();
        let mut a_ref = ArgsRef::<IA>::new(&mut a);
        if self.has_this_pointer {
            a_ref.store_with_this_pointer(args);
        } else {
            a_ref.store_without_this_pointer(args);
        }
        self.hook.call_original_function(a_ref);
        a.return_value()
    }
}

fn hook_fn_for_hookable_function<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static>(hook: &'static Hook<IA, SafeHookContext<IA, F, T>>, mut args: ArgsRef<IA>) {
    let SafeHookContext { safe_hook_function, has_this_pointer, user_context: _ } = hook.context();
    let args = if *has_this_pointer {
        args.with_this_pointer::<F::Args>()
    } else {
        args.without_this_pointer::<F::Args>()
    };
    let safe_hook = SafeHook {
        hook,
        has_this_pointer: *has_this_pointer,
    };
    F::call(safe_hook_function, &safe_hook, F::Args::convert_output_to_owned(args));
}

pub trait HookableFunction<IA: IsaAbi, T, Args>: 'static + Sync {
    type RawFnWithoutHook: RawFnWithoutHook<IA, T>;
    fn into_boxed_fn(self) -> <Self::RawFnWithoutHook as RawFnWithoutHook<IA, T>>::BoxedFn;
}
pub trait RawFnWithoutHook<IA: IsaAbi, T>: Sized + 'static + Sync {
    type BoxedFn;
    type Args: LoadFromArgs + StoreToArgs;
    fn call(function: &Self::BoxedFn, hook: &SafeHook<IA, Self, T>, args: Self::Args);
}
macro_rules! impl_hookable_function {
    (fn($($args:ident),*)) => {
        #[allow(unused_parens)]
        impl<IA: IsaAbi, Ctx: 'static, Function, $($args),*> HookableFunction<IA, Ctx, ($($args,)*)> for Function
        where
            Function: Fn(&SafeHook<IA, fn($($args,)*), Ctx>, $($args),*) + 'static + Sync,
            ($($args),*): LoadFromArgs,
            ($($args),*): StoreToArgs,
            $($args: 'static,)*
        {
            type RawFnWithoutHook = fn($($args),*);
            fn into_boxed_fn(self) -> <Self::RawFnWithoutHook as RawFnWithoutHook<IA, Ctx>>::BoxedFn {
                Box::new(self)
            }
        }

        #[allow(unused_parens)]
        impl<IA: IsaAbi, Ctx: 'static, $($args),*> RawFnWithoutHook<IA, Ctx> for fn($($args),*)
        where
            ($($args),*): LoadFromArgs,
            ($($args),*): StoreToArgs,
            $($args: 'static,)*
        {
            type BoxedFn = Box<dyn Fn(&SafeHook<IA, fn($($args),*), Ctx>, $($args),*)>;
            type Args = ($($args),*);
            fn call(function: &Self::BoxedFn, hook: &SafeHook<IA, Self, Ctx>, args: Self::Args) {
                #[allow(non_snake_case)]
                let ($($args),*) = args;
                (function)(hook, $($args,)*);
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
