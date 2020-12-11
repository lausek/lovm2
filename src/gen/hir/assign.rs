//! instructions for storing data

use super::*;

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
    pub fn local<U, T>(var: &U, expr: T) -> Self
    where
        U: Into<Variable> + Clone,
        T: Into<Expr>,
    {
        let var: Variable = var.clone().into();
        Self {
            expr: expr.into(),
            access: var.into(),
            ty: AssignType::StaticLocal,
        }
    }

    pub fn global<U, T>(var: &U, expr: T) -> Self
    where
        U: Into<Variable> + Clone,
        T: Into<Expr>,
    {
        let var: Variable = var.clone().into();
        Self {
            expr: expr.into(),
            access: var.into(),
            ty: AssignType::StaticGlobal,
        }
    }

    pub fn set<U, T>(access: &U, expr: T) -> Self
    where
        U: Into<Access> + Clone,
        T: Into<Expr>,
    {
        Self {
            expr: expr.into(),
            access: access.clone().into(),
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
    fn lower_dynamic(self, runtime: &mut HirLoweringRuntime) {
        let variable = self.access.target;

        // push (initial) target onto stack
        if runtime.has_local(&variable) {
            runtime.emit(LirElement::push_dynamic(Scope::Local, variable));
        } else {
            runtime.emit(LirElement::push_dynamic(Scope::Global, variable));
        }

        let mut key_it = self.access.keys.into_iter().peekable();
        // push key onto stack
        if let Some(key) = key_it.next() {
            key.lower(runtime);
            runtime.emit(LirElement::Getr);

            while key_it.peek().is_some() {
                let key = key_it.next().unwrap();
                key.lower(runtime);
                runtime.emit(LirElement::Getr);
            }
        }

        // push value onto stack
        self.expr.lower(runtime);

        runtime.emit(LirElement::Set);
    }

    fn lower_static(self, runtime: &mut HirLoweringRuntime) {
        self.expr.lower(runtime);

        let variable = self.access.target;
        match self.ty {
            AssignType::StaticLocal => {
                runtime.emit(LirElement::store(Scope::Local, variable));
            }
            AssignType::StaticGlobal => {
                runtime.emit(LirElement::store(Scope::Global, variable));
            }
            _ => unimplemented!(),
        }
    }
}

impl HirLowering for Assign {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        match &self.ty {
            AssignType::StaticLocal | AssignType::StaticGlobal => self.lower_static(runtime),
            AssignType::Dynamic => self.lower_dynamic(runtime),
        }
    }
}
