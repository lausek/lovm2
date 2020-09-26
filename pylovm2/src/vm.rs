use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use lovm2::prelude::*;

use crate::code::pyerr;
use crate::context::{Context, Lovm2Context};
use crate::expr::any_to_expr;
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
            let arg = any_to_expr(arg)?;
            match self.inner.evaluate_expr(&arg) {
                Ok(val) => ruargs.push(val),
                Err(e) => return create_exception(e).into(),
            }
        }

        match self.inner.call(&name, ruargs.as_slice()) {
            Ok(val) => Ok(lovm2py(&val, py)),
            Err(e) => create_exception(e).into(),
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
        self.inner
            .load_and_import_all(module)
            .map_err(create_exception)
    }

    pub fn run(&mut self) -> PyResult<()> {
        match self.inner.run() {
            Ok(_) => Ok(()),
            Err(e) => create_exception(e).into(),
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

            // TODO: interrupts can raise errors too
            if let Err(err) = func.call1(py, args) {
                err.print(py);
                panic!("");
            }
        });

        Ok(())
    }

    pub fn set_load_hook(&mut self, func: PyObject) -> PyResult<()> {
        let hook = move |name: String| {
            let guard = Python::acquire_gil();
            let py = guard.python();
            let args = PyTuple::new(py, vec![name.to_object(py)]);

            let ret = func.call1(py, args).map_err(|e| pyerr(&e, py))?;
            if ret.is_none() {
                return Ok(None);
            }

            match ret.extract::<Module>(py) {
                Ok(data) => Ok(Some(data.inner.unwrap())),
                Err(e) => Err(pyerr(&e, py).into()),
            }
        };
        self.inner.context_mut().set_load_hook(hook);
        Ok(())
    }
}

fn create_exception(e: Lovm2Error) -> PyErr {
    match e {
        Lovm2Error::Msg(Some(ty), msg) => match ty.as_ref() {
            "AssertionError" => AssertionError::py_err(msg),
            "Exception" => Exception::py_err(msg),
            "ImportError" => ImportError::py_err(msg),
            "ZeroDivisionError" => ZeroDivisionError::py_err(msg),
            _ => Exception::py_err(msg),
        },
        Lovm2Error::Msg(_, msg) => Exception::py_err(msg),
    }
}
