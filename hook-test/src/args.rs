use crate::isa_abi::IsaAbi;

#[repr(transparent)]
pub struct ArgsRef<'a, IA: IsaAbi> {
    args: &'a mut IA::Args,
}
#[repr(transparent)]
pub struct ArgsBoxed<IA: IsaAbi> {
    args: Box<IA::Args>,
}

impl<IA: IsaAbi> ArgsRef<'_, IA> {
    pub fn boxed(&self) -> ArgsBoxed<IA> where IA::Args: Clone {
        ArgsBoxed { args: Box::new(self.args.clone()) }
    }
    pub fn with_this_pointer<T: FromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut self.args, true)
    }
    pub fn without_this_pointer<T: FromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut self.args, false)
    }
    pub fn as_args(&self) -> &IA::Args {
        self.args
    }
}
impl<IA: IsaAbi> ArgsBoxed<IA> {
    pub fn with_this_pointer<T: FromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut *self.args, true)
    }
    pub fn without_this_pointer<T: FromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut *self.args, false)
    }
    pub fn as_args(&self) -> &IA::Args {
        &self.args
    }
}

fn load_args<T: FromArgs>(args: &mut impl Args, has_this_pointer: bool) -> T::Output<'_> {
    // SAFETY: the lifetime is bound to &mut Args
    unsafe {
        T::convert_pointer_to_arg(T::get_pointer_to_arg(&mut AccessArgs {
            args,
            ctx: ArgsContext {
                has_this_pointer,
                int_args_consumed: 0,
                float_args_consumed: 0,
            }
        }))
    }
}

pub unsafe trait Args {
    /// if `ctx.has_this_pointer`, the first returned integer argument must be the this-pointer
    ///
    /// SAFETY: as long as different `ctx` are passed, each pointer returned by `next_int_arg` and
    ///        `next_float_arg` must point to a different memory location
    fn next_int_arg(&mut self, ctx: &ArgsContext) -> *mut usize;
    /// SAFETY: as long as different `ctx` are passed, each pointer returned by `next_int_arg` and
    ///        `next_float_arg` must point to a different memory location
    fn next_float_arg(&mut self, ctx: &ArgsContext) -> *mut f32;

    fn set_return_value(&mut self, ret_val: usize);
}
unsafe impl<T: Args + ?Sized> Args for &'_ mut T {
    fn next_int_arg(&mut self, ctx: &ArgsContext) -> *mut usize {
        T::next_int_arg(self, ctx)
    }
    fn next_float_arg(&mut self, ctx: &ArgsContext) -> *mut f32 {
        T::next_float_arg(self, ctx)
    }
    fn set_return_value(&mut self, ret_val: usize) {
        T::set_return_value(self, ret_val)
    }
}

pub struct AccessArgs<T: Args> {
    args: T,
    ctx: ArgsContext,
}
impl<T: Args> AccessArgs<T> {
    fn next_int_arg(&mut self) -> *mut usize {
        let res = self.args.next_int_arg(&self.ctx);
        self.ctx.int_args_consumed += 1;
        res
    }
    fn next_float_arg(&mut self) -> *mut f32 {
        let res = self.args.next_float_arg(&self.ctx);
        self.ctx.float_args_consumed += 1;
        res
    }
}

pub trait FromArgs {
    type Pointer;
    type Output<'a> where Self: 'a;
    fn get_pointer_to_arg(access: &mut AccessArgs<impl Args>) -> Self::Pointer;
    /// # Safety
    /// * lifetime must be bound to the `Args` lifetime
    unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a>;
}

macro_rules! impl_from_args_for_int {
    ($typ:ty => $function:ident) => {
        impl FromArgs for $typ {
            type Pointer = *mut Self;
            type Output<'a> = &'a mut Self;
            fn get_pointer_to_arg(access: &mut AccessArgs<impl Args>) -> Self::Pointer {
                access.$function() as *mut Self
            }
            unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a> {
                unsafe { &mut *ptr }
            }
        }
    };
}
impl_from_args_for_int!(u8 => next_int_arg);
impl_from_args_for_int!(i8 => next_int_arg);
impl_from_args_for_int!(u16 => next_int_arg);
impl_from_args_for_int!(i16 => next_int_arg);
impl_from_args_for_int!(u32 => next_int_arg);
impl_from_args_for_int!(i32 => next_int_arg);
impl_from_args_for_int!(usize => next_int_arg);
impl_from_args_for_int!(f32 => next_float_arg);

impl<T> FromArgs for *mut T {
    type Pointer = *mut *mut T;
    type Output<'a> = *mut T where T: 'a;
    fn get_pointer_to_arg(access: &mut AccessArgs<impl Args>) -> Self::Pointer {
        access.next_int_arg() as *mut *mut T
    }
    unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a> where T: 'a {
        unsafe { *ptr }
    }
}
macro_rules! impl_load_arg_for_tuple {
    ($($generic:ident),*) => {
        impl<$($generic: FromArgs),*> FromArgs for ($($generic),*) {
            type Pointer = ($($generic::Pointer),*);
            type Output<'a> = ($($generic::Output<'a>),*) where $($generic: 'a),*;
            fn get_pointer_to_arg(access: &mut AccessArgs<impl Args>) -> Self::Pointer {
                (
                    $($generic::get_pointer_to_arg(access)),*
                )
            }
            unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a> {
                unsafe {
                    #[allow(non_snake_case)]
                    let ($($generic),*) = ptr;
                    (
                        $($generic::convert_pointer_to_arg($generic)),*
                    )
                }
            }
        }
    }
}
impl_load_arg_for_tuple!(A, B);
impl_load_arg_for_tuple!(A, B, C);
impl_load_arg_for_tuple!(A, B, C, D);
impl_load_arg_for_tuple!(A, B, C, D, E);
impl_load_arg_for_tuple!(A, B, C, D, E, F);
impl_load_arg_for_tuple!(A, B, C, D, E, F, G);
impl_load_arg_for_tuple!(A, B, C, D, E, F, G, H);
impl_load_arg_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_load_arg_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_load_arg_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);

pub struct ArgsContext {
    has_this_pointer: bool,
    int_args_consumed: usize,
    float_args_consumed: usize,
}

impl ArgsContext {
    pub fn has_this_pointer(&self) -> bool {
        self.has_this_pointer
    }
    pub fn int_args_consumed(&self) -> usize {
        self.int_args_consumed
    }
    pub fn float_args_consumed(&self) -> usize {
        self.float_args_consumed
    }
}
