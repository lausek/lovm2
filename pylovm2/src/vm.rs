use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use crate::context::{Context, Lovm2Context};
use crate::expr::any_to_value;
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

    #[args(args = "*")]
    pub fn call(&mut self, name: &PyString, args: &PyTuple) -> PyResult<RuValue> {
        // TODO: refactor this
        use lovm2::value::instantiate;
        let name = name.to_string()?.to_string();
        let args: Vec<lovm2::value::RuValue> = args.iter()
            .map(|v| instantiate(&any_to_value(v).unwrap()))
            .collect();
        match self.inner.call(&name, args.as_slice()) {
            Ok(val) => {
                use std::cell::RefCell;
                use std::rc::Rc;
                let val = Rc::new(RefCell::new(val));
                Ok(RuValue::from(val))
            }
            Err(msg) => todo!(),
        }
    }

    pub fn ctx(&mut self) -> PyResult<Context> {
        todo!()
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

            if let Err(err) = func.call1(py, args) {
                err.print(py);
                panic!("");
            }
        });

        Ok(())
    }
}
