use lovm2_extend::prelude::*;

#[lovm2_function]
fn minus(op1: i64, op2: i64) -> i64 {
    op1 - op2
}

pub struct CustomVm(Vm);

impl CustomVm {
    pub fn new() -> Self {
        let mut inner = Vm::new();
        inner
            .add_function("minus".into(), create_callable(minus))
            .unwrap();
        Self(inner)
    }

    pub fn inner(&mut self) -> &mut Vm {
        &mut self.0
    }
}
