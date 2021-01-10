//! Create a new `List` containing references to values on the target `List`

use super::*;

/// Create a new `List` containing references to values on the target `List`
#[derive(Clone, Debug)]
pub struct Slice {
    target: Box<Expr>,
    start: Option<Box<Expr>>,
    end: Option<Box<Expr>>,
}

impl Slice {
    pub fn new<T>(target: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            target: Box::new(target.into()),
            start: None,
            end: None,
        }
    }

    pub fn start<T>(mut self, start: T) -> Self
    where
        T: Into<Expr>,
    {
        self.start = Some(Box::new(start.into()));
        self
    }

    pub fn end<T>(mut self, end: T) -> Self
    where
        T: Into<Expr>,
    {
        self.end = Some(Box::new(end.into()));
        self
    }
}

impl HirLowering for Slice {
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        self.target.lower(runtime);

        if let Some(start) = &self.start {
            start.lower(runtime);
        } else {
            runtime.emit(LirElement::push_constant_owned(Value::Nil));
        }

        if let Some(end) = &self.end {
            end.lower(runtime);
        } else {
            runtime.emit(LirElement::push_constant_owned(Value::Nil));
        }

        runtime.emit(LirElement::Slice);
    }
}
