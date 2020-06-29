use crate::hir::branch::Branch;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::hir::repeat::Repeat;
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

    pub fn last_mut(&mut self) -> Option<&mut HIRElement> {
        self.0.last_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<HIRElement> {
        self.0.into_iter()
    }

    pub fn with<T>(mut self, hir: T) -> Self
    where
        T: Into<HIRElement>,
    {
        self.0.push(hir.into());
        self
    }

    pub fn push<T>(&mut self, hir: T)
    where
        T: Into<HIRElement>,
    {
        self.0.push(hir.into());
    }

    pub fn branch(&mut self) -> &mut Branch {
        self.push(Branch::new());
        match self.last_mut().unwrap() {
            HIRElement::Branch(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat(&mut self, condition: Option<Expr>) -> &mut Repeat {
        if let Some(condition) = condition {
            self.push(Repeat::until(condition));
        } else {
            self.push(Repeat::endless());
        }
        match self.last_mut().unwrap() {
            HIRElement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }
}

impl Lowering for Block {
    fn lower(self, runtime: &mut LoweringRuntime) {
        for element in self.0.into_iter() {
            element.lower(runtime);
        }
    }
}
