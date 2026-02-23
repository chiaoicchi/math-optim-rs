use crate::modular::{Gf, Gf32};

macro_rules! impl_gf_binom {
    ($gf_binom:ident, $gf:ident, $t:ty) => {
        /// A precomputed table for combinatorics over Z/pZ.
        ///
        /// # Complexity
        /// Space: O(n)
        pub struct $gf_binom<const P: $t> {
            fact: Box<[$gf<P>]>,
            inv_fact: Box<[$gf<P>]>,
        }

        impl<const P: $t> $gf_binom<P> {
            /// Creates a new table with factorials up to `n` including `n`.
            ///
            /// # Complexity
            /// Time: O(n)
            pub fn new(n: usize) -> Self {
                debug_assert!(n > 0, "n must not be zero");
                let mut fact: Vec<$gf<P>> = Vec::with_capacity(n + 1);
                let mut inv_fact: Vec<$gf<P>> = Vec::with_capacity(n + 1);
                unsafe {
                    let f = fact.as_mut_ptr();
                    f.write($gf::<P>::new(1));
                    for i in 1..=n {
                        f.add(i).write(*f.add(i - 1) * $gf::<P>::from(i));
                    }
                    fact.set_len(n + 1);

                    let inv_f = inv_fact.as_mut_ptr();
                    inv_f.add(n).write((*f.add(n)).inv());
                    for i in (1..=n).rev() {
                        inv_f.add(i - 1).write(*inv_f.add(i) * $gf::<P>::from(i));
                    }
                    inv_fact.set_len(n + 1);
                }
                Self {
                    fact: fact.into_boxed_slice(),
                    inv_fact: inv_fact.into_boxed_slice(),
                }
            }

            /// Returns factorial of `n`.
            ///
            /// # Complexity
            /// Time: O(1)
            pub fn fact(&self, n: usize) -> $gf<P> {
                debug_assert!(
                    n <= self.len(),
                    "n is out of bounds: n={}, max={}",
                    n,
                    self.len()
                );
                unsafe { *self.fact.get_unchecked(n) }
            }

            /// Returns inverse factorial of `n`.
            ///
            /// # Complexity
            /// Time: O(1)
            pub fn inv_fact(&self, n: usize) -> $gf<P> {
                debug_assert!(
                    n <= self.len(),
                    "n is out of bounds: n={}, max={}",
                    n,
                    self.len()
                );
                unsafe { *self.inv_fact.get_unchecked(n) }
            }

            /// Returns permutation P(n, k), if n < k returns 0.
            ///
            /// # Complexity
            /// Time: O(1)
            pub fn perm(&self, n: usize, k: usize) -> $gf<P> {
                debug_assert!(
                    n <= self.len(),
                    "n is out of bounds: n={}, max={}",
                    n,
                    self.len()
                );

                if n < k {
                    return $gf::new(0);
                }
                unsafe {
                    let f = self.fact.as_ptr();
                    let inv_f = self.inv_fact.as_ptr();
                    *f.add(n) * *inv_f.add(n - k)
                }
            }

            /// Returns binomial coefficient [x^k](1+x)^n, if n < k returns 0.
            ///
            /// # Complexity
            /// Time: O(1)
            pub fn binom(&self, n: usize, k: usize) -> $gf<P> {
                debug_assert!(
                    n <= self.len(),
                    "n is out of bounds: n={}, max={}",
                    n,
                    self.len()
                );
                if n < k {
                    return $gf::new(0);
                }
                unsafe {
                    let f = self.fact.as_ptr();
                    let inv_f = self.inv_fact.as_ptr();
                    *f.add(n) * *inv_f.add(n - k) * *inv_f.add(k)
                }
            }

            /// Returns multiset coefficient binom(n+k-1, k).
            ///
            /// # Complexity
            /// Time: O(1)
            pub fn multichoose(&self, n: usize, k: usize) -> $gf<P> {
                if n == 0 {
                    return $gf::new(1);
                }
                debug_assert!(
                    n + k <= self.len() + 1,
                    "n+k-1 is out of bounds: n={}, k={}, max={}",
                    n,
                    k,
                    self.len(),
                );
                unsafe {
                    let f = self.fact.as_ptr();
                    let inv_f = self.inv_fact.as_ptr();
                    *f.add(n + k - 1) * *inv_f.add(n - 1) * *inv_f.add(k)
                }
            }

            /// Returns multinomial coefficient [x0^k0 x1^k1 ...](x0 + x1 + ...)^(k0 + k1 + ...)
            ///
            /// # Complexity
            /// Time: O(|k|)
            pub fn multinomial(&self, k: &[usize]) -> $gf<P> {
                let n = k.iter().sum::<usize>();
                debug_assert!(
                    n <= self.len(),
                    "n is out of bounds: n={}, max={}",
                    n,
                    self.len()
                );
                unsafe {
                    let f = self.fact.as_ptr();
                    let inv_f = self.inv_fact.as_ptr();
                    let mut res = *f.add(n);
                    for &k in k {
                        res *= *inv_f.add(k);
                    }
                    res
                }
            }

            /// Returns the limit of number.
            ///
            /// # Complexity
            /// Time: O(1)
            #[inline]
            #[allow(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                self.fact.len() - 1
            }
        }
    };
}

impl_gf_binom!(GfBinom, Gf, u64);
impl_gf_binom!(GfBinom32, Gf32, u32);
