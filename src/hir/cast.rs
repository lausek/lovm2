use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::value::cast::{RUVALUE_BOOL_TY, RUVALUE_FLOAT_TY, RUVALUE_INT_TY, RUVALUE_STR_TY};

#[derive(Clone, Debug)]
pub struct Cast {
    tid: u16,
    expr: Box<Expr>,
}

impl Cast {
    fn new(tid: u16, expr: Expr) -> Self {
        Self {
            tid,
            expr: Box::new(expr),
        }
    }

    pub fn to_bool<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(RUVALUE_BOOL_TY, expr.into())
    }

    pub fn to_float<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(RUVALUE_FLOAT_TY, expr.into())
    }

    pub fn to_integer<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(RUVALUE_INT_TY, expr.into())
    }

    pub fn to_str<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::new(RUVALUE_STR_TY, expr.into())
    }
}

impl Lowering for Cast {
    fn lower(self, runtime: &mut LoweringRuntime) {
        self.expr.lower(runtime);
        runtime.emit(Instruction::Cast(self.tid));
    }
}
