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
    pub fn with_std(_this: &PyAny, py: Python) -> PyResult<Self> {
        let mut vm = Self::new();

        let stdlib = py.import("pylovm2_stdlib").map_err(|_| {
            PyImportError::new_err("Failed to create VM with standard library. Did you install with stdlib support?\n\tpip install pylovm2[stdlib]\n")
        })?;
        let stdlib_version = stdlib.get("__version__")?;

        // Check that stdlib has the same lovm2 version as this crate to ensure that `Module`
        // has the same memory layout.
        if stdlib_version.compare(crate::VERSION)? != std::cmp::Ordering::Equal {
            return Err(PyTypeError::new_err(
                format!(
                    "Cannot load standard library: lovm2 version is {}, but stdlib version is {}. Consider upgrading.",
                    crate::VERSION,
                    stdlib_version
                ),
            ));
        }

        let module = stdlib.call_method("create_std_module", (), None)?;

        if "Module" != module.get_type().name() {
            return Err(PyTypeError::new_err(
                "Created standard library is not a module",
            ));
        }

        let module: &PyCell<Module> = unsafe {
            // We cannot use `.extract()` here, because pyo3 relies
            // on the PyTypeObjects to be identical i.e. having the same
            // instance pointer. This will never be the case as
            // pylovm2 and pylovm2_stdlib are compiled in different crates.
            PyTryFrom::try_from_unchecked(module)
        };
        let mut module = module.try_borrow_mut()?;

        vm.add_module_unnamespaced(&mut module)?;

        Ok(vm)
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
            match arg.eval(&self.inner.context_mut()) {
                Ok(val) => ruargs.push(val),
                Err(e) => return Err(err_to_exception(e)),
            }
        }

        match self.inner.call(&name, ruargs.as_slice()) {
            Ok(val) => Ok(Value::from_struct(val)),
            Err(e) => Err(err_to_exception(e)),
        }
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
        match self.inner.run() {
            Ok(_) => Ok(()),
            Err(e) => Err(err_to_exception(e)),
        }
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
            .map_err(err_to_exception)?;

        Ok(())
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
