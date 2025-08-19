use crate::args::load::LoadArgs;
use crate::isa_abi::IsaAbi;

mod load;
mod store;

pub use load::{ArgsLoadContext, LoadFromArgs};
pub use store::{ArgsStoreContext, StoreToArgs};
use crate::args::store::StoreArgs;

#[repr(transparent)]
pub struct ArgsRef<'a, IA: IsaAbi> {
    args: &'a mut IA::Args,
}
#[repr(transparent)]
pub struct ArgsBoxed<IA: IsaAbi> {
    args: Box<IA::Args>,
}

impl<IA: IsaAbi> ArgsRef<'_, IA> {
    pub fn new(args: &mut IA::Args) -> ArgsRef<'_, IA> {
        ArgsRef { args }
    }
    pub fn boxed(&self) -> ArgsBoxed<IA> where IA::Args: Clone {
        ArgsBoxed::new(self.args.clone())
    }
    pub fn load<T: LoadFromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut self.args)
    }
    pub fn store<T: StoreToArgs>(&mut self, val: T) {
        store_args(&mut self.args, val)
    }
}
impl<IA: IsaAbi> AsRef<IA::Args> for ArgsRef<'_, IA> {
    fn as_ref(&self) -> &IA::Args {
        &self.args
    }
}
impl<IA: IsaAbi> ArgsBoxed<IA> {
    pub fn new(args: IA::Args) -> Self {
        Self {
            args: Box::new(args),
        }
    }
    pub fn load<T: LoadFromArgs>(&mut self) -> T::Output<'_> {
        load_args::<T>(&mut *self.args)
    }
    pub fn store<T: StoreToArgs>(&mut self, val: T) {
        store_args(&mut *self.args, val)
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

fn load_args<T: LoadFromArgs>(args: &mut impl Args) -> T::Output<'_> {
    // SAFETY: the lifetime is bound to &mut Args
    unsafe {
        T::convert_pointer_to_arg(T::get_pointer_to_arg(&mut LoadArgs::new(args)))
    }
}
fn store_args<T: StoreToArgs>(args: &mut impl Args, val: T) {
    val.store_to_args(&mut StoreArgs::new(args));
}

pub unsafe trait Args {
    fn new() -> Self;
    /// SAFETY: as long as different `ctx` are passed, each pointer returned by `next_int_arg` and
    ///        `next_float_arg` must point to a different memory location
    fn next_int_arg(&mut self, ctx: &ArgsLoadContext) -> *mut usize;
    /// SAFETY: as long as different `ctx` are passed, each pointer returned by `next_int_arg` and
    ///        `next_float_arg` must point to a different memory location
    fn next_float_arg(&mut self, ctx: &ArgsLoadContext) -> *mut f32;

    fn set_next_int_arg(&mut self, val: usize, ctx: &ArgsStoreContext);
    fn set_next_float_arg(&mut self, val: f32, ctx: &ArgsStoreContext);

    fn return_value(&self) -> usize;
    fn set_return_value(&mut self, ret_val: usize);
}
unsafe impl<T: Args> Args for &'_ mut T {
    fn new() -> Self {
        panic!("can't call `<&mut impl Args>::new`")
    }
    fn next_int_arg(&mut self, ctx: &ArgsLoadContext) -> *mut usize {
        T::next_int_arg(self, ctx)
    }
    fn next_float_arg(&mut self, ctx: &ArgsLoadContext) -> *mut f32 {
        T::next_float_arg(self, ctx)
    }
    fn set_next_int_arg(&mut self, val: usize, ctx: &ArgsStoreContext) {
        T::set_next_int_arg(self, val, ctx)
    }
    fn set_next_float_arg(&mut self, val: f32, ctx: &ArgsStoreContext) {
        T::set_next_float_arg(self, val, ctx)
    }
    fn return_value(&self) -> usize {
        T::return_value(self)
    }
    fn set_return_value(&mut self, ret_val: usize) {
        T::set_return_value(self, ret_val)
    }
}
