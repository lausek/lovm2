use pyo3::prelude::*;

type Lovm2RuValue = lovm2::value::RuValueRef;

#[pyclass]
pub struct RuValue {
    inner: Lovm2RuValue,
}

impl RuValue {
    pub fn from(inner: Lovm2RuValue) -> Self {
        Self { inner }
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for RuValue {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.borrow().to_string())
    }
}
