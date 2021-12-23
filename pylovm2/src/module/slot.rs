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
    pub inner: *mut lovm2::prelude::LV2Block,
}

impl LV2Block {
    pub fn from_ptr(inner: *mut lovm2::prelude::LV2Block) -> Self {
        Self {
            inner,
        }
    }

    pub(crate) fn block<'a>(&mut self) -> &'a mut lovm2::prelude::LV2Block {
        unsafe {
            &mut *self.inner
        }
    }
}

//pub type LV2Function = (Vec<lovm2::prelude::LV2Variable>, Py<LV2Block>);
pub struct LV2Function {
    inner: *mut lovm2::prelude::LV2Function,
}

/*
#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LV2Repeat {
    pub ty: lovm2::prelude::LV2RepeatType,
    pub block: LV2Block,
}

impl lovm2::prelude::LV2HirLowering for LV2Repeat {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut lovm2::prelude::LV2HirLoweringRuntime<'lir>) {
        Python::with_gil(|py| {
            let block = self.block.as_ref(py).borrow();
            lovm2::gen::lv2_lower_repeat(runtime, &self.ty, &block.inner);
        })
    }
}
*/

#[derive(Clone)]
pub(super) enum ModuleBuilderSlot {
    // TODO: rename to LV2Function
    Lovm2Hir(lovm2::prelude::LV2Function),
    PyFn(PyObject),
}

impl ModuleBuilderSlot {
    pub fn new() -> Self {
        Self::Lovm2Hir(lovm2::prelude::LV2Function::new())
    }

    pub fn with_args(args: &PyList) -> Self {
        let vars = args
            .iter()
            .map(|arg| arg.str().unwrap().to_string())
            .map(lovm2::prelude::LV2Variable::from)
            .collect();

        Self::Lovm2Hir(lovm2::prelude::LV2Function::with_args(vars))
    }

    pub fn pyfn(pyfn: PyObject) -> Self {
        Self::PyFn(pyfn)
    }

    pub fn code(&mut self) -> PyResult<LV2Block> {
        if let Self::Lovm2Hir(ref mut hir) = self {
            let block = unsafe {
                use lovm2::prelude::LV2AddStatements as _;
                hir.block_mut() as *mut lovm2::prelude::LV2Block
            };
            Ok(LV2Block::from_ptr(block))
        } else {
            Err(err_to_exception(err_custom("slot is not a hir")))
        }
    }
}
