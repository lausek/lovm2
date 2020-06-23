use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::var::Variable;

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
            name: name.into(),
            args: vec![],
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

impl Lowering for Call {
    fn lower(self, runtime: &mut LoweringRuntime) {
        let argn = self.args.len();
        for arg in self.args {
            arg.lower(runtime);
        }
        let gidx = runtime.index_global(&self.name);
        runtime.emit(Instruction::Call(argn as u8, gidx as u16));
    }
}
