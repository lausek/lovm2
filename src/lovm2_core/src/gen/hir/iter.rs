use super::*;

#[derive(Clone, Debug)]
pub enum Iter {
    Create(Box<Expr>),
    CreateRanged(Box<Expr>, Box<Expr>),
    HasNext(Box<Expr>),
    Next(Box<Expr>),
    Reverse(Box<Expr>),
}

impl Iter {
    pub fn create<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::Create(Box::new(expr.into()))
    }

    pub fn create_ranged<T, U>(from: T, to: U) -> Self
    where
        T: Into<Expr>,
        U: Into<Expr>,
    {
        Self::CreateRanged(Box::new(from.into()), Box::new(to.into()))
    }

    pub fn has_next<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::HasNext(Box::new(expr.into()))
    }

    pub fn next<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::Next(Box::new(expr.into()))
    }

    pub fn reverse<T>(expr: T) -> Self
    where
        T: Into<Expr>,
    {
        Self::Reverse(Box::new(expr.into()))
    }
}

impl HirLowering for Iter {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        match self {
            Self::Create(expr) => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterCreate);
            }
            Self::CreateRanged(from, to) => {
                from.lower(runtime);
                to.lower(runtime);
                runtime.emit(LirElement::IterCreateRanged);
            }
            Self::HasNext(expr) => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterHasNext);
            }
            Self::Next(expr) => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterNext);
            }
            Self::Reverse(expr) => {
                expr.lower(runtime);
                runtime.emit(LirElement::IterReverse);
            }
        }
    }
}
