use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::var::Variable;

pub struct Assign {
    expr: Expr,
    variable: Variable,
}

impl Assign {
    pub fn new(variable: Variable, expr: Expr) -> Self {
        Self {
            expr,
            variable,
        }
    }
}

impl Lowering for Assign {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}
