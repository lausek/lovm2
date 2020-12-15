//! execute a `CodeObject` by name using the given arguments

use super::*;

#[derive(Clone, Debug)]
pub struct Call {
    name: Variable,
    args: Vec<Expr>,
    keep_value: bool,
}

impl Call {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Variable>,
    {
        Self {
            args: vec![],
            name: name.into(),
            keep_value: false,
        }
    }

    pub fn with_args<T>(name: T, args: Vec<Expr>) -> Self
    where
        T: Into<Variable>,
    {
        Self {
            args,
            name: name.into(),
            keep_value: false,
        }
    }

    pub fn arg<T>(mut self, expr: T) -> Self
    where
        T: Into<Expr>,
    {
        self.args.push(expr.into());
        self
    }

    pub fn keep(&mut self, keep_value: bool) {
        self.keep_value = keep_value;
    }
}

impl HirLowering for Call {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        // calling convention is pascal-style i.e. f(a, b)
        // will be lowered as:
        //  push a
        //  push b
        let argn = self.args.len();
        for arg in self.args {
            arg.lower(runtime);
        }

        runtime.emit(LirElement::call(argn as u8, self.name));

        // every call has to leave a return value on stack.
        // if that value isn't needed e.g. for assignments, we
        // need to get rid of it.
        if !self.keep_value {
            runtime.emit(LirElement::Drop);
        }
    }
}
