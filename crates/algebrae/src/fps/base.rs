use crate::algebra::Ring;

/// A Formal Power Series (FPS) over set of S.
///
/// # Complexity
/// Space: O(n)
#[derive(Clone)]
pub struct Fps<S: Copy + Ring>(pub(crate) Vec<S>);

impl<S: Copy + Ring> Fps<S> {
    /// Creates a new FPS where f(x) = 0.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn zero() -> Self {
        Fps(vec![])
    }

    /// Creates a new FPS where f(x) = 1.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn one() -> Self {
        Fps(vec![S::one()])
    }

    /// Creates a new FPS where f(x) = c.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn constant(c: S) -> Self {
        Fps(vec![c])
    }

    /// Creates a new FPS with capacity `n`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn with_capacity(n: usize) -> Self {
        Fps(Vec::with_capacity(n))
    }

    /// Creates a FPS from a vec.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn from_vec(coef: Vec<S>) -> Self {
        Fps(coef)
    }

    /// Creates a FPS from a slice.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_slice(coef: &[S]) -> Self {
        Fps(coef.to_vec())
    }

    /// Returns [x^k]f(x) which is the coefficient of itself.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn coef(&self, k: usize) -> S {
        if k < self.0.len() {
            unsafe { *self.0.get_unchecked(k) }
        } else {
            S::zero()
        }
    }

    /// Returns the degree of itself.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn deg(&self) -> Option<usize> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.len() - 1)
        }
    }

    /// Resize itself.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn resize(&mut self, n: usize) {
        self.0.resize(n, S::zero());
    }

    /// Computes f(x) * s where s in S.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn scale(&self, s: S) -> Self {
        let mut res: Vec<S> = Vec::with_capacity(self.0.len());
        unsafe {
            let ptr = res.as_mut_ptr();
            let lhs = self.0.as_ptr();
            for i in 0..self.0.len() {
                ptr.add(i).write(*lhs.add(i) * s);
            }
            res.set_len(self.0.len());
        }
        Fps(res)
    }

    /// Computes f(x) * s where s in S in place.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn scale_assign(&mut self, s: S) {
        unsafe {
            let ptr = self.0.as_mut_ptr();
            for i in 0..self.0.len() {
                *ptr.add(i) = *ptr.add(i) * s;
            }
        }
    }

    /// Computes f(x) * x^k.
    ///
    /// # Complexity
    /// Time: O(n + k)
    pub fn shl(&self, k: usize) -> Self {
        let mut res: Vec<S> = Vec::with_capacity(self.0.len() + k);
        unsafe {
            let ptr = res.as_mut_ptr();
            for i in 0..k {
                ptr.add(i).write(S::zero());
            }
            std::ptr::copy_nonoverlapping(self.0.as_ptr(), ptr.add(k), self.0.len());
            res.set_len(self.0.len() + k);
        }
        Fps(res)
    }

    /// Computes f(x) * x^k in place.
    ///
    /// # Complexity
    /// Time: O(n + k)
    pub fn shl_assign(&mut self, k: usize) {
        let n = self.0.len();
        self.0.reserve(k);
        unsafe {
            let ptr = self.0.as_mut_ptr();
            std::ptr::copy(ptr, ptr.add(k), n);
            for i in 0..k {
                ptr.add(i).write(S::zero());
            }
            self.0.set_len(n + k);
        }
    }
}

impl<S: Copy + Ring + PartialEq> Fps<S> {
    /// Normalize itself.
    ///
    /// # Complexity
    /// Time: O(n)
    #[inline]
    pub fn normalize(&mut self) {
        if !self.0.is_empty() {
            let mut n = self.0.len() - 1;
            let zero = S::zero();
            unsafe {
                let ptr = self.0.as_mut_ptr();
                while n > 0 && *ptr.add(n) == zero {
                    n -= 1;
                }
                if n == 0 && *ptr == zero {
                    self.0.set_len(0);
                } else {
                    self.0.set_len(n + 1);
                }
            }
        }
    }
}

