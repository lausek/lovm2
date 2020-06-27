use pyo3::exceptions::*;
use pyo3::prelude::*;

use crate::module::Module;

#[pyclass]
pub struct Vm {
    inner: lovm2::vm::Vm,
}

#[pymethods]
impl Vm {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: lovm2::vm::Vm::new(),
        }
    }

    pub fn load(&mut self, module: &mut Module) -> PyResult<()> {
        let module = module
            .inner
            .take()
            .expect("module given was already loaded");
        // println!("{:#?}", module);
        if let Err(msg) = self.inner.load_and_import_all(module) {
            return TypeError::into(msg);
        }
        Ok(())
    }

    pub fn run(&mut self) -> PyResult<()> {
        match self.inner.run() {
            Ok(_) => Ok(()),
            Err(msg) => TypeError::into(msg),
        }
    }
}