//! Do type conversion on a lowered `Expr` at runtime

use super::*;

use crate::value::ValueType;

#[derive(Clone, Debug)]
pub struct Cast {
    ty: ValueType,
    expr: Box<Expr>,
}

impl Cast {
    fn new(ty: ValueType, expr: Expr) -> Self {
        Self {
            ty,
            expr: Box::new(expr),
        }
    }

    pub fn to_bool<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(ValueType::Bool, expr.into())
    }

    pub fn to_float<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(ValueType::Float, expr.into())
    }

    pub fn to_integer<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(ValueType::Int, expr.into())
    }

    pub fn to_str<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(ValueType::Str, expr.into())
    }
}

impl HirLowering for Cast {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        self.expr.lower(runtime);
        runtime.emit(LirElement::cast(self.ty));
    }
}
