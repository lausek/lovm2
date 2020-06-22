
use crate::hir::assign::Assign;
use crate::hir::branch::Branch;
use crate::hir::expr::Expr;
use crate::hir::repeat::Repeat;

pub enum HIRElement {
    Assign(Assign),
    Branch(Branch),
    Repeat(Repeat),
}

impl From<Assign> for HIRElement {
    fn from(assign: Assign) -> Self {
        HIRElement::Assign(assign)
    }
}

impl From<Branch> for HIRElement {
    fn from(branch: Branch) -> Self {
        HIRElement::Branch(branch)
    }
}

impl From<Repeat> for HIRElement {
    fn from(repeat: Repeat) -> Self {
        HIRElement::Repeat(repeat)
    }
}
