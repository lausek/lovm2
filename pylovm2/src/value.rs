use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;

use crate::err_to_exception;
use crate::expr::any_to_pylovm2_value;

pub fn lovm2py(val: &lovm2::value::LV2Value, py: Python) -> PyObject {
    match val {
        lovm2::value::LV2Value::Bool(b) => (if *b { 1. } else { 0. }).into_py(py),
        lovm2::value::LV2Value::Int(n) => (*n).into_py(py),
        lovm2::value::LV2Value::Float(n) => (*n).into_py(py),
        lovm2::value::LV2Value::Str(s) => s.into_py(py),
        lovm2::value::LV2Value::Dict(dict) => {
            let map = PyDict::new(py);

            for (key, val) in dict.iter() {
                let (key, val) = (lovm2py(key, py), lovm2py(val, py));

                map.set_item(key, val).unwrap();
            }

            map.to_object(py)
        }
        lovm2::value::LV2Value::List(list) => list
            .iter()
            .map(|item| lovm2py(item, py))
            .collect::<Vec<PyObject>>()
            .to_object(py),
        lovm2::value::LV2Value::Nil => py.None(),
        lovm2::value::LV2Value::Ref(r) => {
            if !r.is_empty() {
                lovm2py(&r.borrow().unwrap(), py)
            } else {
                py.None()
            }
        }
        lovm2::value::LV2Value::Iter(it) => {
            let vals = it.borrow().clone().collect();
            let ls = lovm2::value::box_value(lovm2::value::LV2Value::List(vals));

            lovm2py(&ls, py)
        }
        lovm2::value::LV2Value::Any(_) => todo!(),
    }
}

// TODO: implement ToPyObject, FromPyObject for this type
#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LV2Value {
    pub(crate) inner: lovm2::value::LV2ValueRef,
}

impl LV2Value {
    pub fn from(inner: lovm2::value::LV2ValueRef) -> Self {
        Self { inner }
    }

    pub fn from_struct(inner: lovm2::value::LV2Value) -> Self {
        Self::from(lovm2::value::LV2ValueRef::from(inner))
    }
}

#[pymethods]
impl LV2Value {
    pub fn to_py(&self, py: Python) -> PyObject {
        lovm2py(&*self.inner.borrow().unwrap(), py)
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for LV2Value {
    fn __bool__(&self) -> PyResult<bool> {
        let result = match &*self.inner.borrow().unwrap() {
            lovm2::value::LV2Value::Bool(b) => *b,
            lovm2::value::LV2Value::Int(n) => *n == 0,
            lovm2::value::LV2Value::Float(n) => *n as i64 == 0,
            lovm2::value::LV2Value::Str(s) => !s.is_empty(),
            lovm2::value::LV2Value::Dict(d) => !d.is_empty(),
            lovm2::value::LV2Value::List(l) => !l.is_empty(),
            lovm2::value::LV2Value::Nil => false,
            // TODO: is a reference true?
            lovm2::value::LV2Value::Ref(_) => false,
            _ => todo!(),
        };

        Ok(result)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.borrow().unwrap().to_string())
    }
}

#[pyproto]
impl pyo3::class::number::PyNumberProtocol for LV2Value {
    fn __int__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        match self
            .inner
            .borrow()
            .unwrap()
            .clone()
            .conv(lovm2::value::LV2ValueType::Int)
        {
            Ok(val) => Ok(LV2Value::from_struct(val).to_py(py)),
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
            .conv(lovm2::value::LV2ValueType::Float)
        {
            Ok(val) => Ok(LV2Value::from_struct(val).to_py(py)),
            _ => Err(PyRuntimeError::new_err(
                "cannot convert value to float".to_string(),
            )),
        }
    }
}

#[pyproto]
impl pyo3::class::mapping::PyMappingProtocol for LV2Value {
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

                Ok(LV2Value::from_struct(val).to_py(py))
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
