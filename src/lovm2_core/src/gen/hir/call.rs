//! Execute a `CodeObject` by name using the given arguments

use super::*;

/// Execute a `CodeObject` by name using the given arguments
#[derive(Clone, Debug)]
pub struct LV2Call {
    name: LV2Variable,
    args: Vec<LV2Expr>,
}

impl LV2Call {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<LV2Variable>,
    {
        Self {
            args: vec![],
            name: name.into(),
        }
    }

    pub fn with_args<T>(name: T, args: Vec<LV2Expr>) -> Self
    where
        T: Into<LV2Variable>,
    {
        Self {
            args,
            name: name.into(),
        }
    }

    pub fn arg<T>(mut self, expr: T) -> Self
    where
        T: Into<LV2Expr>,
    {
        self.args.push(expr.into());
        self
    }
}

impl HirLowering for LV2Call {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    {
        // calling convention is pascal-style i.e. f(a, b)
        // will be lowered as:
        //  push a
        //  push b
        for arg in self.args.iter() {
            arg.lower(runtime);
        }

        runtime.emit(LirElement::call(&self.name, self.args.len() as u8));
    }
}
