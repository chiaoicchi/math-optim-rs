/// A sieve of Eratosthenes structure.
///
/// # Complexity
/// Space: O(n)
pub struct SieveEratosthenes(Box<[u64]>);

impl SieveEratosthenes {
    /// Creates a new sieve of Eratosthenes table up to `n` including `n`.
    ///
    /// # Complexity
    /// Time: O(n loglog n)
    pub fn new(n: usize) -> Self {
        debug_assert!(n > 0, "n must not be zero");
        let blocks = (n >> 7) + 1;
        let mut table = vec![!0; blocks + 1];
        unsafe {
            let t = table.as_mut_ptr();
            *t &= !1;
            for i in 1.. {
                let j = (i << 1) + 1;
                if j * j > n {
                    break;
                }
                if (*t.add(i >> 6) >> (i & 63)) & 1 == 1 {
                    let mut k = j * j >> 1;
                    while k >> 6 < blocks {
                        *t.add(k >> 6) &= !(1 << (k & 63));
                        k += j;
                    }
                }
            }
            *t.add((n - 1) >> 7) &= !0 >> (63 - (((n - 1) >> 1) & 63));
            *t.add(((n - 1) >> 7) + 1) = 0;
            *t.add(blocks) = n as u64;
        }
        Self(table.into_boxed_slice())
    }

    /// Returns whether `n` is prime.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn is_prime(&self, n: usize) -> bool {
        debug_assert!(
            n <= self.len(),
            "n is out of bounds: n={}, max={}",
            n,
            self.len(),
        );
        unsafe {
            n == 2
                || (n & 1 == 1 && (self.0.get_unchecked(n >> 7) >> ((n >> 1) as u64 & 63)) & 1 == 1)
        }
    }

    /// Returns the number of primes in table.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn count_primes(&self) -> usize {
        let n = self.len();
        if n < 2 {
            return 0;
        }
        1 + self.0[..self.0.len() - 1]
            .iter()
            .map(|&w| w.count_ones() as usize)
            .sum::<usize>()
    }

    /// Collects all primes up to `n` including `n`.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn primes(&self) -> Vec<usize> {
        let n = self.len();
        let count = self.count_primes();
        let mut primes = Vec::with_capacity(count);
        let ptr = primes.as_mut_ptr();
        let mut idx = 0;
        if n >= 2 {
            unsafe {
                *ptr = 2;
                idx += 1;
            }
        }
        for (i, &block) in self.0[..self.0.len() - 1].iter().enumerate() {
            let mut b = block;
            while b != 0 {
                let bit = b.trailing_zeros() as usize;
                b &= b - 1;
                unsafe {
                    *ptr.add(idx) = (i << 6 | bit) << 1 | 1;
                }
                idx += 1;
            }
        }
        unsafe {
            primes.set_len(count);
        }
        primes
    }

    /// Returns the limit of number.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { *self.0.get_unchecked(self.0.len() - 1) as usize }
    }
}
