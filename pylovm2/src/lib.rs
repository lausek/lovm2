mod code;
mod expr;
mod hir;
mod module;
mod vm;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use self::hir::*;
use self::vm::Vm;

#[pymodule]
fn pylovm2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vm>()?;
    m.add_wrapped(wrap_pymodule!(hir))?;

    Ok(())
}
