use crate::hir::lowering::{Lowering, LoweringRuntime};

pub struct Repeat {}

impl Lowering for Repeat {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}
