use crate::hir::HIRElement;
use crate::hir::lowering::{Lowering, LoweringRuntime};

pub struct Block(Vec<HIRElement>);

impl Lowering for Block {
    fn lower(self, runtime: &mut LoweringRuntime) {}
}
