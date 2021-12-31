pub mod module;

mod code;
mod context;
mod expr;
mod value;
mod vm;

use pyo3::exceptions::*;
use pyo3::prelude::*;

pub use self::expr::LV2Expr;
pub use self::module::{LV2Module, LV2ModuleBuilder};
pub use self::vm::LV2Vm;

#[derive(Clone)]
#[pyclass]
pub struct LV2Variable {
    inner: lovm2::prelude::LV2Variable,
}

#[pymethods]
impl LV2Variable {
    #[new]
    pub fn new(from: &PyAny) -> PyResult<Self> {
        let inner = crate::expr::any_to_ident(from)?;
        Ok(Self { inner })
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for LV2Variable {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[pymodule]
fn pylovm2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", crate::VERSION)?;

    m.add("LV2_ENTRY_POINT", lovm2::module::LV2_ENTRY_POINT)?;
    m.add_class::<LV2Expr>()?;
    m.add_class::<LV2Module>()?;
    m.add_class::<LV2ModuleBuilder>()?;
    m.add_class::<LV2Variable>()?;
    m.add_class::<LV2Vm>()?;

    Ok(())
}

pub(crate) fn err_to_exception(e: lovm2::prelude::LV2Error) -> PyErr {
    let msg = e.to_string();
    match &e.ty {
        lovm2::error::LV2ErrorTy::Custom(ty) => match ty.as_ref() {
            "AssertionError" => PyAssertionError::new_err(msg),
            "Exception" => PyException::new_err(msg),
            "FileNotFoundError" => PyFileNotFoundError::new_err(msg),
            "ImportError" => PyImportError::new_err(msg),
            "ZeroDivisionError" => PyZeroDivisionError::new_err(msg),
            _ => PyException::new_err(msg),
        },
        lovm2::error::LV2ErrorTy::ModuleNotFound => PyImportError::new_err(msg),
        _ => PyException::new_err(msg),
    }
}

pub(crate) fn exception_to_err(e: &PyErr, py: Python) -> lovm2::prelude::LV2Error {
    let ty = e.ptype(py).name().to_string();
    let msg = e.pvalue(py).to_string();

    (ty, msg).into()
}
