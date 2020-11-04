use pyo3::prelude::*;

use crate::value::lovm2py;

pub type Lovm2Context = lovm2::context::Context;
pub type Lovm2Frame = lovm2::frame::Frame;

#[pyclass(unsendable)]
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
    pub fn add_load_path(&mut self, path: String) -> PyResult<()> {
        unsafe {
            if !(*self.inner).load_paths.contains(&path) {
                (*self.inner).load_paths.push(path);
            }
        }
        Ok(())
    }

    pub fn clear_load_path(&mut self) {
        unsafe {
            (*self.inner).load_paths.clear();
        }
    }

    pub fn load_path(&self) -> PyResult<Vec<String>> {
        unsafe { Ok((*self.inner).load_paths.clone()) }
    }

    pub fn frame(&mut self, py: Python) -> PyResult<PyObject> {
        unsafe {
            match (*self.inner).frame_mut() {
                Ok(frame) => {
                    let frame_ref = frame as *mut Lovm2Frame;
                    let obj = Py::new(py, Frame::new(frame_ref))?.to_object(py);
                    Ok(obj)
                }
                Err(_) => Ok(py.None()),
            }
        }
    }

    pub fn globals(&mut self, py: Python, name: String) -> Option<PyObject> {
        unsafe {
            if let Some(val) = (*self.inner)
                .globals
                .get(&lovm2::var::Variable::from(name))
                .cloned()
            {
                return Some(lovm2py(&val, py));
            }
        }
        None
    }
}

#[pyclass(unsendable)]
pub struct Frame {
    inner: *mut Lovm2Frame,
}

impl Frame {
    pub fn new(inner: *mut Lovm2Frame) -> Self {
        Self { inner }
    }
}

// TODO: implement indexing for this type
#[pymethods]
impl Frame {
    pub fn local(&self, py: Python, key: String) -> Option<PyObject> {
        unsafe {
            (*self.inner)
                .locals
                .get(&lovm2::var::Variable::from(key))
                .map(|val| lovm2py(&val, py))
        }
    }
}
