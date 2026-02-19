/// A commutative monoid trait.
pub trait Monoid: Clone {
    /// Returns the identity element of the monoid.
    fn id() -> Self;
    /// Performs the binary operation of the monoid.
    fn op(&self, other: &Self) -> Self;
}

/// A commutative group trait.
pub trait Group: Monoid {
    /// Returns the inverse of the element.
    fn inv(&self) -> Self;
}

/// A fenwick tree data structure.
///
/// # Complexity
/// Space: O(n)
pub struct FenwickTree<S: Monoid>(Vec<S>);

impl<S: Monoid> FenwickTree<S> {
    /// Creates a new fenwick tree with `n` elements, where all initialized to `S::id()`.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn new(n: usize) -> Self {
        Self(vec![S::id(); n + 1])
    }

    /// Creates a fenwick tree from a vec.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_vec(mut v: Vec<S>) -> Self {
        let n = v.len();
        v.reserve(1);
        unsafe {
            let ptr = v.as_mut_ptr();
            std::ptr::copy(ptr, ptr.add(1), n);
            ptr.write(S::id());
            v.set_len(n + 1);
            for i in 1..=n {
                let lsb = i & i.wrapping_neg();
                if i + lsb <= n {
                    *ptr.add(i + lsb) = S::op(&*ptr.add(i + lsb), &*ptr.add(i));
                }
            }
        }
        Self(v)
    }

    /// Creates a fenwick tree from a slice.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_slice(v: &[S]) -> Self {
        let n = v.len();
        let mut data = Vec::with_capacity(n + 1);
        data.push(S::id());
        data.extend_from_slice(v);
        unsafe {
            let d = data.as_mut_ptr();
            for i in 1..=n {
                let lsb = i & i.wrapping_neg();
                if i + lsb <= n {
                    *d.add(i + lsb) = S::op(&*d.add(i + lsb), &*d.add(i));
                }
            }
        }
        Self(data)
    }

    /// Appends an element to end.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn push(&mut self, mut x: S) {
        let lsb = self.0.len() & self.0.len().wrapping_neg();
        let mut t = 1;
        unsafe {
            let d = self.0.as_mut_ptr();
            while t < lsb {
                x = S::op(&x, &*d.add(self.0.len() - t));
                t <<= 1;
            }
            self.0.push(x);
        }
    }

    /// Removes the last element.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn pop(&mut self) {
        if !self.is_empty() {
            self.0.pop();
        }
    }

    /// Sets the value at index `i` to `op(a[i], x)`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn operate(&mut self, mut i: usize, x: S) {
        debug_assert!(
            i < self.len(),
            "index out of bounds: i={}, len={}",
            i,
            self.len(),
        );
        i += 1;
        unsafe {
            let d = self.0.as_mut_ptr();
            while i < self.0.len() {
                *d.add(i) = S::op(&*d.add(i), &x);
                i += i & i.wrapping_neg();
            }
        }
    }

    /// Returns `op(a[0], .., a[r - 1])`. When the range is empty, returns `S::id()`.
    ///
    /// # Complexity
    /// O(log n)
    pub fn prefix_fold(&self, mut r: usize) -> S {
        debug_assert!(
            r <= self.len(),
            "index out of bounds: r={}, len={}",
            r,
            self.len(),
        );
        unsafe {
            let mut res = self.0.get_unchecked(r).clone();
            let d = self.0.as_ptr();
            while r > 0 {
                r &= r - 1;
                res = S::op(&*d.add(r), &res);
            }
            res
        }
    }

    /// Returns `op(a[0], ..., a[n - 1])`.
    ///
    /// # Complexity
    /// O(log n)
    pub fn all_fold(&self) -> S {
        self.prefix_fold(self.len())
    }

    /// Returns the number of elements.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len() - 1
    }

    /// Returns whether the fenwick tree is empty.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.len() == 1
    }
}

impl<S: Group> FenwickTree<S> {
    /// Sets the value at index `i` to `x`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn set(&mut self, i: usize, x: S) {
        debug_assert!(
            i < self.len(),
            "index out of bounds: i={}, len={}",
            i,
            self.len(),
        );
        let diff = S::op(&self.get(i).inv(), &x);
        self.operate(i, diff);
    }

    /// Returns the value at index `i`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn get(&self, i: usize) -> S {
        debug_assert!(
            i < self.len(),
            "index out of bounds: i={}, len={}",
            i,
            self.len(),
        );
        S::op(&self.prefix_fold(i).inv(), &self.prefix_fold(i + 1))
    }

    /// Returns `op(a[l], ..., a[r - 1])`. When range is empty, returns `S::id()`.
    ///
    /// # Complexity
    /// Time: O(log n)
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
            l <= r,
            "left bound must be less than or equal to right bound: l={}, r={}",
            l,
            r,
        );
        debug_assert!(
            r <= self.len(),
            "index out of bounds: r={}, len={}",
            r,
            self.len(),
        );
        S::op(&self.prefix_fold(l).inv(), &self.prefix_fold(r))
    }
}
