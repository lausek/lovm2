use pyo3::prelude::*;
use pyo3::types::*;

use lovm2::code;
use lovm2::prelude::Lovm2Result;
use lovm2::vm;

use crate::exception_to_err;
use crate::expr::any_to_value;
use crate::value::Value;

pub type Lovm2CodeObject = lovm2::code::CodeObject;

// TODO: change this to hold a Rc<CallProtocol>
#[pyclass(unsendable)]
#[derive(Debug)]
pub struct CodeObject {
    inner: CodeObjectWrapper,
}

impl code::CallProtocol for CodeObject {
    fn run(&self, vm: &mut vm::Vm) -> Lovm2Result<()> {
        match &self.inner {
            CodeObjectWrapper::Lovm2(co) => co.run(vm),
            CodeObjectWrapper::Py(pyfn) => {
                let guard = Python::acquire_gil();
                let py = guard.python();

                let frame = vm.context_mut().frame_mut()?;
                let mut args = vec![];
                for _ in 0..frame.argn {
                    let val = vm.context_mut().pop_value()?;
                    let obj: PyObject = Value::from_struct(val).into_py(py);
                    args.insert(0, obj);
                }
                let args = PyTuple::new(py, args.into_iter());

                // call python function, catch exceptions
                let res = pyfn.call1(py, args).map_err(|e| exception_to_err(&e, py))?;

                // convert result of call into ruvalue representation
                let res = any_to_value(res.as_ref(py))
                    .map_err(|_| "error in ruvalue conversion".to_string())?;

                vm.context_mut().push_value(res.clone());

                Ok(())
            }
        }
    }
}

impl From<Lovm2CodeObject> for CodeObject {
    fn from(inner: Lovm2CodeObject) -> Self {
        Self {
            inner: CodeObjectWrapper::Lovm2(inner),
        }
    }
}

impl From<PyObject> for CodeObject {
    fn from(inner: PyObject) -> Self {
        Self {
            inner: CodeObjectWrapper::Py(inner),
        }
    }
}

// needed because pyo3 cannot use pyclass on enum
#[derive(Debug)]
enum CodeObjectWrapper {
    Lovm2(Lovm2CodeObject),
    Py(PyObject),
}
