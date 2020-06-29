use pyo3::prelude::*;

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
        let obj: PyObject = match &*self.inner.borrow() {
            Lovm2RuValueRaw::Bool(b) => (if *b {1} else {0}).into_py(py),
            Lovm2RuValueRaw::Int(n) => (*n).into_py(py),
            Lovm2RuValueRaw::Float(n) => (*n as i64).into_py(py),
            _ => unimplemented!(),
        };
        Ok(obj)
    }

    fn __float__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let obj: PyObject = match &*self.inner.borrow() {
            Lovm2RuValueRaw::Bool(b) => (if *b {1.} else {0.}).into_py(py),
            Lovm2RuValueRaw::Int(n) => (*n as f64).into_py(py),
            Lovm2RuValueRaw::Float(n) => (*n).into_py(py),
            _ => unimplemented!(),
        };
        Ok(obj)
    }
}
