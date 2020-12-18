//! Create a new `List` containing references to values on the target `List`

use super::*;

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
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        self.target.lower(runtime);

        if let Some(start) = self.start {
            start.lower(runtime);
        } else {
            Expr::from(Value::Nil).lower(runtime);
        }

        if let Some(end) = self.end {
            end.lower(runtime);
        } else {
            Expr::from(Value::Nil).lower(runtime);
        }

        runtime.emit(LirElement::Slice);
    }
}
