//! Instructions for storing data

use super::*;

#[derive(Clone)]
pub enum AssignType {
    /// Assign a local variable
    StaticLocal,
    /// Assign a global variable
    StaticGlobal,
    /// Assign to a reference
    Dynamic,
    /// Overwrite a variable with the scope resolved while lowering
    Update,
}

/// Storing data in various locations
#[derive(Clone)]
pub struct Assign {
    expr: Expr,
    access: Access,
    ty: AssignType,
}

impl Assign {
    /// Assign data to a local variable
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

    /// Assign data to a global variable
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

    /// Store data in a reference
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

    /// Increment a variable by one
    pub fn increment<T>(var: &T) -> Self
    where
        T: Into<Variable> + Clone,
    {
        let var: Variable = var.clone().into();
        Self {
            expr: Expr::add(var.clone(), 1),
            access: var.into(),
            ty: AssignType::Update,
        }
    }

    /// Decrement a variable by one
    pub fn decrement<T>(var: &T) -> Self
    where
        T: Into<Variable> + Clone,
    {
        let var: Variable = var.clone().into();
        Self {
            expr: Expr::sub(var.clone(), 1),
            access: var.into(),
            ty: AssignType::Update,
        }
    }
}

/// Consecutive read on a `List` or `Dict`
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

impl <T> From<T> for Access
    where T: std::borrow::Borrow<Variable>
{
    fn from(target: T) -> Self {
        Self {
            keys: vec![],
            target: target.borrow().clone(),
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
            runtime.emit(LirElement::RGet);

            while key_it.peek().is_some() {
                let key = key_it.next().unwrap();
                key.lower(runtime);
                runtime.emit(LirElement::RGet);
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

    fn lower_update(self, runtime: &mut HirLoweringRuntime) {
        self.expr.lower(runtime);

        let var = self.access.target;
        if runtime.has_local(&var) {
            runtime.emit(LirElement::store(Scope::Local, var));
        } else {
            runtime.emit(LirElement::store(Scope::Global, var));
        }
    }
}

impl HirLowering for Assign {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        match &self.ty {
            AssignType::StaticLocal | AssignType::StaticGlobal => self.lower_static(runtime),
            AssignType::Dynamic => self.lower_dynamic(runtime),
            AssignType::Update => self.lower_update(runtime),
        }
    }
}
