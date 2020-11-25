//! trigger a vm interrupt

use crate::bytecode::Instruction;
use crate::hir::lowering::{HirLowering, HirLoweringRuntime};

#[derive(Clone)]
pub struct Interrupt {
    n: u16,
}

impl Interrupt {
    pub fn new(n: u16) -> Self {
        Self { n }
    }
}

impl HirLowering for Interrupt {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        runtime.emit(Instruction::Interrupt(self.n));
    }
}
