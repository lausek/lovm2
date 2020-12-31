//! Execute a `CodeObject` by name using the given arguments

use super::*;

/// Execute a `CodeObject` by name using the given arguments
#[derive(Clone, Debug)]
pub struct Call {
    name: Variable,
    args: Vec<Expr>,
}

impl Call {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Variable>,
    {
        Self {
            args: vec![],
            name: name.into(),
        }
    }

    pub fn with_args<T>(name: T, args: Vec<Expr>) -> Self
    where
        T: Into<Variable>,
    {
        Self {
            args,
            name: name.into(),
        }
    }

    pub fn arg<T>(mut self, expr: T) -> Self
    where
        T: Into<Expr>,
    {
        self.args.push(expr.into());
        self
    }
}

impl HirLowering for Call {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        // calling convention is pascal-style i.e. f(a, b)
        // will be lowered as:
        //  push a
        //  push b
        for arg in self.args.iter() {
            arg.lower(runtime);
        }

        runtime.emit(LirElement::call(self.args.len() as u8, &self.name));
    }
}
