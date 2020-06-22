use crate::hir::lowering::{Lowering, LoweringRuntime};

pub struct Assign {}

impl Lowering for Assign {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}