impl<S: Copy + Ring> std::ops::Index<usize> for Fps<S> {
    type Output = S;
    fn index(&self, idx: usize) -> &Self::Output {
        debug_assert!(
            idx < self.0.len(),
            "index is out of bounds: idx={}, deg={:?}",
            idx,
            self.deg()
        );
        unsafe { self.0.get_unchecked(idx) }
    }
}

impl<S: Copy + Ring> std::ops::IndexMut<usize> for Fps<S> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        debug_assert!(
            idx < self.0.len(),
            "index is out of bounds: idx={}, deg={:?}",
            idx,
            self.deg()
        );
        unsafe { self.0.get_unchecked_mut(idx) }
    }
}

impl<S: Copy + Ring> std::ops::Neg for Fps<S> {
    type Output = Self;
    #[inline]
    fn neg(mut self) -> Self::Output {
        let n = self.0.len();
        unsafe {
            let ptr = self.0.as_mut_ptr();
            for i in 0..n {
                *ptr.add(i) = -(*ptr.add(i));
            }
        }
        self
    }
}

impl<S: Copy + Ring> std::ops::Add for Fps<S> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        let mut lhs = self.0;
        let mut rhs = rhs.0;
        if lhs.len() < rhs.len() {
            std::mem::swap(&mut lhs, &mut rhs);
        }
        unsafe {
            let lhs_ptr = lhs.as_mut_ptr();
            let rhs_ptr = rhs.as_ptr();
            for i in 0..rhs.len() {
                *lhs_ptr.add(i) = *lhs_ptr.add(i) + *rhs_ptr.add(i);
            }
        }
        Fps(lhs)
    }
}

impl<S: Copy + Ring> std::ops::Add<&Self> for Fps<S> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<S: Copy + Ring> std::ops::Add<Fps<S>> for &Fps<S> {
    type Output = Fps<S>;
    fn add(self, mut rhs: Fps<S>) -> Self::Output {
        rhs += self;
        rhs
    }
}

impl<S: Copy + Ring> std::ops::Add for &Fps<S> {
    type Output = Fps<S>;
    fn add(self, rhs: Self) -> Self::Output {
        let n = self.0.len();
        let m = rhs.0.len();
        let mut res: Vec<S> = Vec::with_capacity(n.max(m));
        unsafe {
            let res_ptr = res.as_mut_ptr();
            let lhs_ptr = self.0.as_ptr();
            let rhs_ptr = rhs.0.as_ptr();
            for i in 0..n.min(m) {
                res_ptr.add(i).write(*lhs_ptr.add(i) + *rhs_ptr.add(i));
            }
            if n > m {
                std::ptr::copy_nonoverlapping(lhs_ptr.add(m), res_ptr.add(m), n - m);
            } else if n < m {
                std::ptr::copy_nonoverlapping(rhs_ptr.add(n), res_ptr.add(n), m - n);
            }
            res.set_len(n.max(m));
        }
        Fps(res)
    }
}

impl<S: Copy + Ring> std::ops::Sub for Fps<S> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        let mut lhs = self.0;
        let mut rhs = rhs.0;
        if lhs.len() >= rhs.len() {
            unsafe {
                let lhs_ptr = lhs.as_mut_ptr();
                let rhs_ptr = rhs.as_ptr();
                for i in 0..rhs.len() {
                    *lhs_ptr.add(i) = *lhs_ptr.add(i) - *rhs_ptr.add(i);
                }
            }
            Fps(lhs)
        } else {
            std::mem::swap(&mut lhs, &mut rhs);
            unsafe {
                let lhs_ptr = lhs.as_mut_ptr();
                let rhs_ptr = rhs.as_ptr();
                for i in 0..rhs.len() {
                    *lhs_ptr.add(i) = *rhs_ptr.add(i) - *lhs_ptr.add(i);
                }
                for i in rhs.len()..lhs.len() {
                    *lhs_ptr.add(i) = -*lhs_ptr.add(i);
                }
            }
            Fps(lhs)
        }
    }
}

