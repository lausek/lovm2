use pyo3::prelude::*;

use crate::value::RuValue;

pub type Lovm2Context = lovm2::context::Context;
pub type Lovm2Frame = lovm2::frame::Frame;

#[pyclass]
pub struct Context {
    inner: *mut Lovm2Context,
}

impl Context {
    pub fn new(inner: *mut Lovm2Context) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Context {
    pub fn frame(&mut self, py: Python) -> PyResult<PyObject> {
        unsafe {
            match (*self.inner).frame_mut() {
                Some(frame) => {
                    let frame_ref = frame as *mut Lovm2Frame;
                    let obj = Py::new(py, Frame::new(frame_ref)).unwrap().to_object(py);
                    Ok(obj)
                }
                None => Ok(py.None()),
            }
        }
    }
}

#[pyclass]
pub struct Frame {
    inner: *mut Lovm2Frame,
}

impl Frame {
    pub fn new(inner: *mut Lovm2Frame) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Frame {
    pub fn local(&self, key: String) -> Option<RuValue> {
        unsafe {
            (*self.inner)
                .locals
                .get(&lovm2::var::Variable::from(key))
                .map(|val| RuValue::from(val.clone()))
        }
    }
}
