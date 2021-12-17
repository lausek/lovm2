//! Sum type for every HIR element

use super::*;

/// Sum type for every HIR element
#[derive(Clone)]
pub enum HirElement {
    Assign(Assign),
    Branch(Branch),
    Break(Break),
    Call(Call),
    Continue(Continue),
    Include(Include),
    Interrupt {
        n: u16,
    },
    Repeat(Repeat),
    Return {
        expr: Expr,
    },
    ScopeGlobal {
        ident: Variable,
    },
    ScopeLocal {
        ident: Variable,
    },
}

impl HirLowering for HirElement {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        match self {
            HirElement::Assign(assign) => assign.lower(runtime),
            HirElement::Branch(branch) => branch.lower(runtime),
            HirElement::Break(cmd) => cmd.lower(runtime),
            HirElement::Call(call) => call.lower(runtime),
            HirElement::Continue(cmd) => cmd.lower(runtime),
            HirElement::Include(include) => include.lower(runtime),
            HirElement::Interrupt { n} => {
                runtime.emit(LirElement::Interrupt { n: *n });
            }
            HirElement::Repeat(repeat) => repeat.lower(runtime),
            HirElement::Return { expr} => {
                expr.lower(runtime);
                runtime.emit(LirElement::Ret);
            },
            HirElement::ScopeGlobal { ident } => {
                runtime.emit(LirElement::ScopeGlobal { ident });
            }
            HirElement::ScopeLocal { ident } => {
                runtime.emit(LirElement::ScopeLocal { ident });
            }
        }
    }
}

impl From<Assign> for HirElement {
    fn from(assign: Assign) -> Self {
        HirElement::Assign(assign)
    }
}

impl From<Branch> for HirElement {
    fn from(branch: Branch) -> Self {
        HirElement::Branch(branch)
    }
}

impl From<Break> for HirElement {
    fn from(cmd: Break) -> Self {
        HirElement::Break(cmd)
    }
}

impl From<Call> for HirElement {
    fn from(call: Call) -> Self {
        HirElement::Call(call)
    }
}

impl From<Continue> for HirElement {
    fn from(cmd: Continue) -> Self {
        HirElement::Continue(cmd)
    }
}

impl From<Include> for HirElement {
    fn from(include: Include) -> Self {
        HirElement::Include(include)
    }
}

impl From<Repeat> for HirElement {
    fn from(repeat: Repeat) -> Self {
        HirElement::Repeat(repeat)
    }
}

impl From<&mut Repeat> for HirElement {
    fn from(repeat: &mut Repeat) -> Self {
        HirElement::Repeat(repeat.clone())
    }
}