use crate::{ArgsRef, RawHook, IsaAbi};
use crate::args::{Args, LoadFromArgs, StoreToArgs};

pub struct TypedHook<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static> {
    hook: &'static RawHook<IA, SafeHookContext<IA, F, T>>,
    has_this_pointer: bool,
}
struct SafeHookContext<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static> {
    safe_hook_function: F::BoxedFn,
    has_this_pointer: bool,
    user_context: T,
}

impl<IA: IsaAbi> TypedHook<IA, fn(), ()> {
    #[must_use]
    pub unsafe fn with_this_pointer<Args, HF: HookableFunction<IA, (), Args>>(orig_addr: usize, hook_fn: HF) -> TypedHook<IA, HF::RawFnWithoutHook, ()> {
        unsafe { Self::create(orig_addr, hook_fn, true, ()) }
    }
    #[must_use]
    pub unsafe fn without_this_pointer<Args, HF: HookableFunction<IA, (), Args>>(orig_addr: usize, hook_fn: HF) -> TypedHook<IA, HF::RawFnWithoutHook, ()> {
        unsafe { Self::create(orig_addr, hook_fn, false, ()) }
    }
    #[must_use]
    pub unsafe fn with_this_pointer_and_context<Args, HF: HookableFunction<IA, T, Args>, T: 'static>(orig_addr: usize, hook_fn: HF, context: T) -> TypedHook<IA, HF::RawFnWithoutHook, T> {
        unsafe { Self::create(orig_addr, hook_fn, true, context) }
    }
    #[must_use]
    pub unsafe fn without_this_pointer_and_context<Args, HF: HookableFunction<IA, T, Args>, T: 'static>(orig_addr: usize, hook_fn: HF, context: T) -> TypedHook<IA, HF::RawFnWithoutHook, T> {
        unsafe { Self::create(orig_addr, hook_fn, false, context) }
    }
    #[must_use]
    unsafe fn create<Args, HF: HookableFunction<IA, T, Args>, T: 'static>(orig_addr: usize, hook_fn: HF, has_this_pointer: bool, user_context: T) -> TypedHook<IA, HF::RawFnWithoutHook, T> {
        let context = SafeHookContext {
            safe_hook_function: hook_fn.into_boxed_fn(),
            has_this_pointer,
            user_context,
        };
        TypedHook {
            hook: unsafe { RawHook::with_context(orig_addr, hook_fn_for_hookable_function::<IA, HF::RawFnWithoutHook, T>, context) },
            has_this_pointer,
        }
    }
}

impl<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static> TypedHook<IA, F, T> {
    pub fn enable(&self) {
        self.hook.enable()
    }
    pub fn enabled(self) -> Self {
        self.hook.enable();
        self
    }
    pub fn disable(&self) {
        self.hook.disable()
    }
    pub fn context(&self) -> &T {
        &self.hook.context().user_context
    }
    pub unsafe fn call_original_function(&self, args: F::Args) -> usize {
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

fn hook_fn_for_hookable_function<IA: IsaAbi, F: RawFnWithoutHook<IA, T>, T: 'static>(hook: &'static RawHook<IA, SafeHookContext<IA, F, T>>, mut args: ArgsRef<IA>) {
    let SafeHookContext { safe_hook_function, has_this_pointer, user_context: _ } = hook.context();
    let args = if *has_this_pointer {
        args.with_this_pointer::<F::Args>()
    } else {
        args.without_this_pointer::<F::Args>()
    };
    let safe_hook = TypedHook {
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
    fn call(function: &Self::BoxedFn, hook: &TypedHook<IA, Self, T>, args: Self::Args);
}
macro_rules! impl_hookable_function {
    (fn($($args:ident),*)) => {
        #[allow(unused_parens)]
        impl<IA: IsaAbi, Ctx: 'static, Function, $($args),*> HookableFunction<IA, Ctx, ($($args,)*)> for Function
        where
            Function: Fn(&TypedHook<IA, fn($($args,)*), Ctx>, $($args),*) + 'static + Sync,
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
            type BoxedFn = Box<dyn Fn(&TypedHook<IA, fn($($args),*), Ctx>, $($args),*) + Sync>;
            type Args = ($($args),*);
            fn call(function: &Self::BoxedFn, hook: &TypedHook<IA, Self, Ctx>, args: Self::Args) {
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
