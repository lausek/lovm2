use pyo3::exceptions::*;
use pyo3::prelude::*;

use lovm2::vm;

#[pyclass]
pub struct Vm {
    inner: vm::Vm,
}

#[pymethods]
impl Vm {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: vm::Vm::new(),
        }
    }

    pub fn run(&mut self) -> PyResult<()> {
        match self.inner.run() {
            Ok(_) => Ok(()),
            Err(msg) => TypeError::into(msg),
        }
    }
}
