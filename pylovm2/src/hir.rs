use pyo3::prelude::*;

use crate::expr::Expr;
use crate::module::{Module, ModuleBuilder};

#[pymodule]
pub fn hir(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Expr>()?;
    m.add_class::<Module>()?;
    m.add_class::<ModuleBuilder>()?;

    Ok(())
}
