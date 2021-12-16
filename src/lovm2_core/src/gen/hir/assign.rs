//! Instructions for storing data

use super::*;

#[derive(Clone)]
pub enum AssignType {
    /// Assign a local variable
    StaticLocal,
    /// Assign a global variable
    StaticGlobal,
    /// Assign a variable
    StaticVar,
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

    pub fn var<U, T>(var: &U, expr: T) -> Self
    where
        U: Into<Variable> + Clone,
        T: Into<Expr>,
    {
        let var: Variable = var.clone().into();

        Self {
            expr: expr.into(),
            access: var.into(),
            ty: AssignType::StaticVar,
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

impl<T> From<T> for Access
where
    T: std::borrow::Borrow<Variable>,
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
    fn lower_dynamic<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        let target = &self.access.target;

        // push (initial) target onto stack
        if runtime.has_local(target) {
            runtime.emit(LirElement::push_dynamic(Scope::Local, target));
        } else {
            runtime.emit(LirElement::push_dynamic(Scope::Global, target));
        }

        let mut key_it = self.access.keys.iter().peekable();

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

    fn lower_static<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        let target = &self.access.target;

        self.expr.lower(runtime);

        match self.ty {
            AssignType::StaticLocal => {
                runtime.emit(LirElement::ScopeLocal { ident: target });
                runtime.emit(LirElement::store(Scope::Local, target));
            }
            AssignType::StaticGlobal => {
                runtime.emit(LirElement::ScopeGlobal { ident: target });
                runtime.emit(LirElement::store(Scope::Global, target));
            }
            AssignType::StaticVar => {
                runtime.emit(LirElement::store(Scope::Local, target));
            }
            _ => unreachable!(),
        }
    }

    fn lower_update<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        let target = &self.access.target;

        self.expr.lower(runtime);

        if runtime.has_local(&target) {
            runtime.emit(LirElement::store(Scope::Local, target));
        } else {
            runtime.emit(LirElement::store(Scope::Global, target));
        }
    }
}

impl HirLowering for Assign {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        match &self.ty {
            AssignType::StaticLocal | AssignType::StaticGlobal | AssignType::StaticVar => self.lower_static(runtime),
            AssignType::Dynamic => self.lower_dynamic(runtime),
            AssignType::Update => self.lower_update(runtime),
        }
    }
}
