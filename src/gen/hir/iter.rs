use crate::vm::*;

use super::*;

#[repr(u16)]
#[derive(Clone, Debug)]
enum IterMethod {
    Create = LOVM2_INT_ITER_CREATE,
    HasNext = LOVM2_INT_ITER_HAS_NEXT,
    Next = LOVM2_INT_ITER_NEXT,
    Reverse = LOVM2_INT_ITER_REVERSE,
}

#[derive(Clone, Debug)]
pub struct Iter {
    method: IterMethod,
    expr: Box<Expr>,
}

impl Iter {
    pub fn create<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            method: IterMethod::Create,
            expr: Box::new(expr.into()),
        }
    }

    pub fn has_next<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            method: IterMethod::HasNext,
            expr: Box::new(expr.into()),
        }
    }

    pub fn next<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            method: IterMethod::Next,
            expr: Box::new(expr.into()),
        }
    }

    pub fn reverse<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            method: IterMethod::Reverse,
            expr: Box::new(expr.into()),
        }
    }
}

impl HirLowering for Iter {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        self.expr.lower(runtime);
        runtime.emit(LirElement::Interrupt(self.method as u16));
    }
}
