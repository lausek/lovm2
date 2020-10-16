use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::value::RuValue;
use crate::var::Variable;

#[derive(Clone)]
pub enum AssignScope {
    Local,
    Global,
}

#[derive(Clone)]
pub struct Assign {
    expr: Expr,
    access: Access,
    scope: AssignScope,
}

impl Assign {
    pub fn local<U, T>(access: U, expr: T) -> Self
    where
        U: Into<Access>,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            access: access.into(),
            scope: AssignScope::Local,
        }
    }

    pub fn global<U, T>(access: U, expr: T) -> Self
    where
        U: Into<Access>,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            access: access.into(),
            scope: AssignScope::Global,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Access {
    pub keys: Vec<Expr>,
    pub target: Variable,
}

impl Access {
    pub fn new(target: Variable, keys: Vec<Expr>) -> Self {
        Self { keys, target }
    }

    pub fn target(target: Variable) -> Self {
        Self {
            keys: vec![],
            target,
        }
    }

    pub fn at<T>(mut self, key: T) -> Self
    where
        T: Into<Expr>,
    {
        self.keys.push(key.into());
        self
    }
}

impl From<Variable> for Access {
    fn from(target: Variable) -> Self {
        Self {
            keys: vec![],
            target,
        }
    }
}

impl From<Expr> for Access {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Access(access) => access,
            Expr::Variable(var) => var.into(),
            _ => unimplemented!(),
        }
    }
}

impl Lowering for Assign {
    fn lower(self, runtime: &mut LoweringRuntime) {
        if self.access.keys.is_empty() {
            let needs_box = match &self.expr {
                Expr::DynamicValue(_) => true,
                Expr::Value(RuValue::Dict(_)) => true,
                Expr::Value(RuValue::List(_)) => true,
                _ => false,
            };

            self.expr.lower(runtime);

            if needs_box {
                runtime.emit(Instruction::Box);
            }

            let variable = self.access.target;
            match self.scope {
                AssignScope::Local => {
                    let lidx = runtime.index_local(&variable);
                    runtime.emit(Instruction::Movel(lidx as u16));
                }
                AssignScope::Global => {
                    let gidx = runtime.index_global(&variable);
                    runtime.emit(Instruction::Moveg(gidx as u16));
                }
            }
        } else {
            let variable = self.access.target;
            let mut key_it = self.access.keys.into_iter().peekable();

            // push (initial) target onto stack
            if runtime.has_local(&variable) {
                let lidx = runtime.index_local(&variable);
                runtime.emit(Instruction::Pushl(lidx as u16));
            } else {
                let gidx = runtime.index_global(&variable);
                runtime.emit(Instruction::Pushg(gidx as u16));
            }

            // push key onto stack
            let key = key_it.next().unwrap();
            key.lower(runtime);

            while key_it.peek().is_some() {
                runtime.emit(Instruction::Getr);
                let key = key_it.next().unwrap();
                key.lower(runtime);
            }

            // push value onto stack
            self.expr.lower(runtime);

            runtime.emit(Instruction::Set);
        }
    }
}
