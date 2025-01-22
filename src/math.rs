use num_bigint::BigInt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
pub fn factorial(n: i32) -> PyResult<BigInt> {
    if n < 0 {
        Err(PyValueError::new_err(
            "factorial() not defined for negative values",
        ))
    } else {
        let mut product: BigInt = BigInt::from(1);
        for i in 2..=n {
            product *= BigInt::from(i);
        }
        Ok(product)
    }
}
