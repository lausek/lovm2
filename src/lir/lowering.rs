use super::{Label, LirElement};

pub struct LirLoweringRuntime {
    code: Vec<LirElement>,
}

impl LirLoweringRuntime {
    pub fn new() -> Self {
        Self { code: vec![] }
    }

    pub fn create_new_label(&mut self) -> Label {
        Label(0)
    }

    pub fn emit(&mut self /*, inx: Instruction */) {}
}
