use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::hir::HIRElement;

pub struct Block(Vec<HIRElement>);

impl Lowering for Block {
    fn lower(self, _runtime: &mut LoweringRuntime) {}
}
