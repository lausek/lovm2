pub mod module;

mod code;
mod context;
mod expr;
mod value;
mod vm;

use pyo3::exceptions::*;
use pyo3::prelude::*;

use lovm2::prelude::*;

pub use self::expr::Expr;
pub use self::module::{Module, ModuleBuilder};
pub use self::vm::Vm;

/*
pub(crate) type Lovm2Branch = lovm2::gen::Branch;
pub(crate) type Lovm2Block = lovm2::gen::Block;
pub(crate) type Lovm2Context = lovm2::vm::Context;
pub(crate) type Lovm2Expr = lovm2::gen::Expr;
pub(crate) type Lovm2Frame = lovm2::vm::Frame;
pub(crate) type Lovm2Module = lovm2::module::Module;
pub(crate) type Lovm2ModuleBuilder = lovm2::gen::ModuleBuilder;
pub(crate) type Lovm2Ref = lovm2::value::Reference;
pub(crate) type Lovm2Slots = lovm2::module::Slots;
pub(crate) type Lovm2Value = lovm2::value::ValueRef;
pub(crate) type Lovm2ValueRaw = lovm2::value::Value;
pub(crate) type Lovm2ValueType = lovm2::value::ValueType;
*/

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[pymodule]
fn pylovm2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", crate::VERSION)?;

    m.add("LV2_ENTRY_POINT", lovm2::module::LV2_ENTRY_POINT)?;
    m.add_class::<Expr>()?;
    m.add_class::<Module>()?;
    m.add_class::<ModuleBuilder>()?;
    m.add_class::<Vm>()?;

    Ok(())
}

pub(crate) fn err_to_exception(e: LV2Error) -> PyErr {
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

pub(crate) fn exception_to_err(e: &PyErr, py: Python) -> LV2Error {
    let ty = e.ptype(py).name().to_string();
    let msg = e.pvalue(py).to_string();

    (ty, msg).into()
}
