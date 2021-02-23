//! Loads a module by name into the VM

use super::*;

/// Loads a module by name into the VM
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
    fn lower<'hir, 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    where
        'hir: 'lir,
    {
        self.name.lower(runtime);

        let elem = LirElement::Import {
            namespaced: self.namespaced,
        };

        runtime.emit(elem);
    }
}
