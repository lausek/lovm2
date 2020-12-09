use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use lovm2::prelude::*;
use lovm2::vm::LoadRequest;

use crate::code::pyerr;
use crate::context::Context;
use crate::expr::any_to_expr;
use crate::lv2::*;
use crate::module::Module;
use crate::value::Value;

#[pyclass(unsendable)]
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

    #[classmethod]
    pub fn with_std(_this: &PyAny) -> Self {
        Self {
            inner: lovm2::vm::Vm::with_std(),
        }
    }

    pub fn add_load_path(&mut self, path: String) -> PyResult<()> {
        if !self.inner.load_paths.contains(&path) {
            self.inner.load_paths.push(path);
        }
        Ok(())
    }

    pub fn clear_load_path(&mut self) {
        self.inner.load_paths.clear();
    }

    pub fn load_path(&self) -> PyResult<Vec<String>> {
        Ok(self.inner.load_paths.clone())
    }

    #[args(args = "*")]
    pub fn call(
        &mut self,
        /* py: Python, */ name: &PyString,
        args: &PyTuple,
    ) -> PyResult<Value> {
        let name = name.to_str()?.to_string();

        let mut ruargs = vec![];
        for arg in args.iter() {
            let arg = any_to_expr(arg)?;
            match self.inner.evaluate_expr(&arg) {
                Ok(val) => ruargs.push(val),
                Err(e) => return Err(create_exception(e)),
            }
        }

        match self.inner.call(&name, ruargs.as_slice()) {
            Ok(val) => Ok(Value::from_struct(val)),
            Err(e) => Err(create_exception(e)),
        }
    }

    pub fn ctx(&mut self) -> PyResult<Context> {
        Ok(Context::new(self.inner.context_mut()))
    }

    pub fn load(&mut self, module: &mut Module) -> PyResult<()> {
        let module = module
            .inner
            .take()
            .expect("given module was already loaded");
        self.inner
            .load_and_import_all(module)
            .map_err(create_exception)
    }

    pub fn run(&mut self) -> PyResult<()> {
        match self.inner.run() {
            Ok(_) => Ok(()),
            Err(e) => Err(create_exception(e)),
        }
    }

    pub fn add_interrupt(&mut self, py: Python, id: u16, func: &PyAny) -> PyResult<()> {
        use pyo3::types::PyTuple;

        if !func.is_callable() {
            return Err(PyRuntimeError::new_err("given function is not callable"));
        }

        let func = func.to_object(py);

        self.inner.set_interrupt(id, move |vm| {
            let guard = Python::acquire_gil();
            let py = guard.python();

            let context_ref = &mut vm.ctx as *mut Lovm2Context;
            let ctx = Py::new(py, Context::new(context_ref)).unwrap();
            let args = PyTuple::new(py, vec![ctx]);

            if let Err(e) = func.call1(py, args) {
                return Err(Lovm2Error::from(pyerr(&e, py)));
            }

            Ok(())
        });

        Ok(())
    }

    pub fn set_load_hook(&mut self, func: PyObject) -> PyResult<()> {
        let hook = move |req: &LoadRequest| {
            let guard = Python::acquire_gil();
            let py = guard.python();
            let relative_to = if let Some(relative_to) = &req.relative_to {
                relative_to.to_object(py)
            } else {
                py.None()
            };
            let args = PyTuple::new(py, vec![req.module.to_object(py), relative_to]);

            let ret = func.call1(py, args).map_err(|e| pyerr(&e, py))?;
            if ret.is_none(py) {
                return Ok(None);
            }

            match ret.extract::<Module>(py) {
                Ok(data) => Ok(Some(data.inner.unwrap())),
                Err(e) => Err(pyerr(&e, py).into()),
            }
        };
        self.inner.set_load_hook(hook);
        Ok(())
    }
}

pub(crate) fn create_exception(e: Lovm2Error) -> PyErr {
    let msg = e.to_string();
    match &e.ty {
        Lovm2ErrorTy::Custom(ty) => match ty.as_ref() {
            "AssertionError" => PyAssertionError::new_err(msg),
            "Exception" => PyException::new_err(msg),
            "FileNotFoundError" => PyFileNotFoundError::new_err(msg),
            "ImportError" => PyImportError::new_err(msg),
            "ZeroDivisionError" => PyZeroDivisionError::new_err(msg),
            _ => PyException::new_err(msg),
        },
        Lovm2ErrorTy::ModuleNotFound => PyImportError::new_err(msg),
        _ => PyException::new_err(msg),
    }
}
