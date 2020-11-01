use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::value::Value;

#[derive(Clone)]
pub struct Return {
    expr: Expr,
}

impl Return {
    pub fn nil() -> Self {
        Self {
            expr: Value::Nil.into(),
        }
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
