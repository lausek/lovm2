//! loads a module by name into the vm

use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};

/// loads a module by name into the vm
#[derive(Clone)]
pub struct Include {
    name: Expr,
}

impl Include {
    pub fn load<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        Self { name: name.into() }
    }
}

impl Lowering for Include {
    fn lower(self, runtime: &mut LoweringRuntime) {
        self.name.lower(runtime);
        runtime.emit(Instruction::Load);
    }
}
