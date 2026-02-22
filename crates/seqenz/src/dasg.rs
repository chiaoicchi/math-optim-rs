/// A directed acyclic subsequence graph (subsequence automaton) data structure.
///
/// # Complexity
/// Space: O(nA)
pub struct Dasg<const A: usize> {
    data: Box<[[u32; A]]>,
    dp: std::cell::OnceCell<Box<[usize]>>,
}

impl<const A: usize> Dasg<A> {
    /// Creates a new DASG from a slice.
    ///
    /// # Complexity
    /// Time: O(nA)
    pub fn from_slice(s: &[usize]) -> Self {
        let n = s.len();
        let mut data = vec![[!0; A]; n + 1];
        unsafe {
            let d = data.as_mut_ptr() as *mut u32;
            let s = s.as_ptr();
            for i in (0..n).rev() {
                let c = *s.add(i);
                debug_assert!(c < A, "symbol out of bounds: s[{}]={}, A={}", i, c, A);
                let src = d.add((i + 1) * A);
                let dst = d.add(i * A);
                std::ptr::copy_nonoverlapping(src, dst, A);
                *dst.add(c) = i as u32 + 1;
            }
        }
        Self {
            data: data.into_boxed_slice(),
            dp: std::cell::OnceCell::new(),
        }
    }

    /// Returns whether `t` is a subsequence of the original string.
    ///
    /// # Complexity
    /// Time: O(|t|)
    pub fn contains(&self, t: &[usize]) -> bool {
        let mut state = 0;
        unsafe {
            let d = self.data.as_ptr() as *const u32;
            for &c in t {
                let next = *d.add(state * A + c);
                if next == !0 {
                    return false;
                }
                state = next as usize;
            }
        }
        true
    }

    /// Returns the number of distinct subsequences (including the empty string).
    ///
    /// # Complexity
    /// Time: initial O(nA), after O(1)
    pub fn count(&self) -> usize {
        unsafe { *self.dp().get_unchecked(0) }
    }

    /// Returns the number of distinct subsequences of length `k`.
    ///
    /// # Complexity
    /// Time: O(nkA)
    pub fn count_len(&self, k: usize) -> usize {
        let n = self.len();
        if n < k {
            return 0;
        }
        let mut prev = vec![1; n + 1];
        let mut curr = vec![0; n + 1];
        for _ in 0..k {
            for (curr, row) in curr.iter_mut().zip(self.data.iter()) {
                let mut sum = 0usize;
                for &next in row.iter() {
                    if next != !0 {
                        sum = sum.saturating_add(prev[next as usize]);
                    }
                }
                *curr = sum;
            }
            std::mem::swap(&mut prev, &mut curr);
            curr.fill(0);
        }
        prev[0]
    }

    /// Returns the k-th (0-indexed) subsequence in lexicographic order.
    ///
    /// # Complexity
    /// Time: initial O(nA), after O(|result| A)
    pub fn kth(&self, mut k: usize) -> Option<Vec<usize>> {
        let dp = self.dp();
        if k >= dp[0] {
            return None;
        }
        let mut res = vec![];
        let mut state = 0;
        if k == 0 {
            return Some(res);
        }
        k -= 1;
        loop {
            let row = &self.data[state];
            for (c, &next) in row.iter().enumerate() {
                if next != !0 {
                    let cnt = dp[next as usize];
                    if k < cnt {
                        res.push(c);
                        state = next as usize;
                        break;
                    }
                    k -= cnt;
                }
            }
            if k == 0 {
                break;
            }
            k -= 1;
        }
        Some(res)
    }

    /// Returns the lexicographically smallest subsequence of length `k`.
    ///
    /// # Complexity
    /// Time: O(kA)
    pub fn min_subsequence(&self, k: usize) -> Option<Vec<usize>> {
        let n = self.len();
        if n < k {
            return None;
        }
        let mut res = Vec::with_capacity(k);
        let mut state = 0;
        for i in 0..k {
            let row = &self.data[state];
            for (c, &next) in row.iter().enumerate() {
                if next < !0 && next as usize + k <= n + i + 1 {
                    res.push(c);
                    state = next as usize;
                    break;
                }
            }
        }
        Some(res)
    }

    /// Returns the lexicographically greatest subsequence of length `k`.
    ///
    /// # Complexity
    /// Time: O(kA)
    pub fn max_subsequence(&self, k: usize) -> Option<Vec<usize>> {
        let n = self.len();
        if n < k {
            return None;
        }
        let mut res = Vec::with_capacity(k);
        let mut state = 0;
        for i in 0..k {
            let row = &self.data[state];
            for (c, &next) in row.iter().enumerate().rev() {
                if next < !0 && next as usize + k <= n + i + 1 {
                    res.push(c);
                    state = next as usize;
                    break;
                }
            }
        }
        Some(res)
    }

    /// Returns the length of sequence.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len() - 1
    }

    /// Returns whether the string is empty.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn dp(&self) -> &[usize] {
        self.dp.get_or_init(|| {
            let n = self.len();
            let mut dp = vec![0; n + 1];
            dp[n] = 1;
            for (i, row) in self.data.iter().enumerate().rev() {
                let mut sum = 1usize;
                for &next in row.iter() {
                    if next != !0 {
                        sum = sum.saturating_add(dp[next as usize]);
                    }
                }
                dp[i] = sum;
            }
            dp.into_boxed_slice()
        })
    }
}
