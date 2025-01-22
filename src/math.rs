use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
pub fn factorial(n: i32) -> PyResult<u64> {
    if n < 0 {
        Err(PyValueError::new_err(
            "factorial() not defined for negative values",
        ))
    } else {
        let mut product: u64 = 1;
        for i in 2..=n {
            product *= i as u64;
        }
        Ok(product)
    }
}
