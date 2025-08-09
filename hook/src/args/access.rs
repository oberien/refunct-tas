use crate::args::Args;

pub struct AccessArgs<T: Args> {
    args: T,
    ctx: ArgsAccessContext,
}
impl<T: Args> AccessArgs<T> {
    pub fn new(args: T, has_this_pointer: bool) -> Self {
        Self {
            args,
            ctx: ArgsAccessContext {
                has_this_pointer,
                int_args_consumed: 0,
                float_args_consumed: 0,
            }
        }
    }
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

pub struct ArgsAccessContext {
    has_this_pointer: bool,
    int_args_consumed: usize,
    float_args_consumed: usize,
}

impl ArgsAccessContext {
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

pub trait LoadFromArgs {
    type Pointer;
    type Output<'a> where Self: 'a;
    fn get_pointer_to_arg(access: &mut AccessArgs<impl Args>) -> Self::Pointer;
    /// # Safety
    /// * lifetime must be bound to the `Args` lifetime
    unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a>;
}

macro_rules! impl_from_args_for_int {
    ($typ:ty => $function:ident) => {
        impl LoadFromArgs for $typ {
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

impl<T> LoadFromArgs for *mut T {
    type Pointer = *mut *mut T;
    type Output<'a> = *mut T where T: 'a;
    fn get_pointer_to_arg(access: &mut AccessArgs<impl Args>) -> Self::Pointer {
        access.next_int_arg() as *mut *mut T
    }
    unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a> where T: 'a {
        unsafe { *ptr }
    }
}
impl LoadFromArgs for () {
    type Pointer = ();
    type Output<'a> = ();
    fn get_pointer_to_arg(_access: &mut AccessArgs<impl Args>) -> Self::Pointer {
        ()
    }
    unsafe fn convert_pointer_to_arg<'a>(_ptr: Self::Pointer) -> Self::Output<'a> {
        ()
    }
}
macro_rules! impl_from_args_for_tuple {
    ($($generic:ident),*) => {
        impl<$($generic: LoadFromArgs),*> LoadFromArgs for ($($generic),*) {
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
impl_from_args_for_tuple!(A, B);
impl_from_args_for_tuple!(A, B, C);
impl_from_args_for_tuple!(A, B, C, D);
impl_from_args_for_tuple!(A, B, C, D, E);
impl_from_args_for_tuple!(A, B, C, D, E, F);
impl_from_args_for_tuple!(A, B, C, D, E, F, G);
impl_from_args_for_tuple!(A, B, C, D, E, F, G, H);
impl_from_args_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_from_args_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_from_args_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);

