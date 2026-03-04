macro_rules! impl_int_binom {
    ($int_binom:ident, $int:ty) => {
        /// A precomputed table for combinatorics over Z.
        ///
        /// # Complexity
        /// Space: O(n^2)
        pub struct $int_binom(Box<[$int]>);

        impl $int_binom {
            /// Creates a new table with Pascal triangle up to `n` including `n`
            ///
            /// # Complexity
            /// Time: O(n^2)
            pub fn new(n: usize) -> Self {
                let mut data: Vec<$int> = Vec::with_capacity(((n + 1) * (n + 2)) >> 1);
                unsafe {
                    let ptr = data.as_mut_ptr();
                    ptr.write(1);
                    for i in 1..=n {
                        ptr.add((i * (i + 1)) >> 1).write(1);
                        for j in 1..i {
                            ptr.add(((i * (i + 1)) >> 1) + j).write(
                                *ptr.add((((i - 1) * i) >> 1) + j - 1)
                                    + *ptr.add((((i - 1) * i) >> 1) + j),
                            );
                        }
                        ptr.add(((i * (i + 3)) >> 1)).write(1);
                    }
                    data.set_len(((n + 1) * (n + 2) >> 1));
                }
                Self(data.into_boxed_slice())
            }

            /// Returns binomial coefficient [x^k](1+x)^n, if n < k returns 0.
            ///
            /// # Complexity
            /// Time: O(1)
            pub fn binom(&self, n: usize, k: usize) -> $int {
                if n < k {
                    return 0;
                }
                unsafe { *self.0.get_unchecked(((n * (n + 1)) >> 1) + k) }
            }

            /// Returns the limit of number.
            ///
            /// # Complexity
            /// Time: O(1)
            #[inline]
            #[allow(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                (self.0.len() << 1).isqrt() - 1
            }
        }
    };
}

impl_int_binom!(IntBinom32, u32);
impl_int_binom!(IntBinom, u64);
impl_int_binom!(IntBinom128, u128);
