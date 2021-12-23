use pyo3::prelude::*;

use crate::expr::any_to_ident;
use crate::value::lovm2py;

#[pyclass(unsendable)]
pub struct LV2Context {
    inner: *mut lovm2::vm::LV2Context,
}

impl LV2Context {
    pub fn new(inner: *mut lovm2::vm::LV2Context) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl LV2Context {
    pub fn frame(&mut self, py: Python) -> PyResult<PyObject> {
        unsafe {
            match (*self.inner).frame_mut() {
                Ok(frame) => {
                    let frame_ref = frame as *mut lovm2::vm::LV2StackFrame;
                    let obj = Py::new(py, LV2Frame::new(frame_ref))?.to_object(py);

                    Ok(obj)
                }
                Err(_) => Ok(py.None()),
            }
        }
    }

    pub fn globals(&mut self, py: Python, name: &PyAny) -> PyResult<Option<PyObject>> {
        let name = any_to_ident(name)?;

        unsafe {
            if let Some(val) = (*self.inner).value_of(name).ok() {
                return Ok(Some(lovm2py(&val, py)));
            }
        }

        Ok(None)
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for LV2Context {
    fn __str__(&self) -> String {
        format!("{:?}", unsafe { &*self.inner })
    }
}

#[pyclass(unsendable)]
pub struct LV2Frame {
    inner: *mut lovm2::vm::LV2StackFrame,
}

impl LV2Frame {
    pub fn new(inner: *mut lovm2::vm::LV2StackFrame) -> Self {
        Self { inner }
    }
}

// TODO: implement indexing for this type
#[pymethods]
impl LV2Frame {
    pub fn local(&self, py: Python, name: &PyAny) -> PyResult<Option<PyObject>> {
        let name = any_to_ident(name)?;

        unsafe {
            let result = (*self.inner)
                .value_of(name)
                .ok()
                .map(|val| lovm2py(&val, py));
            Ok(result)
        }
    }
}
