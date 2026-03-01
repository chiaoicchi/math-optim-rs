use algebrae::algebra::{Action, Monoid};

/// A lazy segment tree structure.
///
/// # Complexity
/// Space: O(n)
pub struct LazySegmentTree<S: Monoid, F: Monoid + Action<S>> {
    data: Box<[S]>,
    lazy: Box<[F]>,
}

impl<S: Monoid, F: Monoid + Action<S>> LazySegmentTree<S, F> {
    /// Creates a new lazy segment tree with `n` elements, where all initialized to `S::id()`.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn new(n: usize) -> Self {
        debug_assert!(n > 0, "n must not be zero");
        Self {
            data: vec![S::id(); n << 1].into_boxed_slice(),
            lazy: vec![F::id(); n].into_boxed_slice(),
        }
    }

    /// Creates a lazy segment tree from a vec.
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
        Self {
            data: v.into_boxed_slice(),
            lazy: vec![F::id(); n].into_boxed_slice(),
        }
    }

    /// Creates a lazy segment tree from a slice.
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
        Self {
            data: data.into_boxed_slice(),
            lazy: vec![F::id(); n].into_boxed_slice(),
        }
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
        self.propagate(i);
        unsafe {
            *self.data.get_unchecked_mut(i) = x;
        }
        self.update(i);
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
        self.propagate(i);
        unsafe {
            let data = self.data.as_mut_ptr();
            *data.add(i) = S::op(&*data.add(i), &x);
        }
        self.update(i);
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
        self.propagate(i);
        unsafe {
            let data = self.data.as_mut_ptr();
            *data.add(i) = f.act(&*data.add(i));
        }
        self.update(i);
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

        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();

        self.propagate(l);
        self.propagate(r - 1);

        {
            let (mut l, mut r) = (l, r);
            unsafe {
                let data = self.data.as_mut_ptr();
                let lazy = self.lazy.as_mut_ptr();
                loop {
                    if l >= r {
                        *data.add(l) = f.act(&*data.add(l));
                        if l < self.len() {
                            *lazy.add(l) = F::op(&f, &*lazy.add(l));
                        }
                        l += 1;
                        l >>= l.trailing_zeros();
                    } else {
                        r -= 1;
                        *data.add(r) = f.act(&*data.add(r));
                        if r < self.len() {
                            *lazy.add(r) = F::op(&f, &*lazy.add(r));
                        }
                        r >>= r.trailing_zeros();
                    }
                    if l == r {
                        break;
                    }
                }
            }
        }

        self.update(l);
        self.update(r - 1);
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
        i += self.len();
        self.propagate(i);
        unsafe { self.data.get_unchecked(i).clone() }
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
        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();

        let mut left = S::id();
        let mut right = S::id();

        unsafe {
            let data = self.data.as_ptr();
            let lazy = self.lazy.as_ptr();
            loop {
                if l >= r {
                    let mut i = l >> 1;
                    left = S::op(&left, &*data.add(l));
                    l += 1;
                    l >>= l.trailing_zeros();
                    while i > l >> 1 {
                        left = (*lazy.add(i)).act(&left);
                        i >>= 1;
                    }
                } else {
                    let mut i = r >> 1;
                    r -= 1;
                    right = S::op(&*data.add(r), &right);
                    r >>= r.trailing_zeros();
                    while i > r >> 1 {
                        right = (*lazy.add(i)).act(&right);
                        i >>= 1;
                    }
                }
                if l == r {
                    break;
                }
            }
        }
        let mut res = S::op(&left, &right);
        let mut i = l >> 1;
        unsafe {
            let lazy = self.lazy.as_ptr();
            while i > 0 {
                res = (*lazy.add(i)).act(&res);
                i >>= 1;
            }
        }
        res
    }

    /// Returns `op(a[0], ..., a[n - 1])`.
    ///
    /// # Complexity
    /// O(n log n)
    pub fn all_fold(&self) -> S {
        self.range_fold(..)
    }

    /// Returns the number of elements.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len() >> 1
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
        let n = self.len();
        let data = self.data.as_mut_ptr();
        let lazy = self.lazy.as_mut_ptr();
        unsafe {
            for t in (1..(usize::BITS - i.leading_zeros()) as usize).rev() {
                let k = i >> t;
                let f = std::ptr::replace(lazy.add(k), F::id());
                *data.add(k << 1) = f.act(&*data.add(k << 1));
                *data.add((k << 1) + 1) = f.act(&*data.add((k << 1) + 1));
                if k << 1 < n {
                    *lazy.add(k << 1) = F::op(&f, &*lazy.add(k << 1));
                    *lazy.add((k << 1) + 1) = F::op(&f, &*lazy.add((k << 1) + 1));
                }
            }
        }
    }

    #[inline(always)]
    fn update(&mut self, mut i: usize) {
        let data = self.data.as_mut_ptr();
        unsafe {
            while i > 1 {
                i >>= 1;
                *data.add(i) = S::op(&*data.add(i << 1), &*data.add((i << 1) + 1));
            }
        }
    }
}
