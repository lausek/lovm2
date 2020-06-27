use pyo3::exceptions::*;
use pyo3::prelude::*;

type Lovm2RuValue = lovm2::value::RuValueRef;

#[pyclass]
pub struct RuValue {
    inner: Lovm2RuValue,
}

impl RuValue {
    pub fn from(inner: Lovm2RuValue) -> Self {
        Self {
            inner,
        }
    }
}

#[pymethods]
impl RuValue {
    pub fn __str__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
}
