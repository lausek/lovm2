use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::var::Variable;

pub enum AssignScope {
    Local,
    Global
}

pub struct Assign {
    expr: Expr,
    variable: Variable,
    scope: AssignScope,
}

impl Assign {
    pub fn local(variable: Variable, expr: Expr) -> Self {
        Self {
            expr,
            variable,
            scope: AssignScope::Local,
        }
    }

    pub fn global(variable: Variable, expr: Expr) -> Self {
        Self {
            expr,
            variable,
            scope: AssignScope::Global,
        }
    }
}

impl Lowering for Assign {
    fn lower(self, runtime: &mut LoweringRuntime) {
        self.expr.lower(runtime);
        match self.scope {
            AssignScope::Local => {
                let lidx = runtime.index_local(&self.variable);
                runtime.code.push(Instruction::Movel(lidx as u16));
            }
            AssignScope::Global => {
                let gidx = runtime.index_global(&self.variable);
                runtime.code.push(Instruction::Moveg(gidx as u16));
            }
        };
    }
}
