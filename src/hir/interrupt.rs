use crate::bytecode::Instruction;
use crate::hir::lowering::{Lowering, LoweringRuntime};

#[derive(Clone)]
pub struct Interrupt {
    n: u16,
}

impl Interrupt {
    pub fn new(n: u16) -> Self {
        Self { n }
    }
}

impl Lowering for Interrupt {
    fn lower(self, runtime: &mut LoweringRuntime) {
        runtime.emit(Instruction::Interrupt(self.n));
    }
}
