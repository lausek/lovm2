//! Return value from a `CodeObject`
//!
//! Every `CodeObject` implicitly returns `Nil` if no return value was given.

use super::*;

#[derive(Clone)]
pub struct Return {
    expr: Expr,
}

impl Return {
    pub fn nil() -> Self {
        Self {
            expr: Value::Nil.into(),
        }
    }

    pub fn value<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self { expr: expr.into() }
    }
}

impl HirLowering for Return {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        self.expr.lower(runtime);
        runtime.emit(LirElement::Ret);
    }
}
