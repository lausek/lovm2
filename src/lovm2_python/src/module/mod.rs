mod builder;
mod slot;

use pyo3::exceptions::*;
use pyo3::prelude::*;

pub use self::builder::LV2ModuleBuilder;
use self::slot::ModuleBuilderSlot;

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LV2Module {
    pub inner: Option<lovm2::prelude::LV2Module>,
}

impl From<lovm2::prelude::LV2Module> for LV2Module {
    fn from(inner: lovm2::prelude::LV2Module) -> Self {
        Self {
            inner: Some(inner.into()),
        }
    }
}

#[pymethods]
impl LV2Module {
    #[classmethod]
    pub fn load(_this: &PyAny, path: &PyAny) -> PyResult<Self> {
        let path = path.str()?.to_str()?;

        match lovm2::prelude::LV2Module::load_from_file(path) {
            Ok(inner) => Ok(Self { inner: Some(inner) }),
            Err(err) => Err(PyRuntimeError::new_err(err.to_string())),
        }
    }

    pub fn save(&self, path: String) -> PyResult<()> {
        if let Some(inner) = self.inner.as_ref() {
            return match inner.store_to_file(&path) {
                Ok(_) => Ok(()),
                Err(err) => Err(PyRuntimeError::new_err(err.to_string())),
            };
        }

        Err(PyRuntimeError::new_err("inner module not loaded"))
    }

    pub fn name(&self) -> PyResult<String> {
        if let Some(inner) = self.inner.as_ref() {
            return Ok(inner.name().to_string());
        }

        Err(PyRuntimeError::new_err("inner module not loaded"))
    }

    pub fn uses(&self) -> PyResult<Vec<String>> {
        // TODO: return used dependencies
        Ok(vec![])
    }

    pub fn location(&self) -> PyResult<Option<&String>> {
        self.inner
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("inner module not loaded"))
            .map(lovm2::prelude::LV2Module::location)
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for LV2Module {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.inner.as_ref().unwrap()))
    }
}

#[pyproto]
impl pyo3::class::sequence::PySequenceProtocol for LV2Module {
    fn __contains__(&self, key: &PyAny) -> PyResult<bool> {
        let key = key.str()?.to_str()?;

        Ok(match &self.inner {
            Some(m) => m.slot(&key.into()).is_some(),
            _ => false,
        })
    }
}
