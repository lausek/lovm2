use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use lovm2::value::instantiate;

use crate::context::{Context, Lovm2Context};
use crate::expr::any_to_value;
use crate::module::Module;
use crate::value::lovm2py;

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

    #[args(args = "*")]
    pub fn call(&mut self, py: Python, name: &PyString, args: &PyTuple) -> PyResult<PyObject> {
        let name = name.to_string()?.to_string();

        let mut ruargs = vec![];
        for arg in args.iter() {
            ruargs.push(instantiate(&any_to_value(arg)?));
        }

        match self.inner.call(&name, ruargs.as_slice()) {
            Ok(val) => Ok(lovm2py(&val, py)),
            Err(msg) => RuntimeError::into(msg),
        }
    }

    pub fn ctx(&mut self) -> PyResult<Context> {
        Ok(Context::new(self.inner.context_mut()))
    }

    pub fn load(&mut self, module: &mut Module) -> PyResult<()> {
        let module = module
            .inner
            .take()
            .expect("module given was already loaded");
        if let Err(msg) = self.inner.load_and_import_all(module) {
            return RuntimeError::into(msg);
        }
        Ok(())
    }

    pub fn run(&mut self) -> PyResult<()> {
        match self.inner.run() {
            Ok(_) => Ok(()),
            Err(msg) => RuntimeError::into(msg),
        }
    }

    pub fn add_interrupt(&mut self, py: Python, id: u16, func: &PyAny) -> PyResult<()> {
        use pyo3::types::PyTuple;

        if !func.is_callable() {
            return RuntimeError::into("given function is not callable");
        }

        let func = func.to_object(py);

        self.inner.context_mut().set_interrupt(id, move |ctx| {
            let guard = Python::acquire_gil();
            let py = guard.python();

            let context_ref = ctx as *mut Lovm2Context;
            let ctx = Py::new(py, Context::new(context_ref)).unwrap();
            let args = PyTuple::new(py, vec![ctx]);

            if let Err(err) = func.call1(py, args) {
                err.print(py);
                panic!("");
            }
        });

        Ok(())
    }
}
