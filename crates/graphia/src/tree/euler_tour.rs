use crate::csr::Csr;

/// A Euler tour structure.
///
/// # Complexity
/// Space: O(n)
pub struct EulerTour {
    tin: Box<[usize]>,
    tout: Box<[usize]>,
}

impl EulerTour {
    /// Creates a Euler tour from CSR.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_csr<W: Copy>(root: usize, tree: &Csr<W>) -> Self {
        let n = tree.num_vertices();
        debug_assert!(root < n, "root is out of bounds: root={}, n={}", root, n);
        let mut tin = vec![!0; n];
        let mut tout = vec![!0; n];
        let mut t = 0;
        let mut stack = vec![root];
        unsafe {
            let tin = tin.as_mut_ptr();
            let tout = tout.as_mut_ptr();
            while let Some(u) = stack.pop() {
                if *tin.add(u) == !0 {
                    *tin.add(u) = t;
                    t += 1;
                    stack.push(u);
                    for &(v, _) in tree.adj(u) {
                        if *tin.add(v) == !0 {
                            stack.push(v);
                        }
                    }
                } else {
                    *tout.add(u) = t;
                }
            }
        }
        Self {
            tin: tin.into_boxed_slice(),
            tout: tout.into_boxed_slice(),
        }
    }

    /// Returns the discovery time of vertex `i`.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn tin(&self, i: usize) -> usize {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        unsafe { *self.tin.get_unchecked(i) }
    }

    /// Returns the exclusive end of the subtree interval of vertex `i`.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn tout(&self, i: usize) -> usize {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        unsafe { *self.tout.get_unchecked(i) }
    }

    /// Returns the subtree interval of vertex `i`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn subtree(&self, i: usize) -> std::ops::Range<usize> {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        self.tin(i)..self.tout(i)
    }

    /// Returns the size of subtree of vertex `i`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn subtree_size(&self, i: usize) -> usize {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        self.tout(i) - self.tin(i)
    }

    /// Returns whether vertex `i` is ancestor of vertex `j`. When `i == j`, return `true`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn is_ancestor(&self, i: usize, j: usize) -> bool {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        self.tin(i) <= self.tin(j) && self.tin(j) < self.tout(i)
    }

    /// Returns the DFS pre-order of vertices.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn order(&self) -> Box<[usize]> {
        let n = self.len();
        let mut order = vec![0; n];
        unsafe {
            let order = order.as_mut_ptr();
            for i in 0..n {
                *order.add(self.tin(i)) = i;
            }
        }
        order.into_boxed_slice()
    }

    /// Returns the number of vertices in tree.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.tin.len()
    }
}
