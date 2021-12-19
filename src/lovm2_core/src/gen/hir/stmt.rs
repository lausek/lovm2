//! Sum type for every HIR element

use super::*;

/// Sum type for every HIR element
#[derive(Clone)]
pub enum LV2Statement {
    AssignReference {
        target: LV2Expr,
        source: LV2Expr,
    },
    AssignVariable {
        target: LV2Variable,
        source: LV2Expr,
    },
    Branch(LV2Branch),
    /// Highlevel `break` statement
    Break,
    Call(LV2Call),
    /// Highlevel `continue` statement
    Continue,
    Drop {
        expr: LV2Expr,
    },
    Import {
        name: LV2Expr,
        namespaced: bool,
    },
    Interrupt {
        n: u16,
    },
    Repeat(LV2Repeat),
    Return {
        expr: LV2Expr,
    },
    ScopeGlobal {
        ident: LV2Variable,
    },
    ScopeLocal {
        ident: LV2Variable,
    },
}

impl LV2HirLowering for LV2Statement {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut LV2HirLoweringRuntime<'lir>) {
        match self {
            LV2Statement::AssignReference { target, source } => {
                target.lower(runtime);
                source.lower(runtime);
                runtime.emit(LirElement::Set);
            }
            LV2Statement::AssignVariable { target, source } => {
                source.lower(runtime);
                runtime.emit(LirElement::store(target));
            }
            LV2Statement::Branch(branch) => branch.lower(runtime),
            LV2Statement::Break => {
                let repeat_end = runtime.loop_mut().unwrap().end();
                runtime.emit(LirElement::jump(repeat_end));
            }
            LV2Statement::Call(call) => call.lower(runtime),
            LV2Statement::Continue => {
                let repeat_start = runtime.loop_mut().unwrap().start();
                runtime.emit(LirElement::jump(repeat_start));
            }
            LV2Statement::Drop { expr } => {
                expr.lower(runtime);
                runtime.emit(LirElement::Drop);
            }
            LV2Statement::Import { name, namespaced } => {
                name.lower(runtime);
                let elem = LirElement::Import {
                    namespaced: *namespaced,
                };
                runtime.emit(elem);
            }
            LV2Statement::Interrupt { n } => {
                runtime.emit(LirElement::Interrupt { n: *n });
            }
            LV2Statement::Repeat(repeat) => repeat.lower(runtime),
            LV2Statement::Return { expr } => {
                expr.lower(runtime);
                runtime.emit(LirElement::Ret);
            }
            LV2Statement::ScopeGlobal { ident } => {
                runtime.emit(LirElement::ScopeGlobal { ident });
            }
            LV2Statement::ScopeLocal { ident } => {
                runtime.emit(LirElement::ScopeLocal { ident });
            }
        }
    }
}

impl From<LV2Branch> for LV2Statement {
    fn from(branch: LV2Branch) -> Self {
        LV2Statement::Branch(branch)
    }
}

impl From<LV2Call> for LV2Statement {
    fn from(call: LV2Call) -> Self {
        LV2Statement::Call(call)
    }
}

impl From<LV2Expr> for LV2Statement {
    fn from(expr: LV2Expr) -> Self {
        LV2Statement::Drop { expr }
    }
}

impl From<LV2Repeat> for LV2Statement {
    fn from(repeat: LV2Repeat) -> Self {
        LV2Statement::Repeat(repeat)
    }
}

impl From<&mut LV2Repeat> for LV2Statement {
    fn from(repeat: &mut LV2Repeat) -> Self {
        LV2Statement::Repeat(repeat.clone())
    }
}
