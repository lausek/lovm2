//! Return value from a `CodeObject`
//!
//! Every `CodeObject` implicitly returns `Nil` if no return value was given.

use super::*;

/// Return value from a `CodeObject`
#[derive(Clone)]
pub struct Return {
    expr: Expr,
}

impl Return {
    #[deprecated]
    pub fn nil() -> Self {
        Self {
            expr: Value::Nil.into(),
        }
    }

    #[deprecated]
    pub fn value<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self { expr: expr.into() }
    }
}

impl HirLowering for Return {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        self.expr.lower(runtime);
        runtime.emit(LirElement::Ret);
    }
}
