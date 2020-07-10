use pyo3::exceptions::*;
use pyo3::prelude::*;

use crate::context::{Context, Lovm2Context};
use crate::module::Module;
use crate::value::RuValue;

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

    pub fn globals(&mut self, name: String) -> Option<RuValue> {
        if let Some(val) = self.inner.context_mut().globals.get(&name).cloned() {
            return Some(RuValue::from(val));
        }
        None
    }

    pub fn add_interrupt(&mut self, py: Python, id: u16, func: &PyAny) -> PyResult<()> {
        use pyo3::types::PyTuple;

        if !func.is_callable() {
            return TypeError::into("given function is not callable");
        }

        let func = func.to_object(py);

        self.inner.context_mut().set_interrupt(id, move |ctx| {
            let guard = Python::acquire_gil();
            let py = guard.python();

            let context_ref = ctx as *mut Lovm2Context;
            let ctx = Py::new(py, Context::new(context_ref)).unwrap();
            let args = PyTuple::new(py, vec![ctx]);

            func.call1(py, args).unwrap();
        });

        Ok(())
    }
}
