mod builder;
mod slot;

use pyo3::exceptions::*;
use pyo3::prelude::*;

use lovm2::module;

pub use self::builder::ModuleBuilder;
pub use self::slot::ModuleBuilderSlot;

type Lovm2Branch = lovm2::hir::branch::Branch;
type Lovm2Block = lovm2::hir::block::Block;
type Lovm2Module = lovm2::module::Module;

#[pyclass]
pub struct Module {
    pub inner: Option<Box<dyn module::ModuleProtocol>>,
}

impl Module {
    pub fn from(inner: Lovm2Module) -> Self {
        Self {
            inner: Some(inner.into()),
        }
    }
}

#[pymethods]
impl Module {
    #[classmethod]
    pub fn load(_this: &PyAny, path: &PyAny) -> PyResult<Self> {
        let path = path.str()?.to_string()?;
        match Lovm2Module::load_from_file(path.as_ref()) {
            Ok(inner) => Ok(Self { inner: Some(inner) }),
            Err(err) => RuntimeError::into(err),
        }
    }

    pub fn save(&self, path: String) -> PyResult<()> {
        if let Some(inner) = self.inner.as_ref() {
            return match inner.store_to_file(&path) {
                Ok(_) => Ok(()),
                Err(err) => RuntimeError::into(err),
            };
        }
        RuntimeError::into("inner module not loaded")
    }

    pub fn uses(&self) -> PyResult<Vec<String>> {
        // TODO: implement this
        Ok(vec![])
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Module {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner.as_ref().unwrap()))
    }
}

#[pyproto]
impl pyo3::class::sequence::PySequenceProtocol for Module {
    fn __contains__(&self, key: &PyAny) -> PyResult<bool> {
        let key = key.str()?.to_string()?.to_string();
        Ok(match &self.inner {
            Some(m) => m.slot(&key.into()).is_some(),
            _ => false,
        })
    }
}
