use crate::algebra::Rig;

/// A matrix data structure.
///
/// # Complexity
/// Space: O(nm)
#[derive(Clone)]
pub struct Matrix<T: Copy> {
    h: usize,
    w: usize,
    pub(crate) data: Box<[T]>,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix from a vec.
    ///
    /// # Complexity
    /// Time: O(nm)
    pub fn from_vec(a: Vec<Vec<T>>) -> Self {
        debug_assert!(!a.is_empty(), "a is empty");
        debug_assert!(!a[0].is_empty(), "a is empty");
        debug_assert!(
            a.iter().all(|ai| ai.len() == a[0].len()),
            "a is not matrix shape"
        );
        let h = a.len();
        let w = a[0].len();
        Self {
            h,
            w,
            data: a.into_iter().flatten().collect(),
        }
    }

    /// Creates a new matrix from a slice.
    ///
    /// # Complexity
    /// Time: O(nm)
    pub fn from_slice(a: &[Vec<T>]) -> Self {
        debug_assert!(!a.is_empty(), "a is empty");
        debug_assert!(!a[0].is_empty(), "a is empty");
        debug_assert!(
            a.iter().all(|ai| ai.len() == a[0].len()),
            "a is not matrix shape"
        );
        let h = a.len();
        let w = a[0].len();
        let mut data = Vec::with_capacity(h * w);
        for row in a {
            data.extend_from_slice(row);
        }
        Self {
            h,
            w,
            data: data.into_boxed_slice(),
        }
    }

    /// Creates a new matrix from a flat vec.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn from_flat(h: usize, w: usize, data: Vec<T>) -> Self {
        debug_assert_eq!(data.len(), h * w, "data length mismatch");
        Self {
            h,
            w,
            data: data.into_boxed_slice(),
        }
    }

    /// Returns number of row.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn h(&self) -> usize {
        self.h
    }

    /// Returns number of column.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn w(&self) -> usize {
        self.w
    }

    /// Iterates each row from a matrix.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks_exact(self.w())
    }

    /// Iterates each row with mutable from a matrix.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_exact_mut(self.w())
    }

    /// Returns whether the matrix is square.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn is_square(&self) -> bool {
        self.h() == self.w()
    }
}

impl<T: Rig> Matrix<T> {
    /// Creates a zero matrix.
    ///
    /// # Complexity
    /// Time: O(nm)
    pub fn zero(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![T::zero(); h * w].into_boxed_slice(),
        }
    }

    /// Creates a unit matrix.
    ///
    /// # Complexity
    /// Time: O(nm)
    #[inline(always)]
    pub fn id(n: usize) -> Self {
        let mut data = vec![T::zero(); n * n];
        unsafe {
            let data = data.as_mut_ptr();
            for i in 0..n {
                *data.add((n + 1) * i) = T::one();
            }
        }
        Self {
            h: n,
            w: n,
            data: data.into_boxed_slice(),
        }
    }

    /// Computes pow of matrix.
    ///
    /// # Complexity
    /// Time: O(n^3 log exp)
    pub fn pow(&self, mut exp: u64) -> Self {
        debug_assert!(self.is_square(), "Matrix must be square");
        let n = self.h();
        let mut base = self.clone();
        let mut res = Self::id(n);
        while exp > 0 {
            if exp & 1 == 1 {
                res = res * base.clone();
            }
            base = base.clone() * base;
            exp >>= 1;
        }
        res
    }
}

impl<T: Copy> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.w()..(index + 1) * self.w()]
    }
}

impl<T: Copy> std::ops::IndexMut<usize> for Matrix<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.w..(index + 1) * self.w]
    }
}

impl<T: Copy + std::ops::Neg<Output = T>> std::ops::Neg for Matrix<T> {
    type Output = Self;
    #[inline]
    fn neg(mut self) -> Self::Output {
        for v in self.data.iter_mut() {
            *v = -*v;
        }
        self
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for Matrix<T> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs + *rhs;
        }
        self
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for Matrix<T> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs - *rhs;
        }
        self
    }
}

impl<T: Rig> std::ops::Mul for Matrix<T> {
    type Output = Self;
    /// Computes multiple of matrices.
    ///
    /// # Complexity
    /// Time: O(hwd)
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        debug_assert_eq!(self.w, rhs.h, "Shape is mismatch");
        let h = self.h();
        let w = rhs.w();
        let d = self.w();
        let mut rhs_transpose: Vec<std::mem::MaybeUninit<T>> = Vec::with_capacity(d * w);
        let mut res: Vec<std::mem::MaybeUninit<T>> = Vec::with_capacity(h * w);
        unsafe {
            rhs_transpose.set_len(d * w);
            let rhs_transpose = rhs_transpose.as_mut_ptr() as *mut T;
            let rhs = rhs.data.as_ptr();
            for j in 0..d {
                for k in 0..w {
                    rhs_transpose.add(k * d + j).write(*rhs.add(j * w + k));
                }
            }
            res.set_len(h * w);
            let res = res.as_mut_ptr() as *mut T;
            let lhs = self.data.as_ptr();
            for i in 0..h {
                for k in 0..w {
                    let mut x = T::zero();
                    for j in 0..d {
                        x = x + *lhs.add(i * d + j) * *rhs_transpose.add(k * d + j);
                    }
                    res.add(i * w + k).write(x);
                }
            }
        }
        Self {
            h,
            w,
            data: unsafe { Box::from_raw(Box::into_raw(res.into_boxed_slice()) as *mut [T]) },
        }
    }
}
