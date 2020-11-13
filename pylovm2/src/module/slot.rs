use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyList;

use lovm2::hir;

use crate::code::CodeObject;

use super::builder::*;
use super::Lovm2Block;

#[derive(Clone)]
enum ModuleBuilderSlotInner {
    Lovm2Hir(Option<hir::HIR>),
    PyFn(Option<PyObject>),
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct ModuleBuilderSlot {
    inner: ModuleBuilderSlotInner,
}

#[pymethods]
impl ModuleBuilderSlot {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: ModuleBuilderSlotInner::Lovm2Hir(Some(hir::HIR::new())),
        }
    }

    pub fn args(&mut self, args: &PyList) -> PyResult<()> {
        if let ModuleBuilderSlotInner::Lovm2Hir(ref mut hir) = self.inner {
            use lovm2::var::Variable;

            let mut vars = vec![];
            for arg in args.iter() {
                let name = arg.str()?.to_string();
                vars.push(Variable::from(name));
            }

            hir.replace(hir::HIR::with_args(vars));
        } else {
            unimplemented!()
        }
        Ok(())
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

    // TODO: can we use consuming self here?
    /*
    pub fn complete(&mut self) -> PyResult<CodeObject> {
        match &mut self.inner {
            ModuleBuilderSlotInner::Lovm2Hir(ref mut hir) => {
                if let Some(hir) = hir.take() {
                    return match hir.build() {
                        Ok(co) => Ok(CodeObject::from(co)),
                        Err(msg) => Err(PyRuntimeError::new_err(msg.to_string())),
                    };
                }
                Err(PyRuntimeError::new_err("hir was already built"))
            }
            ModuleBuilderSlotInner::PyFn(ref mut pyfn) => {
                Ok(CodeObject::from(pyfn.take().unwrap()))
            }
        }
    }
    */

    pub fn pyfn(&mut self, pyfn: PyObject) -> PyResult<()> {
        self.inner = ModuleBuilderSlotInner::PyFn(Some(pyfn));
        Ok(())
    }
}
