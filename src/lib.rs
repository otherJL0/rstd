mod math;
use pyo3::prelude::*;

fn register_math_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "math")?;
    submodule.add_function(wrap_pyfunction!(math::factorial, &submodule)?)?;
    parent_module
        .py()
        .import("sys")?
        .getattr("modules")?
        .set_item("rstd.math", &submodule)?;
    parent_module.add_submodule(&submodule)
}

#[pymodule]
fn rstd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_math_submodule(m)?;
    Ok(())
}
