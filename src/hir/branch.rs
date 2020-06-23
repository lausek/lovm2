use crate::block::Block;
use crate::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};

pub struct Branch {
    branches: Vec<(Expr, Block)>,
    default: Option<Block>,
}

impl Branch {
    pub fn new() -> Self {
        Self {
            branches: vec![],
            default: None,
        }
    }
}

impl Lowering for Branch {
    fn lower(self, _runtime: &mut LoweringRuntime) {}
}
