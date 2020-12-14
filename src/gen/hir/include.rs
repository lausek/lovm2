//! loads a module by name into the vm

use super::*;

/// loads a module by name into the vm
#[derive(Clone)]
pub struct Include {
    name: Expr,
    namespaced: bool,
}

impl Include {
    pub fn import<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            name: name.into(),
            namespaced: true,
        }
    }

    pub fn import_global<T>(name: T) -> Self
    where
        T: Into<Expr>,
    {
        Self {
            name: name.into(),
            namespaced: false,
        }
    }
}

impl HirLowering for Include {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        self.name.lower(runtime);

        let elem = LirElement::Import {
            namespaced: self.namespaced,
        };
        runtime.emit(elem);
    }
}
