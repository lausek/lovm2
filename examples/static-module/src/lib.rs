use lovm2_core::extend::prelude::*;

#[lovm2_function]
fn minus(op1: i64, op2: i64) -> i64 {
    op1 - op2
}

pub struct CustomVm(LV2Vm);

impl CustomVm {
    pub fn new() -> Self {
        let mut inner = LV2Vm::new();
        inner
            .add_function("minus", create_callable(minus))
            .unwrap();
        Self(inner)
    }

    pub fn inner(&mut self) -> &mut LV2Vm {
        &mut self.0
    }
}
