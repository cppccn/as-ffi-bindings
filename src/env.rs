use wasmer::{Memory, TypedFunction};

#[derive(Clone, Default)]
pub struct Env {
    pub memory: Option<Memory>,
    pub fn_new: Option<TypedFunction<(i32, i32), i32>>,
    pub fn_pin: Option<TypedFunction<i32, i32>>,
    pub fn_unpin: Option<TypedFunction<i32, ()>>,
    pub fn_collect: Option<TypedFunction<(), ()>>,
}

impl Env {
    pub fn init_with(
        &mut self,
        memory: Option<Memory>,
        asc_fn_new: Option<TypedFunction<(i32, i32), i32>>,
        asc_fn_pin: Option<TypedFunction<i32, i32>>,
        asc_fn_unpin: Option<TypedFunction<i32, ()>>,
        asc_fn_collect: Option<TypedFunction<(), ()>>,
    ) {
        self.memory = memory;
        self.fn_new = asc_fn_new;
        self.fn_pin = asc_fn_pin;
        self.fn_unpin = asc_fn_unpin;
        self.fn_collect = asc_fn_collect;
    }
}
