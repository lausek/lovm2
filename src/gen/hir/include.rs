//! loads a module by name into the vm

use super::*;

/// loads a module by name into the vm
#[derive(Clone)]
pub struct Include {
    name: Expr,
    import: bool,
}

impl Include {
    pub fn load<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            name: name.into(),
            import: false,
        }
    }

    pub fn import<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            name: name.into(),
            import: true,
        }
    }
}

impl HirLowering for Include {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        self.name.lower(runtime);

        let elem = if self.import {
            LirElement::Import
        } else {
            LirElement::Load
        };
        runtime.emit(elem);
    }
}
