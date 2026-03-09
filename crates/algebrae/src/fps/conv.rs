use crate::algebra::Ring;
use crate::fps::Fps;

/// A convolution trait.
pub trait Conv: Copy + Ring {
    fn convolve(a: Vec<Self>, b: Vec<Self>) -> Vec<Self>;
}

impl<S: Conv> std::ops::Mul for Fps<S> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Fps(S::convolve(self.0, rhs.0))
    }
}

impl<S: Conv> std::ops::Mul<&Self> for Fps<S> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        Fps(S::convolve(self.0, rhs.0.clone()))
    }
}

impl<S: Conv> std::ops::Mul<Fps<S>> for &Fps<S> {
    type Output = Fps<S>;
    fn mul(self, rhs: Fps<S>) -> Self::Output {
        Fps(S::convolve(rhs.0, self.0.clone()))
    }
}

impl<S: Conv> std::ops::Mul for &Fps<S> {
    type Output = Fps<S>;
    fn mul(self, rhs: Self) -> Self::Output {
        Fps(S::convolve(self.0.clone(), rhs.0.clone()))
    }
}

impl<S: Conv> std::ops::MulAssign for Fps<S> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let a = unsafe { std::ptr::read(&self.0) };
        let res = S::convolve(a, rhs.0);
        unsafe { std::ptr::write(&mut self.0, res) };
    }
}

impl<S: Conv> std::ops::MulAssign<&Self> for Fps<S> {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        let a = unsafe { std::ptr::read(&self.0) };
        let res = S::convolve(a, rhs.0.clone());
        unsafe { std::ptr::write(&mut self.0, res) };
    }
}

impl<S: Conv + PartialEq + std::ops::Div<Output = S>> Fps<S> {
    /// Computes inverse of Formal Power Series (FPS).
    ///
    /// # Complexity
    /// Time: O(n log n)
    pub fn inv(&self, n: usize) -> Self {
        debug_assert!(self.deg().is_some(), "Fps must not be zero");
        debug_assert!(self[0] != S::zero(), "a_0 must not be zero");
        let mut res = Fps::constant(S::one() / self[0]);
        let mut m = 1;
        let two = S::one() + S::one();
        while m < n {
            m <<= 1;
            let mut f = Fps::from_slice(&self.0[..m.min(n)]) * &res;
            f.resize(m);
            f = Fps::constant(two) - f;
            res *= f;
            res.resize(m.min(n));
        }
        res
    }
}
