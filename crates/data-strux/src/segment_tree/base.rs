use crate::segment_tree::Monoid;

/// A segment tree structure.
///
/// # Complexity
/// Space: O(n)
pub struct SegmentTree<S: Monoid>(Box<[S]>);

impl<S: Monoid> SegmentTree<S> {
    /// Creates a new segment tree with `n` elements, where all initialized to `S::id()`.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn new(n: usize) -> Self {
        debug_assert!(n > 0, "n must not be zero");
        Self(vec![S::id(); n << 1].into_boxed_slice())
    }

    /// Creates a segment tree from a vec.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_vec(mut v: Vec<S>) -> Self {
        let n = v.len();
        debug_assert!(n > 0, "n must not be zero");
        v.reserve(n);
        unsafe {
            let ptr = v.as_mut_ptr();
            ptr.copy_to(ptr.add(n), n);
            for i in (1..n).rev() {
                ptr.add(i)
                    .write(S::op(&*ptr.add(i << 1), &*ptr.add((i << 1) + 1)));
            }
            ptr.write(S::id());
            v.set_len(n << 1);
        }
        Self(v.into_boxed_slice())
    }

    /// Creates a segment tree from a slice.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_slice(v: &[S]) -> Self {
        let n = v.len();
        debug_assert!(n > 0, "n must not be zero");
        let mut data = vec![S::id(); n << 1];
        unsafe {
            let d = data.as_mut_ptr();
            std::ptr::copy_nonoverlapping(v.as_ptr(), d.add(n), n);
            for i in (1..n).rev() {
                *d.add(i) = S::op(&*d.add(i << 1), &*d.add((i << 1) + 1));
            }
        }
        Self(data.into_boxed_slice())
    }

    /// Sets the value at index `i` to `x`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn set(&mut self, mut i: usize, x: S) {
        debug_assert!(
            i < self.len(),
            "index out of bounds: i={}, len={}",
            i,
            self.len(),
        );
        i += self.len();
        unsafe {
            let d = self.0.as_mut_ptr();
            *d.add(i) = x;
            while i > 1 {
                i >>= 1;
                *d.add(i) = S::op(&*d.add(i << 1), &*d.add((i << 1) + 1));
            }
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
        i += self.len();
        unsafe {
            let d = self.0.as_mut_ptr();
            *d.add(i) = S::op(&*d.add(i), &x);
            while i > 1 {
                i >>= 1;
                *d.add(i) = S::op(&*d.add(i << 1), &*d.add((i << 1) + 1));
            }
        }
    }

    /// Returns the value at index `i`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn get(&self, i: usize) -> S {
        debug_assert!(
            i < self.len(),
            "index out of bounds: i={}, len={}",
            i,
            self.len(),
        );
        unsafe { self.0.get_unchecked(self.len() + i).clone() }
    }

    /// Returns `op(a[l], ..., a[r - 1])`. When range is empty, return `S::id()`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn range_fold(&self, range: impl std::ops::RangeBounds<usize>) -> S {
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
            return S::id();
        }
        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();

        let mut left = S::id();
        let mut right = S::id();
        unsafe {
            let d = self.0.as_ptr();
            loop {
                if l >= r {
                    left = S::op(&left, &*d.add(l));
                    l += 1;
                    l >>= l.trailing_zeros();
                } else {
                    r -= 1;
                    right = S::op(&*d.add(r), &right);
                    r >>= r.trailing_zeros();
                }
                if l == r {
                    break;
                }
            }
        }
        S::op(&left, &right)
    }

    /// Returns `op(a[0], ..., a[n - 1])`.
    ///
    /// # Complexity
    /// O(1)
    pub fn all_fold(&self) -> S {
        unsafe { self.0.get_unchecked(1).clone() }
    }

    /// Returns the number of elements.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len() >> 1
    }

    /// Returns whether the segment tree is empty.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
