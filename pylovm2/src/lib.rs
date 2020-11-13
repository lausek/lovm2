mod code;
mod context;
mod expr;
mod lv2;
mod module;
mod value;
mod vm;

use pyo3::prelude::*;

use self::expr::Expr;
use self::module::{Module, ModuleBuilder, ModuleBuilderSlot};
use self::vm::Vm;

#[pymodule]
fn pylovm2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("ENTRY_POINT", lovm2::module::ENTRY_POINT)?;
    m.add_class::<Expr>()?;
    m.add_class::<Module>()?;
    m.add_class::<ModuleBuilder>()?;
    m.add_class::<ModuleBuilderSlot>()?;
    m.add_class::<Vm>()?;

    Ok(())
}
