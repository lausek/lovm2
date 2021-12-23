use pyo3::prelude::*;
use pyo3::types::*;

use lovm2::error::*;

use crate::exception_to_err;
use crate::expr::any_to_value;
use crate::value::LV2Value;

// TODO: change this to hold a Rc<LV2CallProtocol>
#[pyclass(unsendable)]
#[derive(Debug)]
pub struct CodeObject {
    inner: CodeObjectWrapper,
}

impl lovm2::code::LV2CallProtocol for CodeObject {
    fn run(&self, vm: &mut lovm2::vm::LV2Vm) -> lovm2::prelude::LV2Result<()> {
        match &self.inner {
            CodeObjectWrapper::Lovm2(co) => co.run(vm),
            CodeObjectWrapper::Py(pyfn) => {
                let guard = Python::acquire_gil();
                let py = guard.python();

                let frame = vm.context_mut().frame_mut()?;
                let mut args = vec![];

                for _ in 0..frame.argn {
                    let val = vm.context_mut().pop_value()?;
                    let obj: PyObject = LV2Value::from_struct(val).into_py(py);

                    args.insert(0, obj);
                }

                let args = PyTuple::new(py, args.into_iter());

                // call python function, catch exceptions
                let res = pyfn.call1(py, args).map_err(|e| exception_to_err(&e, py))?;

                // convert result of call into ruvalue representation
                let res = any_to_value(res.as_ref(py))
                    .or_else(|_| err_from_string("error in ruvalue conversion"))?;

                vm.context_mut().push_value(res.clone());

                Ok(())
            }
        }
    }
}

impl From<lovm2::code::LV2CodeObject> for CodeObject {
    fn from(inner: lovm2::code::LV2CodeObject) -> Self {
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
    Lovm2(lovm2::code::LV2CodeObject),
    Py(PyObject),
}
