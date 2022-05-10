use pyo3::prelude::*;
use pyo3::types::PyList;

use lovm2::error::err_custom;

use crate::err_to_exception;
use crate::expr::any_to_ident;

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LV2Block {
    pub inner: *mut lovm2::prelude::LV2Block,
}

impl LV2Block {
    pub fn from_ptr(inner: *mut lovm2::prelude::LV2Block) -> Self {
        Self { inner }
    }

    pub(crate) fn block<'a>(&mut self) -> &'a mut lovm2::prelude::LV2Block {
        unsafe { &mut *self.inner }
    }
}

#[derive(Clone)]
pub(super) enum ModuleBuilderSlot {
    LV2Function(lovm2::prelude::LV2Function),
    PyFn(PyObject),
}

impl ModuleBuilderSlot {
    pub fn new() -> Self {
        Self::LV2Function(lovm2::prelude::LV2Function::new())
    }

    pub fn with_args(args: &PyList) -> PyResult<Self> {
        let vars = args
            .iter()
            .map(|arg| any_to_ident(arg))
            .collect::<PyResult<_>>()?;

        Ok(Self::LV2Function(lovm2::prelude::LV2Function::with_args(
            vars,
        )))
    }

    pub fn pyfn(pyfn: PyObject) -> Self {
        Self::PyFn(pyfn)
    }

    pub fn code(&mut self) -> PyResult<LV2Block> {
        if let Self::LV2Function(ref mut f) = self {
            use lovm2::prelude::LV2AddStatements as _;
            let block = f.block_mut() as *mut lovm2::prelude::LV2Block;
            Ok(LV2Block::from_ptr(block))
        } else {
            Err(err_to_exception(err_custom("slot is not a hir")))
        }
    }
}
