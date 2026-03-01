use crate::{algebra::Field, linear::Matrix};

impl<T: PartialEq + Field> Matrix<T> {
    /// Calculates determinant of matrix.
    ///
    /// # Complexity
    /// Time: O(n^3)
    pub fn det(&self) -> T {
        debug_assert!(self.is_square(), "matrix must be square");
        let n = self.h();
        let mut data = self.data.clone();
        let mut res = T::one();
        unsafe {
            let ptr = data.as_mut_ptr();
            for i in (0..n).rev() {
                let mut pivot = n;
                for k in (0..=i).rev() {
                    if *ptr.add(k * n + i) != T::zero() {
                        pivot = k;
                        break;
                    }
                }
                if pivot == n {
                    return T::zero();
                }

                if pivot != i {
                    res = -res;
                    for j in 0..=i {
                        std::ptr::swap(ptr.add(i * n + j), ptr.add(pivot * n + j));
                    }
                }

                let diag = *ptr.add(i * n + i);
                res = res * diag;
                let inv = T::one() / diag;
                for j in 0..i {
                    *ptr.add(i * n + j) = *ptr.add(i * n + j) * inv;
                }
                for t in 0..i {
                    let p = *ptr.add(t * n + i);
                    for j in 0..i {
                        *ptr.add(t * n + j) = *ptr.add(t * n + j) - p * *ptr.add(i * n + j);
                    }
                }
            }
        }
        res
    }

    /// Calculates rank of matrix.
    ///
    /// # Complexity
    /// Time: O(hw min(h, w))
    pub fn rank(&self) -> usize {
        let mut a = self.clone();
        a.row_reduce()
    }

    /// Calculates inverse matrix.
    ///
    /// # Complexity
    /// Time: O(n^3)
    pub fn inverse(&self) -> Option<Self> {
        debug_assert!(self.is_square(), "matrix must be square");
        let n = self.h();
        let w = n << 1;
        let mut data = vec![T::zero(); n * w];
        unsafe {
            let dst = data.as_mut_ptr();
            let src = self.data.as_ptr();
            for i in 0..n {
                std::ptr::copy_nonoverlapping(src.add(i * n), dst.add(i * w), n);
                *dst.add(i * w + n + i) = T::one();
            }
        }
        let mut rank = 0;
        unsafe {
            let ptr = data.as_mut_ptr();
            for col in 0..n {
                let mut pivot = n;
                for row in rank..n {
                    if *ptr.add(row * w + col) != T::zero() {
                        pivot = row;
                        break;
                    }
                }
                if pivot == n {
                    return None;
                }
                if pivot != rank {
                    for j in col..w {
                        std::ptr::swap(ptr.add(rank * w + j), ptr.add(pivot * w + j));
                    }
                }
                let diag = *ptr.add(rank * w + col);
                let inv = T::one() / diag;
                for j in col..w {
                    *ptr.add(rank * w + j) = *ptr.add(rank * w + j) * inv;
                }
                for row in 0..n {
                    if row == rank {
                        continue;
                    }
                    let p = *ptr.add(row * w + col);
                    if p == T::zero() {
                        continue;
                    }
                    for j in col..w {
                        *ptr.add(row * w + j) = *ptr.add(row * w + j) - p * *ptr.add(rank * w + j);
                    }
                }
                rank += 1;
            }
        }
        let mut res: Vec<T> = Vec::with_capacity(n * n);
        unsafe {
            res.set_len(n * n);
            let data = data.as_ptr();
            let res = res.as_mut_ptr();
            for i in 0..n {
                std::ptr::copy_nonoverlapping(data.add(i * w + n), res.add(i * n), n);
            }
        }
        Some(Matrix::from_flat(n, n, res))
    }

    /// Reduces the matrix to row echelon form and returns the rank.
    ///
    /// # Complexity
    /// Time: O(hw min(h, w))
    pub fn row_reduce(&mut self) -> usize {
        let h = self.h();
        let w = self.w();
        let mut rank = 0;
        unsafe {
            let ptr = self.data.as_mut_ptr();
            for col in 0..w {
                let mut pivot = h;
                for row in rank..h {
                    if *ptr.add(row * w + col) != T::zero() {
                        pivot = row;
                        break;
                    }
                }
                if pivot == h {
                    continue;
                }

                if pivot != rank {
                    for j in col..w {
                        std::ptr::swap(ptr.add(rank * w + j), ptr.add(pivot * w + j));
                    }
                }

                let diag = *ptr.add(rank * w + col);
                let inv = T::one() / diag;
                for j in col + 1..w {
                    *ptr.add(rank * w + j) = *ptr.add(rank * w + j) * inv;
                }
                for row in rank + 1..h {
                    let p = *ptr.add(row * w + col);
                    for j in col + 1..w {
                        *ptr.add(row * w + j) = *ptr.add(row * w + j) - p * *ptr.add(rank * w + j);
                    }
                }
                rank += 1;
            }
        }
        rank
    }

    /// Reduces the matrix to reduced row echelon form and returns the pivot column indices.
    ///
    /// # Complexity
    /// Time: O(hw min(h, w))
    pub fn rref(&mut self) -> usize {
        let h = self.h();
        let w = self.w();
        let mut rank = 0;
        unsafe {
            let ptr = self.data.as_mut_ptr();
            for col in 0..w {
                let mut pivot = h;
                for row in rank..h {
                    if *ptr.add(row * w + col) != T::zero() {
                        pivot = row;
                        break;
                    }
                }
                if pivot == h {
                    continue;
                }

                if pivot != rank {
                    for j in col..w {
                        std::ptr::swap(ptr.add(rank * w + j), ptr.add(pivot * w + j));
                    }
                }

                let diag = *ptr.add(rank * w + col);
                let inv = T::one() / diag;
                for j in col..w {
                    *ptr.add(rank * w + j) = *ptr.add(rank * w + j) * inv;
                }
                for row in 0..h {
                    if row == rank {
                        continue;
                    }
                    let p = *ptr.add(row * w + col);
                    if p == T::zero() {
                        continue;
                    }
                    for j in col..w {
                        *ptr.add(row * w + j) = *ptr.add(row * w + j) - p * *ptr.add(rank * w + j);
                    }
                }
                rank += 1;
            }
        }
        rank
    }
}
