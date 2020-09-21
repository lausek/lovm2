use pyo3::prelude::*;

use lovm2::code;
use lovm2::context;

pub type Lovm2CodeObject = lovm2::code::CodeObject;

// TODO: change this to hold a Rc<CallProtocol>
#[pyclass]
#[derive(Debug)]
pub struct CodeObject {
    inner: Lovm2CodeObject,
}

impl code::CallProtocol for CodeObject {
    fn code_object(&self) -> Option<&Lovm2CodeObject> {
        Some(&self.inner)
    }

    fn run(&self, ctx: &mut context::Context) -> Result<(), String> {
        self.inner.run(ctx)
    }
}

impl CodeObject {
    pub fn from(inner: Lovm2CodeObject) -> Self {
        Self { inner }
    }
}
