//! Instructions for storing data

use super::*;

/// Storing data in various locations
#[derive(Clone)]
pub enum Assign {
    Reference {
        target: Expr,
        source: Expr,
    },
    Variable {
        target: Variable,
        source: Expr
    }
}

impl Assign {
    pub fn var<U, T>(target: &U, source: T) -> Self
    where
        U: Into<Variable> + Clone,
        T: Into<Expr>,
    {
        Assign::Variable {
            target: target.clone().into(),
            source: source.into(),
        }
    }

    /// Store data in a reference
    pub fn set<U, T>(target: &U, source: T) -> Self
    where
        U: Into<Expr> + Clone,
        T: Into<Expr>,
    {
        Assign::Reference {
            target: target.clone().into(),
            source: source.into(),
        }
    }
}

impl HirLowering for Assign {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        match self {
            Assign::Reference { target, source} => {
                target.lower(runtime);
                source.lower(runtime);
                runtime.emit(LirElement::Set);
            }
            Assign::Variable { target, source } => {
                source.lower(runtime);
                runtime.emit(LirElement::store(target));
            }
        }
    }
}
