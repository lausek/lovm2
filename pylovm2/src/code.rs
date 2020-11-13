use pyo3::prelude::*;
use pyo3::types::*;

use lovm2::code;
use lovm2::context;
use lovm2::prelude::{Lovm2Error, Lovm2Result};

use crate::expr::any_to_value;
use crate::value::Value;

pub type Lovm2CodeObject = lovm2::code::CodeObject;

pub fn pyerr(e: &PyErr, py: Python) -> Lovm2Error {
    let ty = e.ptype(py).name().to_string();
    let msg = e.pvalue(py).to_string();
    (ty, msg).into()
}

// TODO: change this to hold a Rc<CallProtocol>
#[pyclass(unsendable)]
#[derive(Debug)]
pub struct CodeObject {
    inner: CodeObjectWrapper,
}

impl code::CallProtocol for CodeObject {
    /*
    fn code_object(&self) -> Option<&Lovm2CodeObject> {
        match &self.inner {
            CodeObjectWrapper::Lovm2(co) => Some(&co),
            _ => None,
        }
    }
    */

    fn run(&self, ctx: &mut context::Context) -> Lovm2Result<()> {
        match &self.inner {
            CodeObjectWrapper::Lovm2(co) => co.run(ctx),
            CodeObjectWrapper::Py(pyfn) => {
                let guard = Python::acquire_gil();
                let py = guard.python();

                let frame = ctx.frame_mut()?;
                let mut args = vec![];
                for _ in 0..frame.argn {
                    let val = ctx.pop_value()?;
                    let obj: PyObject = Value::from_struct(val).into_py(py);
                    args.insert(0, obj);
                }
                let args = PyTuple::new(py, args.into_iter());

                // call python function, catch exceptions
                let res = pyfn.call1(py, args).map_err(|e| pyerr(&e, py))?;

                // convert result of call into ruvalue representation
                let res = any_to_value(res.as_ref(py))
                    .map_err(|_| "error in ruvalue conversion".to_string())?;

                ctx.push_value(res.clone());

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
