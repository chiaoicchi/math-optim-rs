use crate::{algebra::Field, linear::Matrix};

/// Solves linear system ax = b. Returns a particular solutiojn and a basis of the kernel or None
/// if no solution exists.
///
/// # Complexity
/// Time: O(hw min(h, w))
pub fn linear_system<T: PartialEq + Field>(a: &Matrix<T>, b: &[T]) -> Option<(Vec<T>, Matrix<T>)> {
    debug_assert_eq!(a.h(), b.len(), "dimension mismatch");
    let h = a.h();
    let w = a.w();

    let mut aug_data: Vec<T> = Vec::with_capacity(h * (w + 1));
    unsafe {
        let a = a.data.as_ptr();
        let b = b.as_ptr();
        let aug_ptr = aug_data.as_mut_ptr();
        for i in 0..h {
            std::ptr::copy_nonoverlapping(a.add(i * w), aug_ptr.add(i * (w + 1)), w);
            aug_ptr.add(i * (w + 1) + w).write(*b.add(i));
        }
        aug_data.set_len(h * (w + 1));
    }
    let mut aug = Matrix::from_flat(h, w + 1, aug_data);

    let mut pivots = Vec::new();
    unsafe {
        let w = w + 1;
        let ptr = aug.data.as_mut_ptr();
        for col in 0..w - 1 {
            let mut pivot = h;
            for row in pivots.len()..h {
                if *ptr.add(row * w + col) != T::zero() {
                    pivot = row;
                    break;
                }
            }
            if pivot == h {
                continue;
            }

            if pivot != pivots.len() {
                for j in col..w {
                    std::ptr::swap(ptr.add(pivots.len() * w + j), ptr.add(pivot * w + j));
                }
            }

            let diag = *ptr.add(pivots.len() * w + col);
            let inv = T::one() / diag;
            for j in col..w {
                *ptr.add(pivots.len() * w + j) = *ptr.add(pivots.len() * w + j) * inv;
            }
            for row in 0..h {
                if row == pivots.len() {
                    continue;
                }
                let p = *ptr.add(row * w + col);
                if p == T::zero() {
                    continue;
                }
                for j in col..w {
                    *ptr.add(row * w + j) =
                        *ptr.add(row * w + j) - p * *ptr.add(pivots.len() * w + j);
                }
            }
            pivots.push(col);
        }
    }

    let rank = pivots.len();

    if rank > 0 && *pivots.last().unwrap() == w {
        return None;
    }

    let mut sol = vec![T::zero(); w];
    unsafe {
        let sol = sol.as_mut_ptr();
        let aug = aug.data.as_ptr();
        for (r, &col) in pivots.iter().enumerate() {
            *sol.add(col) = *aug.add(r * (w + 1) + w);
        }
    }

    let mut pivot_set = vec![false; w];
    let mut kernel_size = w;
    unsafe {
        let ptr = pivot_set.as_mut_ptr();
        for &col in &pivots {
            *ptr.add(col) = true;
            kernel_size -= 1;
        }
    }

    if kernel_size == 0 {
        return Some((sol, Matrix::from_flat(0, w, Vec::new())));
    }

    let mut kernel_data = vec![T::zero(); kernel_size * w];
    unsafe {
        let kernel_data_ptr = kernel_data.as_mut_ptr();
        let pivot_set = pivot_set.as_ptr();
        let aug = aug.data.as_ptr();
        let mut cnt = 0;
        for col in 0..w {
            if *pivot_set.add(col) {
                continue;
            }
            *kernel_data_ptr.add(cnt * w + col) = T::one();
            for (r, &pc) in pivots.iter().enumerate() {
                *kernel_data_ptr.add(cnt * w + pc) = -*aug.add(r * (w + 1) + col);
            }
            cnt += 1;
        }
    }

    Some((sol, Matrix::from_flat(kernel_size, w, kernel_data)))
}