impl<S: Copy + Ring> std::ops::Sub<&Self> for Fps<S> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<S: Copy + Ring> std::ops::Sub<Fps<S>> for &Fps<S> {
    type Output = Fps<S>;
    fn sub(self, mut rhs: Fps<S>) -> Self::Output {
        let n = self.0.len();
        let m = rhs.0.len();
        unsafe {
            let lhs_ptr = self.0.as_ptr();
            let rhs_ptr = rhs.0.as_mut_ptr();
            for i in 0..n.min(m) {
                *rhs_ptr.add(i) = *lhs_ptr.add(i) - *rhs_ptr.add(i);
            }
        }
        if n > m {
            rhs.0.reserve(n - m);
            unsafe {
                let lhs_ptr = self.0.as_ptr();
                let rhs_ptr = rhs.0.as_mut_ptr();
                std::ptr::copy_nonoverlapping(lhs_ptr.add(m), rhs_ptr.add(m), n - m);
                rhs.0.set_len(n);
            }
        } else if n < m {
            unsafe {
                let rhs_ptr = rhs.0.as_mut_ptr();
                for i in n..m {
                    *rhs_ptr.add(i) = -*rhs_ptr.add(i);
                }
            }
        }
        rhs
    }
}

impl<S: Copy + Ring> std::ops::Sub for &Fps<S> {
    type Output = Fps<S>;
    fn sub(self, rhs: Self) -> Self::Output {
        let n = self.0.len();
        let m = rhs.0.len();
        let mut res: Vec<S> = Vec::with_capacity(n.max(m));
        unsafe {
            let res_ptr = res.as_mut_ptr();
            let lhs_ptr = self.0.as_ptr();
            let rhs_ptr = rhs.0.as_ptr();
            for i in 0..n.min(m) {
                res_ptr.add(i).write(*lhs_ptr.add(i) - *rhs_ptr.add(i));
            }
            if n > m {
                std::ptr::copy_nonoverlapping(lhs_ptr.add(m), res_ptr.add(m), n - m);
            } else if n < m {
                for i in n..m {
                    res_ptr.add(i).write(-*rhs_ptr.add(i));
                }
            }
            res.set_len(n.max(m));
        }
        Fps(res)
    }
}

impl<S: Copy + Ring> std::ops::AddAssign<&Self> for Fps<S> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        let n = self.0.len();
        let m = rhs.0.len();
        unsafe {
            let lhs_ptr = self.0.as_mut_ptr();
            let rhs_ptr = rhs.0.as_ptr();
            for i in 0..n.min(m) {
                *lhs_ptr.add(i) = *lhs_ptr.add(i) + *rhs_ptr.add(i);
            }
        }
        if n < m {
            self.0.reserve(m - n);
            unsafe {
                let lhs_ptr = self.0.as_mut_ptr();
                let rhs_ptr = rhs.0.as_ptr();
                std::ptr::copy_nonoverlapping(rhs_ptr.add(n), lhs_ptr.add(n), m - n);
                self.0.set_len(m);
            }
        }
    }
}

impl<S: Copy + Ring> std::ops::SubAssign<&Self> for Fps<S> {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        let n = self.0.len();
        let m = rhs.0.len();
        unsafe {
            let lhs_ptr = self.0.as_mut_ptr();
            let rhs_ptr = rhs.0.as_ptr();
            for i in 0..n.min(m) {
                *lhs_ptr.add(i) = *lhs_ptr.add(i) - *rhs_ptr.add(i);
            }
        }
        if n < m {
            self.0.reserve(m - n);
            unsafe {
                let lhs_ptr = self.0.as_mut_ptr();
                let rhs_ptr = rhs.0.as_ptr();
                for i in n..m {
                    lhs_ptr.add(i).write(-*rhs_ptr.add(i));
                }
                self.0.set_len(m);
            }
        }
    }
}

macro_rules! forward_ref_op_assign {
    ($($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl<S: Copy + Ring> std::ops::$trait for Fps<S> {
                #[inline]
                fn $method(&mut self, rhs: Self) {
                    std::ops::$trait::$method(self, &rhs)
                }
            }
        )*
    };
}

forward_ref_op_assign! {
    AddAssign, add_assign;
    SubAssign, sub_assign;
}
