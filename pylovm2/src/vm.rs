use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use lovm2::prelude::*;
use lovm2::vm::LoadRequest;

use crate::context::Context;
use crate::expr::any_to_expr;
use crate::lv2::*;
use crate::module::Module;
use crate::value::Value;
use crate::{err_to_exception, exception_to_err};

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
    pub fn with_std(_this: &PyAny) -> PyResult<Self> {
        Ok(Self {
            inner: lovm2::create_vm_with_std(),
        })
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
    pub fn call(&mut self, name: &PyString, args: &PyTuple) -> PyResult<Value> {
        let name = name.to_str()?.to_string();
        let ruargs: PyResult<Vec<Lovm2ValueRaw>> = args
            .iter()
            .map(|arg| {
                any_to_expr(arg)?
                    .eval(&self.inner.context_mut())
                    .map_err(err_to_exception)
            })
            .collect();

        self.inner
            .call(&name, ruargs?.as_slice())
            .map(Value::from_struct)
            .map_err(err_to_exception)
    }

    pub fn ctx(&mut self) -> PyResult<Context> {
        Ok(Context::new(self.inner.context_mut()))
    }

    pub fn add_module(&mut self, module: &mut Module, namespaced: Option<bool>) -> PyResult<()> {
        let namespaced = namespaced.unwrap_or(true);
        let module = module
            .inner
            .take()
            .expect("given module was already loaded");

        self.inner
            .add_module(module, namespaced)
            .map_err(err_to_exception)
    }

    pub fn add_module_unnamespaced(&mut self, module: &mut Module) -> PyResult<()> {
        self.add_module(module, Some(false))
    }

    pub fn add_main_module(&mut self, module: &mut Module) -> PyResult<()> {
        let module = module
            .inner
            .take()
            .expect("given module was already loaded");

        self.inner.add_main_module(module).map_err(err_to_exception)
    }

    pub fn run(&mut self) -> PyResult<()> {
        self.inner.run().map(|_| ()).map_err(err_to_exception)
    }

    pub fn add_interrupt(&mut self, py: Python, id: u16, func: &PyAny) -> PyResult<()> {
        use pyo3::types::PyTuple;

        if !func.is_callable() {
            return Err(PyRuntimeError::new_err("given function is not callable"));
        }

        let func = func.to_object(py);

        self.inner
            .set_interrupt(id, move |vm| {
                let guard = Python::acquire_gil();
                let py = guard.python();

                let context_ref = vm.context_mut() as *mut Lovm2Context;
                let ctx = Py::new(py, Context::new(context_ref)).unwrap();
                let args = PyTuple::new(py, vec![ctx]);

                if let Err(e) = func.call1(py, args) {
                    return Err(Lovm2Error::from(exception_to_err(&e, py)));
                }

                Ok(())
            })
            .map(|_| ())
            .map_err(err_to_exception)
    }

    pub fn set_load_hook(&mut self, func: PyObject) -> PyResult<()> {
        let hook = move |req: &LoadRequest| {
            let guard = Python::acquire_gil();
            let py = guard.python();
            let args = PyTuple::new(
                py,
                vec![req.module.to_object(py), req.relative_to.to_object(py)],
            );

            let ret = func.call1(py, args).map_err(|e| exception_to_err(&e, py))?;

            if ret.is_none(py) {
                return Ok(None);
            }

            match ret.extract::<Module>(py) {
                Ok(data) => Ok(Some(data.inner.unwrap())),
                Err(e) => Err(exception_to_err(&e, py).into()),
            }
        };

        self.inner.set_load_hook(hook);

        Ok(())
    }

    pub fn set_import_hook(&mut self, func: PyObject) -> PyResult<()> {
        let hook = move |module: Option<&str>, name: &str| {
            let guard = Python::acquire_gil();
            let py = guard.python();
            let args = PyTuple::new(py, vec![module.to_object(py), name.to_object(py)]);
            let result = func.call1(py, args).map_err(|e| exception_to_err(&e, py))?;

            if result.is_none(py) {
                return Ok(None);
            }

            let result = result
                .as_ref(py)
                .str()
                .unwrap()
                .to_string_lossy()
                .to_string();

            Ok(Some(result))
        };

        self.inner.set_import_hook(hook);

        Ok(())
    }
}
