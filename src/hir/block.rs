//! general collection for program statements

use crate::hir::branch::Branch;
use crate::hir::expr::Expr;
use crate::hir::lowering::{HirLowering, HirLoweringRuntime};
use crate::hir::repeat::Repeat;
use crate::hir::HirElement;

#[derive(Clone)]
pub struct Block(Vec<HirElement>);

impl Block {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn from(&mut self, block: Block) {
        self.0 = block.0;
    }

    pub fn last_mut(&mut self) -> Option<&mut HirElement> {
        self.0.last_mut()
    }

    pub fn with<T>(mut self, hir: T) -> Self
    where
        T: Into<HirElement>,
    {
        self.0.push(hir.into());
        self
    }

    pub fn push<T>(&mut self, hir: T)
    where
        T: Into<HirElement>,
    {
        self.0.push(hir.into());
    }

    pub fn branch(&mut self) -> &mut Branch {
        self.push(Branch::new());
        match self.last_mut().unwrap() {
            HirElement::Branch(ref mut r) => r,
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
            HirElement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }
}

impl std::iter::IntoIterator for Block {
    type Item = HirElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl HirLowering for Block {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        for element in self.0.into_iter() {
            element.lower(runtime);
        }
    }
}
