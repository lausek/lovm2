use std::cell::RefCell;
use std::rc::Rc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;

use crate::expr::any_to_value;

type Lovm2RuValueRaw = lovm2::value::RuValue;
type Lovm2RuValue = lovm2::value::RuValueRef;

pub fn lovm2py(val: &Lovm2RuValueRaw, py: Python) -> PyObject {
    match val {
        Lovm2RuValueRaw::Bool(b) => (if *b { 1. } else { 0. }).into_py(py),
        Lovm2RuValueRaw::Int(n) => (*n).into_py(py),
        Lovm2RuValueRaw::Float(n) => (*n).into_py(py),
        Lovm2RuValueRaw::Str(s) => s.into_py(py),
        Lovm2RuValueRaw::Dict(dict) => {
            let map = PyDict::new(py);
            for (key, val) in dict.borrow().iter() {
                let (key, val) = (lovm2py(key, py), lovm2py(val, py));
                map.set_item(key, val).unwrap();
            }
            map.to_object(py)
        }
        Lovm2RuValueRaw::List(list) => list
            .borrow()
            .iter()
            .map(|item| lovm2py(item, py))
            .collect::<Vec<PyObject>>()
            .to_object(py),
        Lovm2RuValueRaw::Nil => py.None(),
    }
}

// TODO: implement ToPyObject, FromPyObject for this type
#[pyclass]
pub struct RuValue {
    inner: Lovm2RuValue,
}

impl RuValue {
    pub fn from(inner: Lovm2RuValue) -> Self {
        Self { inner }
    }

    pub fn from_struct(inner: Lovm2RuValueRaw) -> Self {
        Self::from(Rc::new(RefCell::new(inner)))
    }
}

#[pymethods]
impl RuValue {
    pub fn to_py(&self, py: Python) -> PyObject {
        lovm2py(&*self.inner.borrow(), py)
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for RuValue {
    fn __bool__(&self) -> PyResult<bool> {
        let result = match &*self.inner.borrow() {
            Lovm2RuValueRaw::Bool(b) => *b,
            Lovm2RuValueRaw::Int(n) => *n == 0,
            Lovm2RuValueRaw::Float(n) => *n as i64 == 0,
            Lovm2RuValueRaw::Str(s) => !s.is_empty(),
            Lovm2RuValueRaw::Dict(d) => !d.borrow().is_empty(),
            Lovm2RuValueRaw::List(l) => !l.borrow().is_empty(),
            Lovm2RuValueRaw::Nil => false,
        };
        Ok(result)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.borrow().to_string())
    }
}

#[pyproto]
impl pyo3::class::number::PyNumberProtocol for RuValue {
    fn __int__(&self) -> PyResult<PyObject> {
        use lovm2::value::cast::RUVALUE_INT_TY;
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self.inner.borrow().clone().cast(RUVALUE_INT_TY) {
            Ok(val) => Ok(RuValue::from_struct(val).to_py(py)),
            _ => RuntimeError::into("cannot convert value to int".to_string()),
        }
    }

    fn __float__(&self) -> PyResult<PyObject> {
        use lovm2::value::cast::RUVALUE_FLOAT_TY;
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self.inner.borrow().clone().cast(RUVALUE_FLOAT_TY) {
            Ok(val) => Ok(RuValue::from_struct(val).to_py(py)),
            _ => RuntimeError::into("cannot convert value to float".to_string()),
        }
    }
}

#[pyproto]
impl pyo3::class::mapping::PyMappingProtocol for RuValue {
    fn __delitem__(&mut self, key: &PyAny) -> PyResult<()> {
        let key = any_to_value(key)?;
        let key = lovm2::value::instantiate(&key);
        self.inner.borrow_mut().delete(key).unwrap();
        Ok(())
    }

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

    fn __len__(&self) -> PyResult<usize> {
        match self.inner.borrow().len() {
            Ok(n) => Ok(n),
            _ => RuntimeError::into("len not supported on this value".to_string()),
        }
    }

    /*
    fn __setitem__(&mut self, key: &PyAny, val: &PyAny) -> PyResult<()> {
        // self.inner.borrow_mut().set(key, val);
        todo!()
    }
    */
}

/*
#[pyproto]
impl pyo3::class::iter::PyIterProtocol for RuValue {
    fn __iter__(mut slf: PyRefMut<Self>) -> PyResult<PyObject> {
        todo!()
    }
}
*/
