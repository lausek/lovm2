use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::hir::HIRElement;

#[derive(Clone)]
pub struct Block(Vec<HIRElement>);

impl Block {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push<T>(mut self, hir: T) -> Self
    where
        T: Into<HIRElement>,
    {
        self.0.push(hir.into());
        self
    }

    pub fn push_inplace<T>(&mut self, hir: T)
    where
        T: Into<HIRElement>,
    {
        self.0.push(hir.into());
    }
}

impl Lowering for Block {
    fn lower(self, runtime: &mut LoweringRuntime) {
        for element in self.0.into_iter() {
            element.lower(runtime);
        }
    }
}
