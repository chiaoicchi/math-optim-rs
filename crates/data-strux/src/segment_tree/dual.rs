use algebrae::algebra::{Action, Monoid};

/// A dual segment tree structure.
///
/// # Complexity
/// Space: O(n)
pub struct DualSegmentTree<S: Clone, F: Monoid + Action<S>> {
    data: Box<[S]>,
    func: Box<[F]>,
}

impl<S: Clone, F: Monoid + Action<S>> DualSegmentTree<S, F> {
    /// Creates a new dual segment tree from a vec.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_vec(v: Vec<S>) -> Self {
        let n = v.len();
        debug_assert!(n > 0, "n must not be zero");
        Self {
            data: v.into_boxed_slice(),
            func: vec![F::id(); n << 1].into_boxed_slice(),
        }
    }

    /// Creates a new dual segment tree from a slice.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_slice(v: &[S]) -> Self {
        let n = v.len();
        debug_assert!(n > 0, "n must not be zero");
        Self {
            data: v.to_vec().into_boxed_slice(),
            func: vec![F::id(); n << 1].into_boxed_slice(),
        }
    }

    /// Sets the value at index `i` to `f(a[i])`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn apply(&mut self, mut i: usize, f: F) {
        debug_assert!(
            i < self.len(),
            "index out of bounds: i={}, len={}",
            i,
            self.len(),
        );
        i += self.len();
        // If action monoid is commutative, this propagation is not needed.
        self.propagate(i);
        unsafe {
            let func = self.func.as_mut_ptr();
            *func.add(i) = f.op(&*func.add(i));
        }
    }

    /// Applies action `f` to all elements in the given range.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn range_apply(&mut self, range: impl std::ops::RangeBounds<usize>, f: F) {
        let mut l = match range.start_bound() {
            std::ops::Bound::Unbounded => 0,
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + 1,
        } + self.len();
        let mut r = match range.end_bound() {
            std::ops::Bound::Unbounded => self.len(),
            std::ops::Bound::Included(&x) => x + 1,
            std::ops::Bound::Excluded(&x) => x,
        } + self.len();
        debug_assert!(
            l <= r,
            "left bound must be less than or equal to right bound: l={}, r={}",
            l - self.len(),
            r - self.len(),
        );
        debug_assert!(
            r <= self.len() << 1,
            "index out of bounds: r={}, len={}",
            r - self.len(),
            self.len(),
        );
        if l == r {
            return;
        }

        // If action monoid is commutative, this propagation is not needed.
        self.propagate(l);
        self.propagate(r - 1);

        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();

        unsafe {
            let func = self.func.as_mut_ptr();
            loop {
                if l >= r {
                    *func.add(l) = f.op(&*func.add(l));
                    l += 1;
                    l >>= l.trailing_zeros();
                } else {
                    r -= 1;
                    *func.add(r) = f.op(&*func.add(r));
                    r >>= r.trailing_zeros();
                }
                if l == r {
                    break;
                }
            }
        }
    }

    /// Returns the value at index `i`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn get(&mut self, mut i: usize) -> S {
        debug_assert!(
            i < self.len(),
            "index out of bounds: i={}, len={}",
            i,
            self.len(),
        );
        unsafe {
            let mut res = self.data.get_unchecked(i).clone();
            i += self.len();
            let func = self.func.as_mut_ptr();
            while i > 0 {
                res = (*func.add(i)).act(&res);
                i >>= 1;
            }
            res
        }
    }

    /// Returns the number of elements.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the segment tree is empty.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    fn propagate(&mut self, i: usize) {
        let func = self.func.as_mut_ptr();
        unsafe {
            for t in (1..(usize::BITS - i.leading_zeros()) as usize).rev() {
                let k = i >> t;
                let f = std::ptr::replace(func.add(k), F::id());
                *func.add(k << 1) = f.op(&*func.add(k << 1));
                *func.add((k << 1) + 1) = f.op(&*func.add((k << 1) + 1));
            }
        }
    }
}
