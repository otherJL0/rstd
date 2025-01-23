use num_bigint::BigInt;
use pyo3::{exceptions::PyValueError, prelude::*};

#[inline(always)]
fn _factorial(n: u64, end: u64) -> BigInt {
    let mut product = BigInt::from(n);
    for i in (end + 1..n).rev() {
        product *= BigInt::from(i);
    }
    product
}

#[pyfunction]
pub fn factorial(n: u64) -> BigInt {
    _factorial(n, 0)
}

#[pyfunction]
pub fn comb(n: u64, k: u64) -> BigInt {
    if k > n {
        return BigInt::ZERO;
    }
    let k = k.min(n - k);
    _factorial(n, n - k) / _factorial(k, 0)
}

#[pyfunction]
pub fn isqrt(n: BigInt) -> PyResult<BigInt> {
    if n < BigInt::ZERO {
        return Err(PyValueError::new_err(
            "isqrt() argument must be nonnegative",
        ));
    }
    if n == BigInt::ZERO {
        return Ok(BigInt::ZERO);
    }

    if n < BigInt::from(4) {
        return Ok(BigInt::from(1));
    }
    let mut low = BigInt::ZERO;
    let mut high = n.clone();
    while low < high {
        let mid: BigInt = (&low + &high) >> 1;
        let mid_squared = &mid * &mid;
        if mid_squared > n {
            high = mid.clone();
        } else if mid_squared == n || (&mid + 1) * (&mid + 1) > n {
            return Ok(mid);
        } else {
            low = mid.clone();
        }
    }
    Ok(low)
}

#[pyfunction]
#[pyo3(signature = (n, k=None))]
pub fn perm(n: u64, k: Option<u64>) -> BigInt {
    match k {
        None => _factorial(n, 0),
        Some(_k) => {
            if _k > n {
                BigInt::ZERO
            } else {
                _factorial(n, n - _k)
            }
        }
    }
}
