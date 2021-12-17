//! Instructions for storing data

use super::*;

#[derive(Clone)]
pub enum AssignType {
    /// Assign to a reference
    Reference,
    /// Assign a variable
    Variable,
}

/// Storing data in various locations
#[derive(Clone)]
pub struct Assign {
    access: Access,
    expr: Expr,
    ty: AssignType,
}

impl Assign {
    pub fn var<U, T>(var: &U, expr: T) -> Self
    where
        U: Into<Variable> + Clone,
        T: Into<Expr>,
    {
        let var: Variable = var.clone().into();

        Self {
            access: var.into(),
            expr: expr.into(),
            ty: AssignType::Variable,
        }
    }

    /// Store data in a reference
    pub fn set<U, T>(access: &U, expr: T) -> Self
    where
        U: Into<Access> + Clone,
        T: Into<Expr>,
    {
        Self {
            access: access.clone().into(),
            expr: expr.into(),
            ty: AssignType::Reference,
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
    fn lower_reference<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        let target = &self.access.target;

        runtime.emit(LirElement::push_dynamic(target));

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

    fn lower_variable<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        let target = &self.access.target;

        self.expr.lower(runtime);

        match self.ty {
            AssignType::Variable => {
                runtime.emit(LirElement::store(target));
            }
            _ => unreachable!(),
        }
    }
}

impl HirLowering for Assign {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        match &self.ty {
            AssignType::Reference => self.lower_reference(runtime),
            AssignType::Variable => self.lower_variable(runtime),
        }
    }
}
