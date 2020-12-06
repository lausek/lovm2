use pyo3::prelude::*;
use pyo3::types::PyList;

use lovm2::gen;

use super::builder::*;
use crate::lv2::*;

#[derive(Clone)]
pub(super) enum ModuleBuilderSlotInner {
    Lovm2Hir(Option<gen::HIR>),
    PyFn(Option<PyObject>),
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct ModuleBuilderSlot {
    pub(super) inner: ModuleBuilderSlotInner,
}

#[pymethods]
impl ModuleBuilderSlot {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: ModuleBuilderSlotInner::Lovm2Hir(Some(gen::HIR::new())),
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

            hir.replace(gen::HIR::with_args(vars));
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

    pub fn pyfn(&mut self, pyfn: PyObject) -> PyResult<()> {
        self.inner = ModuleBuilderSlotInner::PyFn(Some(pyfn));
        Ok(())
    }
}
