use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::var::Variable;

#[derive(Clone)]
pub enum AssignScope {
    Local,
    Global,
}

#[derive(Clone)]
pub struct Assign {
    expr: Expr,
    locator: AssignLocator,
    scope: AssignScope,
}

impl Assign {
    pub fn local<U, T>(locator: U, expr: T) -> Self
    where
        U: Into<AssignLocator>,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            locator: locator.into(),
            scope: AssignScope::Local,
        }
    }

    pub fn global<U, T>(locator: U, expr: T) -> Self
    where
        U: Into<AssignLocator>,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            locator: locator.into(),
            scope: AssignScope::Global,
        }
    }
}

#[derive(Clone)]
pub enum AssignLocator {
    Access(Vec<Expr>),
    Variable(Variable),
}

impl From<Variable> for AssignLocator {
    fn from(var: Variable) -> Self {
        Self::Variable(var)
    }
}

impl From<Expr> for AssignLocator {
    fn from(expr: Expr) -> Self {
        if let Expr::Access(fields) = expr {
            Self::Access(fields)
        } else {
            unimplemented!()
        }
    }
}

impl Lowering for Assign {
    fn lower(self, runtime: &mut LoweringRuntime) {
        self.expr.lower(runtime);

        match self.locator {
            AssignLocator::Access(fields) => {
                let mut field_it = fields.iter();
                field_it.next().unwrap();

                runtime.emit(Instruction::Set);
            }
            AssignLocator::Variable(variable) => match self.scope {
                AssignScope::Local => {
                    let lidx = runtime.index_local(&variable);
                    runtime.emit(Instruction::Movel(lidx as u16));
                }
                AssignScope::Global => {
                    let gidx = runtime.index_global(&variable);
                    runtime.emit(Instruction::Moveg(gidx as u16));
                }
            },
        }
    }
}
