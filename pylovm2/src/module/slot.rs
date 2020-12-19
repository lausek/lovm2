use pyo3::prelude::*;
use pyo3::types::PyList;

use lovm2::gen;
use lovm2::Variable;

use crate::err_to_exception;
use crate::lv2::*;

use super::builder::*;

#[derive(Clone)]
pub(super) enum ModuleBuilderSlot {
    Lovm2Hir(gen::Hir),
    PyFn(PyObject),
}

impl ModuleBuilderSlot {
    pub fn new() -> Self {
        Self::Lovm2Hir(gen::Hir::new())
    }

    pub fn with_args(args: &PyList) -> Self {
        let mut vars = vec![];
        for arg in args.iter() {
            let name = arg.str().unwrap().to_string();
            vars.push(Variable::from(name));
        }

        Self::Lovm2Hir(gen::Hir::with_args(vars))
    }

    pub fn pyfn(pyfn: PyObject) -> Self {
        Self::PyFn(pyfn)
    }

    pub fn code(&mut self) -> PyResult<BlockBuilder> {
        if let Self::Lovm2Hir(ref mut hir) = self {
            let inner = &mut hir.code as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        } else {
            Err(err_to_exception(format!("slot is not a hir").into()))
        }
    }
}
