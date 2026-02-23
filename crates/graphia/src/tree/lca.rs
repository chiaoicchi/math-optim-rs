use crate::csr::Csr;

/// A Lowest Common Ancestor data structure.
///
/// # Complexity
/// Space: O(n)
pub struct Lca {
    depth: Box<[u32]>,
    tour: Box<[u32]>,
    table: Box<[u32]>,
    tin: Box<[u32]>,
}

impl Lca {
    /// Creates a new LCA from CSR.
    ///
    /// # Complexity
    /// Time: O(n log n)
    pub fn from_csr<W: Copy>(root: usize, tree: &Csr<W>) -> Self {
        let n = tree.num_vertices();
        debug_assert!(n > 0, "n mut not be zero");
        debug_assert!(root < n, "root is out of bounds: root={}, n={}", root, n);
        let m = (n << 1) - 1;
        let mut tin = vec![!0; n];
        let mut tour: Vec<u32> = Vec::with_capacity(m);
        let mut depth: Vec<u32> = Vec::with_capacity(m);
        let mut p = 0;
        let mut stack = vec![root];
        unsafe {
            let tin = tin.as_mut_ptr();
            let t = tour.as_mut_ptr();
            let d = depth.as_mut_ptr();
            while let Some(u) = stack.pop() {
                if u >> (usize::BITS - 1) == 0 {
                    *tin.add(u) = p;
                    *t.add(p as usize) = u as u32;
                    *d.add(p as usize) = if p == 0 {
                        0
                    } else {
                        *d.add(p as usize - 1) + 1
                    };
                    p += 1;
                    for &(v, _) in tree.adj(u) {
                        if *tin.add(v) == !0 {
                            stack.push(!u);
                            stack.push(v);
                        }
                    }
                } else {
                    *t.add(p as usize) = (!u) as u32;
                    *d.add(p as usize) = *d.add(*tin.add(!u) as usize);
                    p += 1;
                }
            }
            tour.set_len(p as usize);
            depth.set_len(p as usize);
        }
        let log = (usize::BITS - 1 - m.leading_zeros()) as usize;
        let mut table: Vec<u32> = (0..m as u32).collect();
        table.reserve(m * log);
        unsafe {
            let t = table.as_mut_ptr();
            let dep = depth.as_ptr();
            for k in 1..=log {
                for i in 0..m + 1 - (1 << k) {
                    let a = *t.add((k - 1) * m + i);
                    let b = *t.add((k - 1) * m + i + (1 << (k - 1)));
                    *t.add(k * m + i) = if *dep.add(a as usize) <= *dep.add(b as usize) {
                        a
                    } else {
                        b
                    };
                }
                for i in m + 1 - (1 << k)..m {
                    *t.add(k * m + i) = *t.add((k - 1) * m + i);
                }
            }
            table.set_len(m * (log + 1));
        }
        Self {
            depth: depth.into_boxed_slice(),
            tour: tour.into_boxed_slice(),
            table: table.into_boxed_slice(),
            tin: tin.into_boxed_slice(),
        }
    }

    /// Returns LCA of `u` and `v`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn lca(&self, i: usize, j: usize) -> usize {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        debug_assert!(
            j < self.len(),
            "j is out of bounds: j={}, n={}",
            j,
            self.len()
        );
        let m = (self.len() << 1) - 1;
        unsafe {
            let depth = self.depth.as_ptr();
            let table = self.table.as_ptr();
            let tin = self.tin.as_ptr();
            let mut l = *tin.add(i) as usize;
            let mut r = *tin.add(j) as usize;
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            let log = (usize::BITS - 1 - (r + 1 - l).leading_zeros()) as usize;
            let a = *table.add(log * m + l) as usize;
            let b = *table.add(log * m + r + 1 - (1 << log)) as usize;
            let argmin = if *depth.add(a) <= *depth.add(b) { a } else { b };
            *self.tour.get_unchecked(argmin) as usize
        }
    }

    /// Returns the depth of vertex `i`.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn depth(&self, i: usize) -> usize {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        unsafe {
            *self
                .depth
                .get_unchecked(*self.tin.get_unchecked(i) as usize) as usize
        }
    }

    /// Returns distance between `i` and `j`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn dist(&self, i: usize, j: usize) -> usize {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        debug_assert!(
            j < self.len(),
            "j is out of bounds: j={}, n={}",
            j,
            self.len()
        );
        self.depth(i) + self.depth(j) - 2 * self.depth(self.lca(i, j))
    }

    /// Returns the number of vertices in tree.
    ///
    /// # Complexity
    /// Time: O(1)
    #[allow(clippy::len_without_is_empty)]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.tin.len()
    }
}
