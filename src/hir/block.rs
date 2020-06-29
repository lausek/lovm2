use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::hir::HIRElement;

#[derive(Clone)]
pub struct Block(Vec<HIRElement>);

impl Block {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn from(&mut self, block: Block) {
        self.0 = block.0;
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

    pub fn last_mut(&mut self) -> Option<&mut HIRElement> {
        self.0.last_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<HIRElement> {
        self.0.into_iter()
    }
}

impl Lowering for Block {
    fn lower(self, runtime: &mut LoweringRuntime) {
        for element in self.0.into_iter() {
            element.lower(runtime);
        }
    }
}
