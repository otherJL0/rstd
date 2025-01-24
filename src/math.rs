use num_bigint::BigInt;
use pyo3::prelude::*;

#[inline(always)]
fn _factorial(end: i64, start: i64) -> BigInt {
    let mut product = BigInt::from(end);
    for i in (start + 1..end).rev() {
        product *= BigInt::from(i);
    }
    product
}

#[pyfunction]
pub fn factorial(n: i64) -> BigInt {
    _factorial(0, n)
}

#[pyfunction]
pub fn comb(n: i64, k: i64) -> BigInt {
    if k > n {
        return BigInt::ZERO;
    }
    let k = k.min(n - k);
    _factorial(n - k, n) / _factorial(0, k)
}

#[pyfunction]
pub fn isqrt(n: i64) -> PyResult<i64> {
    if n == 0 {
        return Ok(0);
    }
    if n < 4 {
        return Ok(1);
    }
    let mut low = 0;
    let mut high = n;
    while low < high {
        let mid = (low + high) >> 1;
        let mid_squared = mid * mid;
        if mid_squared > n {
            high = mid;
        } else if mid_squared == n || (mid + 1) * (mid + 1) > n {
            return Ok(mid);
        } else {
            low = mid;
        }
    }
    Ok(low)
}

#[pyfunction]
#[pyo3(signature = (n, k=None))]
pub fn perm(n: i64, k: Option<i64>) -> BigInt {
    match k {
        None => _factorial(0, n),
        Some(_k) => {
            if _k > n {
                BigInt::ZERO
            } else {
                _factorial(n - _k, n)
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
                    "one greater of square {} should be {}",
                    greater_result, i,
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
        })
    }
}
