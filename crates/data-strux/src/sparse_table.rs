/// A band trait.
pub trait Band: Clone {
    fn op(&self, other: &Self) -> Self;
}

/// A sparse table data structure.
///
/// # Complexity
/// Space: O(n log n)
pub struct SparseTable<S: Band> {
    data: Box<[S]>,
    n: usize,
}

impl<S: Band> SparseTable<S> {
    /// Creates a new sparse table from a vec.
    ///
    /// # Complexity
    /// Time: O(n log n)
    pub fn from_vec(mut v: Vec<S>) -> Self {
        let n = v.len();
        debug_assert!(n > 0, "sparse table must not be empty");
        let log = (usize::BITS - 1 - n.leading_zeros()) as usize;
        v.reserve(n * log);
        unsafe {
            let ptr = v.as_mut_ptr();
            for k in 1..=log {
                for i in 0..n + 1 - (1 << k) {
                    ptr.add(k * n + i).write(S::op(
                        &*ptr.add((k - 1) * n + i),
                        &*ptr.add((k - 1) * n + i + (1 << (k - 1))),
                    ));
                }
                for i in n + 1 - (1 << k)..n {
                    ptr.add(k * n + i)
                        .write((*ptr.add((k - 1) * n + i)).clone());
                }
            }
            v.set_len(n * (log + 1));
        }
        Self {
            data: v.into_boxed_slice(),
            n,
        }
    }

    /// Creates a new sparse table from a slice.
    ///
    /// # Complexity
    /// Time: O(n log n)
    pub fn from_slice(v: &[S]) -> Self {
        let n = v.len();
        debug_assert!(n > 0, "sparse table must not be empty");
        let log = (usize::BITS - 1 - n.leading_zeros()) as usize;
        let mut data = vec![v[0].clone(); n * (log + 1)];
        data[..n].clone_from_slice(v);
        unsafe {
            let d = data.as_mut_ptr();
            for k in 1..=log {
                for i in 0..n + 1 - (1 << k) {
                    *d.add(k * n + i) = S::op(
                        &*d.add((k - 1) * n + i),
                        &*d.add((k - 1) * n + i + (1 << (k - 1))),
                    );
                }
            }
        }
        Self {
            data: data.into_boxed_slice(),
            n,
        }
    }

    /// Returns `op(a[l], ..., a[r - 1])`. Range must not be empty.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn range_fold(&self, range: impl std::ops::RangeBounds<usize>) -> S {
        let l = match range.start_bound() {
            std::ops::Bound::Unbounded => 0,
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + 1,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Unbounded => self.len(),
            std::ops::Bound::Included(&x) => x + 1,
            std::ops::Bound::Excluded(&x) => x,
        };
        debug_assert!(
            l < r,
            "left bound must be less than right bound: l={}, r={}",
            l,
            r,
        );
        debug_assert!(
            r <= self.len(),
            "index out of bounds: r={}, len={}",
            r,
            self.len(),
        );
        let log = (usize::BITS - 1 - (r - l).leading_zeros()) as usize;
        unsafe {
            let d = self.data.as_ptr();
            S::op(
                &*d.add(log * self.n + l),
                &*d.add(log * self.n + r - (1 << log)),
            )
        }
    }

    /// Returns the number of elements.
    ///
    /// # Complexity
    /// Time: O(1)
    #[allow(clippy::len_without_is_empty)]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.n
    }
}
