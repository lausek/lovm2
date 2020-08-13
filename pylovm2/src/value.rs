use pyo3::exceptions::*;
use pyo3::prelude::*;

use crate::expr::any_to_value;

type Lovm2RuValueRaw = lovm2::value::RuValue;
type Lovm2RuValue = lovm2::value::RuValueRef;

#[pyclass]
pub struct RuValue {
    inner: Lovm2RuValue,
}

impl RuValue {
    pub fn from(inner: Lovm2RuValue) -> Self {
        Self { inner }
    }

    pub fn to_py(&self, py: Python) -> PyObject {
        match &*self.inner.borrow() {
            Lovm2RuValueRaw::Bool(b) => (if *b { 1. } else { 0. }).into_py(py),
            Lovm2RuValueRaw::Int(n) => (*n as f64).into_py(py),
            Lovm2RuValueRaw::Float(n) => (*n).into_py(py),
            _ => unimplemented!(),
        }
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for RuValue {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.borrow().to_string())
    }
}

// TODO: rely on lovm2 type conversion here
#[pyproto]
impl pyo3::class::number::PyNumberProtocol for RuValue {
    fn __int__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let obj: PyObject = self.to_py(py);
        Ok(obj)
    }

    fn __float__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let obj: PyObject = self.to_py(py);
        Ok(obj)
    }
}

#[pyproto]
impl pyo3::class::mapping::PyMappingProtocol for RuValue {
    fn __getitem__(&self, key: &PyAny) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let key = any_to_value(key)?;
        let key = lovm2::value::instantiate(&key);
        // TODO: avoid clone here
        match self.inner.borrow().get(key.clone()) {
            Ok(val) => {
                let val = lovm2::value::box_ruvalue(val);
                Ok(RuValue::from(val).to_py(py))
            }
            Err(_) => RuntimeError::into(format!("key {} not found on value", key)),
        }
    }
}
