use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::var::Variable;

pub struct Call {
    name: Variable,
    args: Vec<Expr>,
}

impl Lowering for Call {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}
