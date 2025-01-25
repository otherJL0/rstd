use crate::sieve;
use num_bigint::BigUint;

mod xmath {
    use num_bigint::BigUint;
    use num_bigint::ToBigUint;

    /// Returns the integer floor of the square root of `n`.
    /// (For large `u64`, you may want a more careful approach,
    /// but this is fine for typical ranges.)
    pub fn floor_sqrt(n: u64) -> u64 {
        (n as f64).sqrt() as u64
    }

    /// Returns the population count (number of set bits) of `n`.
    /// This is the Rust equivalent of Go’s `BitCount64(n)`.
    #[inline]
    pub fn bit_count64(n: u64) -> u32 {
        n.count_ones()
    }

    /// Multiplies all numbers in `factors` together into a BigUint.
    pub fn product_u64(factors: &[u64]) -> BigUint {
        let mut result = 1u64.to_biguint().unwrap();
        for &f in factors {
            result *= f;
        }
        result
    }
}

/// The "Swing" struct storing a prime sieve and (optionally) a buffer
/// for collecting prime factors.
pub struct Swing {
    primes: sieve::Sieve,
    factors: Option<Vec<u64>>,
}

/// A static table (similar to `smallOddSwing` in the Go code).
/// Because all values are positive and fit under 2^63, we can store them as `u64`.
static SMALL_ODD_SWING: [u64; 65] = [
    1,
    1,
    1,
    3,
    3,
    15,
    5,
    35,
    35,
    315,
    63,
    693,
    231,
    3003,
    429,
    6435,
    6435,
    109395,
    12155,
    230945,
    46189,
    969969,
    88179,
    2028117,
    676039,
    16900975,
    1300075,
    35102025,
    5014575,
    145422675,
    9694845,
    300540195,
    300540195,
    9917826435,
    583401555,
    20419054425,
    2268783825,
    83945001525,
    4418157975,
    172308161025,
    34461632205,
    1412926920405,
    67282234305,
    2893136075115,
    263012370465,
    11835556670925,
    514589420475,
    24185702762325,
    8061900920775,
    395033145117975,
    15801325804719,
    805867616040669,
    61989816618513,
    3285460280781189,
    121683714103007,
    6692604275665385,
    956086325095055,
    54496920530418135,
    1879204156221315,
    110873045217057585,
    7391536347803839,
    450883717216034179,
    14544636039226909,
    916312070471295267,
    916312070471295267,
];

impl Swing {
    /// Constructs a new `Swing` for factorial computations up to `n`.
    /// This parallels `NewSwing(n uint64)`.
    pub fn new(n: u64) -> Self {
        // Create prime sieve up to n
        let primes = sieve::Sieve::new(n);

        // Only allocate `factors` if n >= length of the small table
        let factors = if n >= SMALL_ODD_SWING.len() as u64 {
            // In Go, it does `make([]uint64, n)`. We can do the same capacity,
            // but we typically use a Vec in Rust. We'll store it in an Option
            // and only fill it as we go.
            Some(vec![0u64; n as usize])
        } else {
            None
        };

        Swing { primes, factors }
    }

    /// Computes the "odd swing" portion of n! (the product of certain
    /// primes/factors described by A056040).
    /// This matches your Go `OddSwing(k uint64)`.
    ///
    /// Returns a `BigUint`.
    fn odd_swing(&mut self, k: u64) -> BigUint {
        use xmath::{floor_sqrt, product_u64};
        // If `k` is within our small table, return directly.
        if (k as usize) < SMALL_ODD_SWING.len() {
            return BigUint::from(SMALL_ODD_SWING[k as usize]);
        }

        let root_k = floor_sqrt(k);

        // We'll track how many factors we actually push.
        // This replicates Go’s `i` index.
        let mut i: usize = 0;
        let factors_slice = self
            .factors
            .as_mut()
            .expect("Logic error: factors Vec not allocated.");

        // 1) For primes in [3 .. root_k], repeatedly factor out p whenever k/p is odd
        self.primes.iterate_primes(3, root_k, |p| {
            let mut q = k / p;
            while q > 0 {
                if (q & 1) == 1 {
                    factors_slice[i] = p;
                    i += 1;
                }
                q >>= 1;
            }
        });

        // 2) For primes in [(root_k+1) .. (k/3)], if k/p is odd, collect p
        self.primes.iterate_primes(root_k + 1, k / 3, |p| {
            if ((k / p) & 1) == 1 {
                factors_slice[i] = p;
                i += 1;
            }
        });

        // 3) For primes in [(k/2+1) .. k], collect them all
        self.primes.iterate_primes((k / 2) + 1, k, |p| {
            factors_slice[i] = p;
            i += 1;
        });

        // Now multiply all these collected factors
        product_u64(&factors_slice[..i])
    }
}

/// Computes n! as described by the “Swinging Factorial” approach
/// and returns it as a BigUint. This matches Go’s `SwingingFactorial(n)`.
///
/// In particular, it computes `odd_swing(n) << BitCount64(n>>1)`.
pub fn swinging_factorial(n: u64) -> BigUint {
    use xmath::bit_count64;

    // Construct a Swing for n
    let mut s = Swing::new(n);

    // Compute odd swing portion
    let mut r = s.odd_swing(n);

    // Now left-shift by popcount(n >> 1).
    // Equivalent to `r.Lsh(r, xmath.BitCount64(n>>1))` in Go.
    r <<= bit_count64(n >> 1);
    r
}
