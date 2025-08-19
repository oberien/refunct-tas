use crate::args::Args;

pub struct StoreArgs<T: Args> {
    args: T,
    ctx: ArgsStoreContext,
}
impl<T: Args> StoreArgs<T> {
    pub fn new(args: T) -> Self {
        Self {
            args,
            ctx: ArgsStoreContext {
                int_args_stored: 0,
                float_args_stored: 0,
            }
        }
    }
    fn set_next_int_arg(&mut self, val: usize) {
        self.args.set_next_int_arg(val, &self.ctx);
        self.ctx.int_args_stored += 1;
    }
    fn set_next_float_arg(&mut self, val: f32) {
        self.args.set_next_float_arg(val, &self.ctx);
        self.ctx.float_args_stored += 1;
    }
}

pub struct ArgsStoreContext {
    int_args_stored: usize,
    float_args_stored: usize,
}

impl ArgsStoreContext {
    pub fn int_args_stored(&self) -> usize {
        self.int_args_stored
    }
    pub fn float_args_stored(&self) -> usize {
        self.float_args_stored
    }
}

pub trait StoreToArgs {
    fn store_to_args(self, store_args: &mut StoreArgs<impl Args>);
}

macro_rules! impl_store_to_args_for_number {
    ($typ:ty => $function:ident) => {
        impl StoreToArgs for $typ {
            fn store_to_args(self, store_args: &mut StoreArgs<impl Args>) {
                store_args.$function(self as _);
            }
        }
    };
}
impl_store_to_args_for_number!(bool => set_next_int_arg);
impl_store_to_args_for_number!(u8 => set_next_int_arg);
impl_store_to_args_for_number!(i8 => set_next_int_arg);
impl_store_to_args_for_number!(u16 => set_next_int_arg);
impl_store_to_args_for_number!(i16 => set_next_int_arg);
impl_store_to_args_for_number!(u32 => set_next_int_arg);
impl_store_to_args_for_number!(i32 => set_next_int_arg);
impl_store_to_args_for_number!(usize => set_next_int_arg);
impl_store_to_args_for_number!(f32 => set_next_float_arg);

impl<T> StoreToArgs for *mut T {
    fn store_to_args(self, store_args: &mut StoreArgs<impl Args>) {
        store_args.set_next_int_arg(self.addr())
    }
}
impl StoreToArgs for () {
    fn store_to_args(self, _store_args: &mut StoreArgs<impl Args>) {
        // noop
    }
}
macro_rules! impl_store_to_args_for_tuple {
    ($($generic:ident),*) => {
        impl<$($generic: StoreToArgs),*> StoreToArgs for ($($generic),*) {
            fn store_to_args(self, store_args: &mut StoreArgs<impl Args>) {
                #[allow(non_snake_case)]
                let ($($generic,)*) = self;
                $(
                    $generic::store_to_args($generic, store_args);
                )*
            }
        }
    }
}
impl_store_to_args_for_tuple!(A, B);
impl_store_to_args_for_tuple!(A, B, C);
impl_store_to_args_for_tuple!(A, B, C, D);
impl_store_to_args_for_tuple!(A, B, C, D, E);
impl_store_to_args_for_tuple!(A, B, C, D, E, F);
impl_store_to_args_for_tuple!(A, B, C, D, E, F, G);
impl_store_to_args_for_tuple!(A, B, C, D, E, F, G, H);
impl_store_to_args_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_store_to_args_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_store_to_args_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);

