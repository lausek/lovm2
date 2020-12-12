use pyo3::prelude::*;
use pyo3::types::PyList;

use lovm2::gen;

use super::builder::*;
use crate::lv2::*;

#[derive(Clone)]
pub(super) enum ModuleBuilderSlotInner {
    Lovm2Hir(Option<gen::Hir>),
    PyFn(Option<PyObject>),
}

#[derive(Clone)]
pub(super) struct ModuleBuilderSlot {
    pub(super) inner: ModuleBuilderSlotInner,
}

impl ModuleBuilderSlot {
    pub fn new() -> Self {
        Self {
            inner: ModuleBuilderSlotInner::Lovm2Hir(Some(gen::Hir::new())),
        }
    }

    pub fn with_args(args: &PyList) -> Self {
        use lovm2::var::Variable;

        let mut vars = vec![];
        for arg in args.iter() {
            let name = arg.str().unwrap().to_string();
            vars.push(Variable::from(name));
        }

        Self {
            inner: ModuleBuilderSlotInner::Lovm2Hir(Some(gen::Hir::with_args(vars))),
        }
    }

    pub fn code(&mut self) -> PyResult<BlockBuilder> {
        if let ModuleBuilderSlotInner::Lovm2Hir(ref mut hir) = self.inner {
            let hir = hir.as_mut().unwrap();
            let inner = &mut hir.code as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        } else {
            unimplemented!()
        }
    }

    pub fn pyfn(pyfn: PyObject) -> Self {
        Self {
            inner: ModuleBuilderSlotInner::PyFn(Some(pyfn)),
        }
    }
}
