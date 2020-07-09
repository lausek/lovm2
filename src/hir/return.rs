use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};

#[derive(Clone)]
pub struct Return {
    expr: Expr,
}

impl Return {
    pub fn none() -> Self {
        Self { expr: 0.into() }
    }

    pub fn value<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self { expr: expr.into() }
    }
}

impl Lowering for Return {
    fn lower(self, runtime: &mut LoweringRuntime) {
        self.expr.lower(runtime);
        runtime.emit(Instruction::Ret);
    }
}
