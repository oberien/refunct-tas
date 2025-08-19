use crate::args::Args;

pub struct LoadArgs<T: Args> {
    args: T,
    ctx: ArgsLoadContext,
}
impl<T: Args> LoadArgs<T> {
    pub fn new(args: T) -> Self {
        Self {
            args,
            ctx: ArgsLoadContext {
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

pub struct ArgsLoadContext {
    int_args_consumed: usize,
    float_args_consumed: usize,
}

impl ArgsLoadContext {
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
    fn get_pointer_to_arg(load_args: &mut LoadArgs<impl Args>) -> Self::Pointer;
    /// # Safety
    /// * lifetime must be bound to the `Args` lifetime
    unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a>;
    fn convert_output_to_owned<'a>(output: Self::Output<'a>) -> Self where Self: 'a;
}

impl LoadFromArgs for bool {
    type Pointer = *mut usize;
    type Output<'a> = &'a mut bool;
    fn get_pointer_to_arg(load_args: &mut LoadArgs<impl Args>) -> Self::Pointer {
        load_args.next_int_arg()
    }
    unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a> {
        unsafe {
            let val = *ptr & 0xff != 0;
            *ptr = val as usize;
            &mut *(ptr as *mut bool)
        }
    }
    fn convert_output_to_owned<'a>(output: Self::Output<'a>) -> Self where Self: 'a {
        *output
    }
}

macro_rules! impl_load_from_args_for_number {
    ($typ:ty => $function:ident) => {
        impl LoadFromArgs for $typ {
            type Pointer = *mut Self;
            type Output<'a> = &'a mut Self;
            fn get_pointer_to_arg(load_args: &mut LoadArgs<impl Args>) -> Self::Pointer {
                load_args.$function() as *mut Self
            }
            unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a> {
                unsafe { &mut *ptr }
            }
            fn convert_output_to_owned<'a>(output: Self::Output<'a>) -> Self where Self: 'a {
                *output
            }
        }
    };
}
impl_load_from_args_for_number!(u8 => next_int_arg);
impl_load_from_args_for_number!(i8 => next_int_arg);
impl_load_from_args_for_number!(u16 => next_int_arg);
impl_load_from_args_for_number!(i16 => next_int_arg);
impl_load_from_args_for_number!(u32 => next_int_arg);
impl_load_from_args_for_number!(i32 => next_int_arg);
impl_load_from_args_for_number!(usize => next_int_arg);
impl_load_from_args_for_number!(f32 => next_float_arg);

impl<T> LoadFromArgs for *mut T {
    type Pointer = *mut *mut T;
    type Output<'a> = *mut T where T: 'a;
    fn get_pointer_to_arg(load_args: &mut LoadArgs<impl Args>) -> Self::Pointer {
        load_args.next_int_arg() as *mut *mut T
    }
    unsafe fn convert_pointer_to_arg<'a>(ptr: Self::Pointer) -> Self::Output<'a> where T: 'a {
        unsafe { *ptr }
    }
    fn convert_output_to_owned<'a>(output: Self::Output<'a>) -> Self where Self: 'a {
        output
    }
}
impl LoadFromArgs for () {
    type Pointer = ();
    type Output<'a> = ();
    fn get_pointer_to_arg(_load_args: &mut LoadArgs<impl Args>) -> Self::Pointer {
        // noop
    }
    unsafe fn convert_pointer_to_arg<'a>(_ptr: Self::Pointer) -> Self::Output<'a> {
        // noop
    }
    fn convert_output_to_owned<'a>(output: Self::Output<'a>) -> Self where Self: 'a {
        output
    }
}
macro_rules! impl_load_from_args_for_tuple {
    ($($generic:ident),*) => {
        impl<$($generic: LoadFromArgs),*> LoadFromArgs for ($($generic),*) {
            type Pointer = ($($generic::Pointer),*);
            type Output<'a> = ($($generic::Output<'a>),*) where $($generic: 'a),*;
            fn get_pointer_to_arg(load_args: &mut LoadArgs<impl Args>) -> Self::Pointer {
                (
                    $($generic::get_pointer_to_arg(load_args)),*
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
            fn convert_output_to_owned<'a>(output: Self::Output<'a>) -> Self where Self: 'a {
                #[allow(non_snake_case)]
                let ($($generic),*) = output;
                (
                    $($generic::convert_output_to_owned($generic)),*
                )
            }
        }
    }
}
impl_load_from_args_for_tuple!(A, B);
impl_load_from_args_for_tuple!(A, B, C);
impl_load_from_args_for_tuple!(A, B, C, D);
impl_load_from_args_for_tuple!(A, B, C, D, E);
impl_load_from_args_for_tuple!(A, B, C, D, E, F);
impl_load_from_args_for_tuple!(A, B, C, D, E, F, G);
impl_load_from_args_for_tuple!(A, B, C, D, E, F, G, H);
impl_load_from_args_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_load_from_args_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_load_from_args_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
