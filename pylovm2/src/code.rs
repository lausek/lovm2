use pyo3::prelude::*;

use lovm2::code;
use lovm2::context;

#[pyclass]
#[derive(Debug)]
pub struct CodeObject {
    inner: code::CodeObject,
}

impl code::CallProtocol for CodeObject {
    fn run(&self, ctx: &mut context::Context) -> Result<(), String> {
        self.inner.run(ctx)
    }
}

impl CodeObject {
    pub fn from(inner: code::CodeObject) -> Self {
        Self { inner }
    }
}
