//! loads a module by name into the vm

use super::*;

/// loads a module by name into the vm
#[derive(Clone)]
pub struct Include {
    name: Expr,
}

impl Include {
    pub fn load<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        Self { name: name.into() }
    }
}

impl HirLowering for Include {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        self.name.lower(runtime);
        runtime.emit(LirElement::Load);
    }
}
