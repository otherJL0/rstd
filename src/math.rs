use num_bigint::BigUint;
use num_traits::One;
use pyo3::{exceptions::PyValueError, prelude::*};

fn factorial_u64(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut product = n;
    for i in 1..n {
        product *= i;
    }
    product
}

fn factorial_u128(n: u128) -> u128 {
    let mut product = n;
    for i in 1..n {
        product *= i;
    }
    product
}

fn factorial_biguint(n: u64) -> BigUint {
    let mut product = BigUint::one();
    for i in 1..=n {
        product *= BigUint::from(i);
    }
    product
}

fn product_range(end: u64, start: u64) -> BigUint {
    let mut product = BigUint::one();
    for i in start..=end {
        product *= BigUint::from(i);
    }
    product
}

#[pyfunction]
pub fn factorial(n: i64) -> PyResult<BigUint> {
    if n < 0 {
        Err(PyValueError::new_err(
            "factorial() not defined for negative values",
        ))
    } else if n < 21 {
        Ok(BigUint::from(factorial_u64(n as u64)))
    } else if n < 35 {
        Ok(BigUint::from(factorial_u128(n as u128)))
    } else {
        Ok(factorial_biguint(n as u64))
    }
}

#[pyfunction]
pub fn comb(n: i64, k: i64) -> PyResult<BigUint> {
    if n < 0 {
        Err(PyValueError::new_err(
            "factorial() not defined for negative values",
        ))
    } else if k > n {
        Ok(BigUint::ZERO)
    } else {
        let k = k.min(n - k);
        let n = n as u64;
        Ok(product_range(n - k as u64, n) / factorial(k).unwrap())
    }
}

/// Use Newton-Raphson algorithm to compute isqrt
#[pyfunction]
pub fn isqrt(n: i64) -> PyResult<i64> {
    if n < 0 {
        return Err(PyValueError::new_err(
            "isqrt() argument must be nonnegative",
        ));
    }
    if n < 2 {
        return Ok(n);
    }
    let mut x0 = n / 2;
    let mut x1 = (x0 + n / x0) / 2;
    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + n / x0) / 2;
    }
    Ok(x0)
}

#[pyfunction]
#[pyo3(signature = (n, k=None))]
pub fn perm(n: i64, k: Option<i64>) -> BigUint {
    let n = n as u64;
    match k {
        None => product_range(1, n),
        Some(start) => {
            let start = start as u64;
            if start > n {
                BigUint::ZERO
            } else {
                product_range(n - start, n)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_isqrt() {
        for i in 4..10000 {
            let square: i64 = i * i;
            if let Ok(less_result) = isqrt(square - 1) {
                assert_eq!(
                    less_result,
                    i - 1,
                    "one less of square {} should be {}",
                    less_result,
                    i - 1
                );
            }
            assert_eq!(isqrt(square).unwrap(), i);
            if let Ok(greater_result) = isqrt(square + 1) {
                assert_eq!(
                    greater_result, i,
                    "one greater of square {greater_result} should be {i}",
                );
            }
        }
    }

    #[bench]
    fn bench_binary_search_isqrt(b: &mut Bencher) {
        b.iter(|| {
            (i64::MIN..i64::MAX).for_each(|n| {
                let _ = isqrt(n);
            });
        });
    }
}
