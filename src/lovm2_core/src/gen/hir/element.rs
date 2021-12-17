//! Sum type for every HIR element

use super::*;

/// Sum type for every HIR element
#[derive(Clone)]
pub enum HirElement {
    AssignReference {
        target: Expr,
        source: Expr,
    },
    AssignVariable {
        target: Variable,
        source: Expr,
    },
    Branch(Branch),
    /// Highlevel `break` statement
    Break,
    Call(Call),
    /// Highlevel `continue` statement
    Continue,
    Import {
        name: Expr,
        namespaced: bool,
    },
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
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    {
        match self {
            HirElement::AssignReference { target, source } => {
                target.lower(runtime);
                source.lower(runtime);
                runtime.emit(LirElement::Set);
            }
            HirElement::AssignVariable { target, source } => {
                source.lower(runtime);
                runtime.emit(LirElement::store(target));
            }
            HirElement::Branch(branch) => branch.lower(runtime),
            HirElement::Break => {
                let repeat_end = runtime.loop_mut().unwrap().end();
                runtime.emit(LirElement::jump(repeat_end));
            }
            HirElement::Call(call) => call.lower(runtime),
            HirElement::Continue => {
                let repeat_start = runtime.loop_mut().unwrap().start();
                runtime.emit(LirElement::jump(repeat_start));
            }
            HirElement::Import { name, namespaced} => {
                name.lower(runtime);
                let elem = LirElement::Import { namespaced: *namespaced };
                runtime.emit(elem);
            }
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

impl From<Branch> for HirElement {
    fn from(branch: Branch) -> Self {
        HirElement::Branch(branch)
    }
}

impl From<Call> for HirElement {
    fn from(call: Call) -> Self {
        HirElement::Call(call)
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