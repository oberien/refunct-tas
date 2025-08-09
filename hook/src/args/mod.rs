use crate::args::access::AccessArgs;
use crate::isa_abi::IsaAbi;

mod access;
mod write;

pub use access::{ArgsAccessContext, LoadFromArgs};

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
    pub fn with_this_pointer<T: LoadFromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut self.args, true)
    }
    pub fn without_this_pointer<T: LoadFromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut self.args, false)
    }
}
impl<IA: IsaAbi> AsRef<IA::Args> for ArgsRef<'_, IA> {
    fn as_ref(&self) -> &IA::Args {
        &self.args
    }
}
impl<IA: IsaAbi> ArgsBoxed<IA> {
    pub fn with_this_pointer<T: LoadFromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut *self.args, true)
    }
    pub fn without_this_pointer<T: LoadFromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut *self.args, false)
    }
    pub fn as_args(&self) -> &IA::Args {
        &self.args
    }
}
impl<IA: IsaAbi> AsRef<IA::Args> for ArgsBoxed<IA> {
    fn as_ref(&self) -> &IA::Args {
        &self.args
    }
}

fn load_args<T: LoadFromArgs>(args: &mut impl Args, has_this_pointer: bool) -> T::Output<'_> {
    // SAFETY: the lifetime is bound to &mut Args
    unsafe {
        T::convert_pointer_to_arg(T::get_pointer_to_arg(&mut AccessArgs::new(args, has_this_pointer)))
    }
}

pub unsafe trait Args {
    /// if `ctx.has_this_pointer`, the first returned integer argument must be the this-pointer
    ///
    /// SAFETY: as long as different `ctx` are passed, each pointer returned by `next_int_arg` and
    ///        `next_float_arg` must point to a different memory location
    fn next_int_arg(&mut self, ctx: &ArgsAccessContext) -> *mut usize;
    /// SAFETY: as long as different `ctx` are passed, each pointer returned by `next_int_arg` and
    ///        `next_float_arg` must point to a different memory location
    fn next_float_arg(&mut self, ctx: &ArgsAccessContext) -> *mut f32;

    fn set_return_value(&mut self, ret_val: usize);
}
unsafe impl<T: Args + ?Sized> Args for &'_ mut T {
    fn next_int_arg(&mut self, ctx: &ArgsAccessContext) -> *mut usize {
        T::next_int_arg(self, ctx)
    }
    fn next_float_arg(&mut self, ctx: &ArgsAccessContext) -> *mut f32 {
        T::next_float_arg(self, ctx)
    }
    fn set_return_value(&mut self, ret_val: usize) {
        T::set_return_value(self, ret_val)
    }
}
