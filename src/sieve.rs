use num_bigint::BigUint;

/// 64-bit word size constants matching the Go version
const BITS_PER_INT: u64 = 64;
const MASK: u64 = BITS_PER_INT - 1;
const LOG2_INT: u64 = 6;

/// Holds the completed sieve. The underlying vector stores
/// flags for composites in blocks of 64 bits each.
///
/// - `sieve_len`: The maximum number for which this Sieve can accurately
///   answer primality queries.
/// - `is_composite`: Each `u64` is treated as a 64-bit field of flags.
///   If `is_composite[i] & (1 << j) != 0`, then the integer mapped by
///   `(i, j)` is composite (not prime).
pub struct Sieve {
    pub sieve_len: u64,
    is_composite: Vec<u64>,
}

#[allow(dead_code)]
impl Sieve {
    /// Constructs and returns a new sieve for numbers up to `n`.
    /// This mimics the specialized approach in your Go code:
    /// it skips even numbers (other than 2) and uses a 3-based
    /// indexing system.
    pub fn new(n: u64) -> Self {
        // Create the empty Sieve struct
        let mut s = Sieve {
            sieve_len: n,
            is_composite: Vec::new(),
        };

        // Small n case: If `n < 965` we just short-circuit
        // with a pre-computed sieve array (taken from your Go code).
        if n < 965 {
            s.is_composite = vec![
                3644759964122252416,
                10782565419096678876,
                5393006238418678630,
                7319957818701628715,
                16892333181782511326,
            ];
            return s;
        }

        // Otherwise, allocate enough space to mark composites
        // up to `n`.
        //
        // The Go code uses `n / (3 * 64) + 1`, so we replicate that here:
        let size = (n / (3 * BITS_PER_INT) + 1) as usize;
        s.is_composite = vec![0u64; size];

        // Variables used in the sieve cancellation loop:
        let (mut d1, mut d2) = (8, 8);
        let (mut p1, mut p2) = (3, 7);
        let (mut s1, mut s2) = (7, 3);
        let mut l = 0u64;
        let mut toggle = false;
        let max = n / 3;

        // Main sieve cancellation loop:
        while s1 < max {
            // If we haven't marked this position as composite,
            // it corresponds to a prime. We proceed to mark
            // its multiples.
            let index = (l >> LOG2_INT) as usize;
            let bit = 1 << (l & MASK);
            if (s.is_composite[index] & bit) == 0 {
                // inc = p1 + p2
                let inc = p1 + p2;

                // Mark off s1, s1+inc, s1+2*inc, ...
                for c in (s1..max).step_by(inc as usize) {
                    let idx = (c >> LOG2_INT) as usize;
                    let composite_bit = 1 << (c & MASK);
                    s.is_composite[idx] |= composite_bit;
                }
                // Mark off s1 + s2, s1 + s2 + inc, ...
                for c in ((s1 + s2)..max).step_by(inc as usize) {
                    let idx = (c >> LOG2_INT) as usize;
                    let composite_bit = 1 << (c & MASK);
                    s.is_composite[idx] |= composite_bit;
                }
            }

            l += 1;
            toggle = !toggle;
            if toggle {
                s1 += d2;
                d1 += 16;
                p1 += 2;
                p2 += 2;
                s2 = p2;
            } else {
                s1 += d1;
                d2 += 8;
                p1 += 2;
                p2 += 6;
                s2 = p1;
            }
        }

        s
    }

    /// Iterates over primes in the range [`min`, `max`], calling `visitor`
    /// on each prime found.
    ///
    /// - Skips anything above the sieve’s limit `self.sieve_len`.
    /// - If `max < 2`, there are no primes to visit.
    pub fn iterate_primes<F>(&self, min: u64, max: u64, mut visitor: F)
    where
        F: FnMut(u64),
    {
        if max > self.sieve_len || max < 2 {
            return;
        }

        // 2 is prime if in range
        if min <= 2 {
            visitor(2);
        }

        // 3 is prime if in range
        if min <= 3 && 3 <= max {
            visitor(3);
        }

        // We skip even numbers > 2, so we need to convert
        // from the actual min, max to the "3-based" indexing
        let abs_pos = (min + (min + 1) % 2) / 3 - 1;
        let mut index = abs_pos / BITS_PER_INT;
        let mut bit_pos = abs_pos % BITS_PER_INT;
        let mut prime = 5 + 3 * (BITS_PER_INT * index + bit_pos) - (bit_pos & 1);
        let mut inc = (bit_pos & 1) * 2 + 2;

        // We will loop until `prime > max`
        while prime <= max {
            // Extract a 64-bit chunk from `is_composite`
            let mut bit_field = self.is_composite[index as usize] >> bit_pos;
            // Move to the next 64-bit chunk afterwards
            index += 1;

            // Iterate over bits in this 64-bit chunk
            while bit_pos < BITS_PER_INT {
                // If the current bit is not set, `prime` is prime
                if (bit_field & 1) == 0 {
                    visitor(prime);
                }

                prime += inc; // move to the next prime candidate
                if prime > max {
                    return;
                }

                inc = 6 - inc; // toggle between +2 and +4 steps

                bit_field >>= 1;
                bit_pos += 1;
            }

            bit_pos = 0; // continue with the next 64-bit chunk
        }
    }

    /// Returns the total count of primes from `1` up to `n`.
    /// This is a static helper that constructs a new sieve
    /// and counts the primes within it.
    pub fn number_of_primes_not_exceeding(n: u64) -> usize {
        let sieve = Sieve::new(n);
        let mut count = 0;
        sieve.iterate_primes(1, n, |_prime| {
            count += 1;
        });
        count
    }

    /// Returns the count of primes within the sieve between
    /// `[low, high]`.
    pub fn number_of_primes(&self, low: u64, high: u64) -> usize {
        if high > self.sieve_len {
            panic!("high bound not in the range of the sieve.");
        }

        let mut count = 0;
        self.iterate_primes(low, high, |_p| {
            count += 1;
        });
        count
    }

    /// Returns `true` if `n` is prime, otherwise `false`.
    pub fn is_prime(&self, n: u64) -> bool {
        if n > self.sieve_len {
            panic!("n not in the range of the sieve.");
        }
        let mut found_count = 0;
        self.iterate_primes(n, n, |_prime| {
            found_count += 1;
        });
        found_count == 1
    }

    /// Computes the product of all primes between `[lo, hi]`,
    /// known as the “primorial” in that range.
    ///
    /// Uses a divide-and-conquer approach to avoid building
    /// huge intermediate arrays. If `hi - lo < 200`,
    /// it directly multiplies the primes in that small range;
    /// otherwise it splits the range into two halves, multiplies
    /// the results, and merges.
    pub fn primorial(&self, lo: u64, hi: u64) -> BigUint {
        use num_bigint::ToBigUint;

        if lo > hi {
            return 1u64.to_biguint().unwrap();
        }

        // small range
        if hi - lo < 200 {
            let mut result = BigUint::from(1u64);
            self.iterate_primes(lo, hi, |p| {
                result *= p;
            });
            return result;
        }

        // large range: split
        let mid = (lo + hi) / 2;
        let left = self.primorial(lo, mid);
        let right = self.primorial(mid + 1, hi);
        left * right
    }
}
