use crate::hir::lowering::{Lowering, LoweringRuntime};

pub struct Repeat {}

impl Lowering for Repeat {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}

pub struct Break {}

impl Lowering for Break {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}

pub struct Continue {}

impl Lowering for Continue {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}
