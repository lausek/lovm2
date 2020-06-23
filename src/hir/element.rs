
use crate::hir::assign::Assign;
use crate::hir::branch::Branch;
use crate::hir::expr::Expr;
use crate::hir::repeat::{Break, Continue, Repeat};

pub enum HIRElement {
    Assign(Assign),
    Branch(Branch),
    Break(Break),
    Continue(Continue),
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

impl From<Break> for HIRElement {
    fn from(cmd: Break) -> Self {
        HIRElement::Break(cmd)
    }
}

impl From<Continue> for HIRElement {
    fn from(cmd: Continue) -> Self {
        HIRElement::Continue(cmd)
    }
}

impl From<Repeat> for HIRElement {
    fn from(repeat: Repeat) -> Self {
        HIRElement::Repeat(repeat)
    }
}
