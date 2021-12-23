//! Highlevel programming constructs.

use super::*;

/// Highlevel programming constructs.
#[derive(Clone)]
pub enum LV2Statement {
    /// Set the value of a [Ref][LV2Value::Ref].
    AssignReference {
        target: LV2Expr,
        source: LV2Expr,
    },
    /// Assign a variable to a new value.
    AssignVariable {
        target: LV2Variable,
        source: LV2Expr,
    },
    /// Conditional execution of code.
    Branch(LV2Branch),
    /// Highlevel `break` statement
    Break,
    Call(LV2Call),
    /// Highlevel `continue` statement
    Continue,
    /// Evaluate an expression and dispose the result.
    Drop {
        expr: LV2Expr,
    },
    /// Import some module into the runtime.
    Import {
        name: LV2Expr,
        namespaced: bool,
    },
    /// Trigger an interrupt.
    Interrupt {
        n: u16,
    },
    /// Repeating a block of code.
    Repeat(LV2Repeat),
    /// Explicitly return a value from a function.
    Return {
        expr: LV2Expr,
    },
    /// Change the scope of `ident` to global.
    ScopeGlobal {
        ident: LV2Variable,
    },
    /// Change the scope of `ident` to local.
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
