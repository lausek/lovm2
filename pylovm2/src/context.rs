use pyo3::prelude::*;

#[pyclass]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
}
