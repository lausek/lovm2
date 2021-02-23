pub mod module;

mod code;
mod context;
mod expr;
mod lv2;
mod value;
mod vm;

use pyo3::exceptions::*;
use pyo3::prelude::*;

use lovm2::prelude::*;

pub use self::expr::Expr;
pub use self::module::{Module, ModuleBuilder};
pub use self::vm::Vm;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[pymodule]
fn pylovm2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", crate::VERSION)?;

    m.add("ENTRY_POINT", lovm2::module::ENTRY_POINT)?;
    m.add_class::<Expr>()?;
    m.add_class::<Module>()?;
    m.add_class::<ModuleBuilder>()?;
    m.add_class::<Vm>()?;

    Ok(())
}

pub(crate) fn err_to_exception(e: Lovm2Error) -> PyErr {
    let msg = e.to_string();
    match &e.ty {
        Lovm2ErrorTy::Custom(ty) => match ty.as_ref() {
            "AssertionError" => PyAssertionError::new_err(msg),
            "Exception" => PyException::new_err(msg),
            "FileNotFoundError" => PyFileNotFoundError::new_err(msg),
            "ImportError" => PyImportError::new_err(msg),
            "ZeroDivisionError" => PyZeroDivisionError::new_err(msg),
            _ => PyException::new_err(msg),
        },
        Lovm2ErrorTy::ModuleNotFound => PyImportError::new_err(msg),
        _ => PyException::new_err(msg),
    }
}

pub(crate) fn exception_to_err(e: &PyErr, py: Python) -> Lovm2Error {
    let ty = e.ptype(py).name().to_string();
    let msg = e.pvalue(py).to_string();

    (ty, msg).into()
}
