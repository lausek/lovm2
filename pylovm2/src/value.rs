use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;

use crate::err_to_exception;
use crate::expr::any_to_pylovm2_value;
use crate::lv2::*;

pub fn lovm2py(val: &Lovm2ValueRaw, py: Python) -> PyObject {
    match val {
        Lovm2ValueRaw::Bool(b) => (if *b { 1. } else { 0. }).into_py(py),
        Lovm2ValueRaw::Int(n) => (*n).into_py(py),
        Lovm2ValueRaw::Float(n) => (*n).into_py(py),
        Lovm2ValueRaw::Str(s) => s.into_py(py),
        Lovm2ValueRaw::Dict(dict) => {
            let map = PyDict::new(py);
            for (key, val) in dict.iter() {
                let (key, val) = (lovm2py(key, py), lovm2py(val, py));
                map.set_item(key, val).unwrap();
            }
            map.to_object(py)
        }
        Lovm2ValueRaw::List(list) => list
            .iter()
            .map(|item| lovm2py(item, py))
            .collect::<Vec<PyObject>>()
            .to_object(py),
        Lovm2ValueRaw::Nil => py.None(),
        Lovm2ValueRaw::Ref(r) => {
            if !r.is_empty() {
                lovm2py(&r.borrow().unwrap(), py)
            } else {
                py.None()
            }
        }
        Lovm2ValueRaw::Any(any) => {
            if let Some(it) = any.borrow().0.downcast_ref::<lovm2::value::Iter>() {
                let ls = lovm2::value::box_value(Lovm2ValueRaw::List(it.clone().collect()));
                lovm2py(&ls, py)
            } else {
                todo!()
            }
        }
    }
}

// TODO: implement ToPyObject, FromPyObject for this type
#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Value {
    inner: Lovm2Value,
}

impl Value {
    pub fn from(inner: Lovm2Value) -> Self {
        Self { inner }
    }

    pub fn from_struct(inner: Lovm2ValueRaw) -> Self {
        Self::from(Lovm2Ref::from(inner))
    }
}

#[pymethods]
impl Value {
    pub fn to_py(&self, py: Python) -> PyObject {
        lovm2py(&*self.inner.borrow().unwrap(), py)
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Value {
    fn __bool__(&self) -> PyResult<bool> {
        let result = match &*self.inner.borrow().unwrap() {
            Lovm2ValueRaw::Bool(b) => *b,
            Lovm2ValueRaw::Int(n) => *n == 0,
            Lovm2ValueRaw::Float(n) => *n as i64 == 0,
            Lovm2ValueRaw::Str(s) => !s.is_empty(),
            Lovm2ValueRaw::Dict(d) => !d.is_empty(),
            Lovm2ValueRaw::List(l) => !l.is_empty(),
            Lovm2ValueRaw::Nil => false,
            // TODO: is a reference true?
            Lovm2ValueRaw::Ref(_) => false,
            _ => todo!(),
        };
        Ok(result)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.borrow().unwrap().to_string())
    }
}

#[pyproto]
impl pyo3::class::number::PyNumberProtocol for Value {
    fn __int__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self
            .inner
            .borrow()
            .unwrap()
            .clone()
            .conv(Lovm2ValueType::Int)
        {
            Ok(val) => Ok(Value::from_struct(val).to_py(py)),
            _ => Err(PyRuntimeError::new_err(
                "cannot convert value to int".to_string(),
            )),
        }
    }

    fn __float__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self
            .inner
            .borrow()
            .unwrap()
            .clone()
            .conv(Lovm2ValueType::Float)
        {
            Ok(val) => Ok(Value::from_struct(val).to_py(py)),
            _ => Err(PyRuntimeError::new_err(
                "cannot convert value to float".to_string(),
            )),
        }
    }
}

#[pyproto]
impl pyo3::class::mapping::PyMappingProtocol for Value {
    fn __delitem__(&mut self, key: &PyAny) -> PyResult<()> {
        let key = any_to_pylovm2_value(key)?;
        let key = key.inner.borrow().unwrap();
        self.inner.borrow_mut().unwrap().delete(&key).unwrap();
        Ok(())
    }

    fn __getitem__(&self, key: &PyAny) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let key = any_to_pylovm2_value(key)?;
        let key = key.inner.borrow().unwrap();
        // TODO: avoid clone here
        match self.inner.borrow().unwrap().get(&key) {
            Ok(val) => {
                let val = lovm2::value::box_value(val);
                Ok(Value::from_struct(val).to_py(py))
            }
            Err(_) => Err(PyRuntimeError::new_err(format!(
                "key {} not found on value",
                key
            ))),
        }
    }

    fn __len__(&self) -> PyResult<usize> {
        match self.inner.borrow().unwrap().len() {
            Ok(n) => Ok(n),
            _ => Err(PyRuntimeError::new_err(
                "len not supported on this value".to_string(),
            )),
        }
    }

    fn __setitem__(&mut self, key: &PyAny, val: &PyAny) -> PyResult<()> {
        let (key, val) = (any_to_pylovm2_value(key)?, any_to_pylovm2_value(val)?);
        let (key, val) = (
            key.inner.borrow().unwrap(),
            val.inner.borrow().unwrap().clone(),
        );
        self.inner
            .borrow_mut()
            .unwrap()
            .set(&key, val)
            .map_err(err_to_exception)
    }
}

/*
#[pyproto]
impl pyo3::class::iter::PyIterProtocol for Value {
    fn __iter__(mut slf: PyRefMut<Self>) -> PyResult<PyObject> {
        todo!()
    }
}
*/
