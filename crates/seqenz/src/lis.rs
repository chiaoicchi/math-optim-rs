/// Computes Longest Incresing Subsequence (LIS) length.
///
/// # Complexity
/// Time: O(n log n), Space: O(n)
#[allow(clippy::op_ref)]
pub fn lis_len<T: PartialOrd>(a: &[T]) -> usize {
    let n = a.len();
    let mut dp = Vec::new();
    unsafe {
        let a = a.as_ptr();
        for i in 0..n {
            let pos = dp.partition_point(|&j| &*a.add(j) < &*a.add(i));
            if pos == dp.len() {
                dp.push(i);
            } else {
                *dp.get_unchecked_mut(pos) = i;
            }
        }
    }
    dp.len()
}

/// Computes Longest Incresing Subsequence (LIS) and returns its indices.
///
/// # Complexity
/// Time: O(n log n), Space: O(n)
#[allow(clippy::op_ref)]
pub fn lis<T: PartialOrd>(a: &[T]) -> Vec<usize> {
    let n = a.len();
    let mut dp = Vec::new();
    let mut prev: Vec<std::mem::MaybeUninit<usize>> = Vec::with_capacity(n);
    unsafe {
        let a = a.as_ptr();
        prev.set_len(n);
        let prev_ptr = prev.as_mut_ptr() as *mut usize;
        for i in 0..n {
            let pos = dp.partition_point(|&j| &*a.add(j) < &*a.add(i));
            if pos == dp.len() {
                dp.push(i);
            } else {
                *dp.get_unchecked_mut(pos) = i;
            }
            prev_ptr.add(i).write(if pos > 0 {
                *dp.get_unchecked(pos - 1)
            } else {
                !0
            });
        }
    }
    let len = dp.len();
    let mut res: Vec<std::mem::MaybeUninit<usize>> = Vec::with_capacity(len);
    unsafe {
        res.set_len(len);
        let res_ptr = res.as_mut_ptr() as *mut usize;
        let prev_ptr = prev.as_mut_ptr() as *mut usize;
        let mut cur = *dp.get_unchecked(dp.len() - 1);
        for k in (0..len).rev() {
            *res_ptr.add(k) = cur;
            cur = *prev_ptr.add(cur);
        }
        std::mem::transmute::<Vec<std::mem::MaybeUninit<usize>>, Vec<usize>>(res)
    }
}
