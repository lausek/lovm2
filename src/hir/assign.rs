use crate::bytecode::Instruction;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::var::Variable;

#[derive(Clone)]
pub enum AssignType {
    StaticLocal,
    StaticGlobal,
    Dynamic,
}

#[derive(Clone)]
pub struct Assign {
    expr: Expr,
    access: Access,
    ty: AssignType,
}

impl Assign {
    pub fn local<U, T>(var: U, expr: T) -> Self
    where
        U: Into<Variable>,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            access: var.into().into(),
            ty: AssignType::StaticLocal,
        }
    }

    pub fn global<U, T>(var: U, expr: T) -> Self
    where
        U: Into<Variable>,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            access: var.into().into(),
            ty: AssignType::StaticGlobal,
        }
    }

    pub fn set<U, T>(access: U, expr: T) -> Self
    where
        U: Into<Access>,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            access: access.into(),
            ty: AssignType::Dynamic,
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

impl Assign {
    fn lower_dynamic(self, runtime: &mut LoweringRuntime) {
        let variable = self.access.target;

        // push (initial) target onto stack
        if runtime.has_local(&variable) {
            let lidx = runtime.index_local(&variable);
            runtime.emit(Instruction::Pushl(lidx as u16));
        } else {
            let gidx = runtime.index_global(&variable);
            runtime.emit(Instruction::Pushg(gidx as u16));
        }

        let mut key_it = self.access.keys.into_iter().peekable();
        // push key onto stack
        if let Some(key) = key_it.next() {
            key.lower(runtime);
            runtime.emit(Instruction::Getr);

            while key_it.peek().is_some() {
                let key = key_it.next().unwrap();
                key.lower(runtime);
                runtime.emit(Instruction::Getr);
            }
        }

        // push value onto stack
        self.expr.lower(runtime);

        runtime.emit(Instruction::Set);
    }

    fn lower_static(self, runtime: &mut LoweringRuntime) {
        self.expr.lower(runtime);

        let variable = self.access.target;
        match self.ty {
            AssignType::StaticLocal => {
                let lidx = runtime.index_local(&variable);
                runtime.emit(Instruction::Movel(lidx as u16));
            }
            AssignType::StaticGlobal => {
                let gidx = runtime.index_global(&variable);
                runtime.emit(Instruction::Moveg(gidx as u16));
            }
            _ => unimplemented!(),
        }
    }
}

impl Lowering for Assign {
    fn lower(self, runtime: &mut LoweringRuntime) {
        match &self.ty {
            AssignType::StaticLocal | AssignType::StaticGlobal => self.lower_static(runtime),
            AssignType::Dynamic => self.lower_dynamic(runtime),
        }
    }
}
