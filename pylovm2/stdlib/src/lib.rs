use pyo3::prelude::*;

use pylovm2::Module;

#[pyfunction]
pub fn create_std_module() -> Module {
    Module::from(lovm2_std::create_std_module())
}

#[pymodule]
fn pylovm2_stdlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(pyo3::wrap_pyfunction!(create_std_module))?;
    Ok(())
}
