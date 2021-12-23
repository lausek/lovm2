use pyo3::prelude::*;
use pyo3::types::PyList;

use lovm2::error::err_custom;
//use lovm2::gen::{HasBlock, Hir};
//use lovm2::Variable;

use crate::err_to_exception;

use super::builder::*;

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LV2Block {
    pub inner: lovm2::prelude::LV2Block,
}

impl LV2Block {
    pub fn new() -> Self {
        Self {
            inner: lovm2::prelude::LV2Block::new(),
        }
    }
}

pub type LV2Function = (Vec<lovm2::prelude::LV2Variable>, Py<LV2Block>);

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LV2Repeat {
    pub ty: lovm2::prelude::LV2RepeatType,
    pub block: Py<LV2Block>,
}

impl lovm2::prelude::LV2HirLowering for LV2Repeat {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut lovm2::prelude::LV2HirLoweringRuntime<'lir>) {
        Python::with_gil(|py| {
            let block = self.block.as_ref(py).borrow();
            lovm2::gen::lv2_lower_repeat(runtime, &self.ty, &block.inner);
        })
    }
}

#[derive(Clone)]
pub(super) enum ModuleBuilderSlot {
    // TODO: rename to LV2Function
    Lovm2Hir(LV2Function),
    PyFn(PyObject),
}

impl ModuleBuilderSlot {
    pub fn new() -> Self {
        Python::with_gil(|py| {
            let block = Py::new(py, LV2Block::new()).unwrap();
            Self::Lovm2Hir((vec![], block))
        })
    }

    pub fn with_args(args: &PyList) -> Self {
        Python::with_gil(|py| {
            let block = Py::new(py, LV2Block::new()).unwrap();
            let args = args
                .iter()
                .map(|arg| arg.str().unwrap().to_string())
                .map(lovm2::prelude::LV2Variable::from)
                .collect();
            Self::Lovm2Hir((args, block))
        })
    }

    pub fn pyfn(pyfn: PyObject) -> Self {
        Self::PyFn(pyfn)
    }

    pub fn code(&mut self) -> PyResult<Py<LV2Block>> {
        if let Self::Lovm2Hir(ref mut hir) = self {
            Ok(hir.1.clone())
        } else {
            Err(err_to_exception(err_custom("slot is not a hir")))
        }
    }
}
