/// A graph represented in Compressed Sparse Row (CSR) format. For unweighted graphs, use `Csr<()>`
/// - the weight field is a ZST and incurs no memory overhead.
///
/// # Complexity
/// Spase: O(n + m)
pub struct Csr<W: Copy> {
    offset: Box<[usize]>,
    edge: Box<[(usize, W)]>,
}

impl<W: Copy> Csr<W> {
    /// Creates a directed weighted CSR from an edge slice.
    ///
    /// # Complexity
    /// Time: O(n + m)
    pub fn from_directed_weighted(n: usize, edges: &[(usize, usize, W)]) -> Self {
        let m = edges.len();

        let mut offset = vec![0; n + 1];
        let mut edge: Vec<std::mem::MaybeUninit<(usize, W)>> = Vec::with_capacity(m);
        unsafe {
            let offset = offset.as_mut_ptr();
            for &(u, _, _) in edges {
                debug_assert!(u < n, "source vertex out of bounds: u={}, n={}", u, n);
                *offset.add(u + 1) += 1;
            }
            for i in 1..=n {
                *offset.add(i) += *offset.add(i - 1);
            }
            edge.set_len(m);
            let edge = edge.as_mut_ptr() as *mut (usize, W);
            for &(u, v, w) in edges {
                debug_assert!(v < n, "destination vertex out of bounds: v={}, n={}", v, n);
                let pos = *offset.add(u);
                edge.add(pos).write((v, w));
                *offset.add(u) += 1;
            }
            std::ptr::copy(offset, offset.add(1), n);
            *offset = 0;
        }

        Self {
            offset: offset.into_boxed_slice(),
            edge: unsafe {
                Box::from_raw(Box::into_raw(edge.into_boxed_slice()) as *mut [(usize, W)])
            },
        }
    }

    /// Creates a directed weighted CSR from an edge slice.
    ///
    /// # Complexity
    /// Time: O(n + m)
    pub fn from_undirected_weighted(n: usize, edges: &[(usize, usize, W)]) -> Self {
        let m = edges.len();

        let mut offset = vec![0; n + 1];
        let mut edge: Vec<std::mem::MaybeUninit<(usize, W)>> = Vec::with_capacity(2 * m);
        unsafe {
            let offset = offset.as_mut_ptr();
            for &(u, v, _) in edges {
                debug_assert!(u < n, "vertex out of bounds: u={}, n={}", u, n);
                debug_assert!(v < n, "vertex out of bounds: v={}, n={}", v, n);
                *offset.add(u + 1) += 1;
                *offset.add(v + 1) += 1;
            }
            for i in 1..=n {
                *offset.add(i) += *offset.add(i - 1);
            }
            edge.set_len(2 * m);
            let edge = edge.as_mut_ptr() as *mut (usize, W);
            for &(u, v, w) in edges {
                let pos = *offset.add(u);
                edge.add(pos).write((v, w));
                *offset.add(u) += 1;
                let pos = *offset.add(v);
                edge.add(pos).write((u, w));
                *offset.add(v) += 1;
            }
            std::ptr::copy(offset, offset.add(1), n);
            *offset = 0;
        }

        Self {
            offset: offset.into_boxed_slice(),
            edge: unsafe {
                Box::from_raw(Box::into_raw(edge.into_boxed_slice()) as *mut [(usize, W)])
            },
        }
    }

    /// Returns the out-degree of vertex `v`.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn deg(&self, v: usize) -> usize {
        debug_assert!(
            v < self.num_vertices(),
            "vertex out of bounds: v={}, num_vertices={}",
            v,
            self.num_vertices(),
        );
        unsafe {
            let offset = self.offset.as_ptr();
            *offset.add(v + 1) - *offset.add(v)
        }
    }

    /// Returns a slice of the adjacency list for vertex `v`.
    ///
    /// # Complexity
    /// Time: O(deg(v))
    pub fn adj(&self, v: usize) -> &[(usize, W)] {
        debug_assert!(
            v < self.num_vertices(),
            "vertex out of bounds: v={}, num_vertices={}",
            v,
            self.num_vertices(),
        );
        unsafe {
            let offset = self.offset.as_ptr();
            let start = *offset.add(v);
            let len = *offset.add(v + 1) - start;
            std::slice::from_raw_parts(self.edge.as_ptr().add(start), len)
        }
    }

    /// Constructs the reverse graph, where every edge `(u, v, w)` becomes `(v, u, w)`.
    ///
    /// # Time complexity
    ///
    /// Time: O(n + m)
    pub fn reverse(&self) -> Self {
        let n = self.num_vertices();
        let m = self.num_edges();

        let mut offset = vec![0; n + 1];
        let mut edge: Vec<std::mem::MaybeUninit<(usize, W)>> = Vec::with_capacity(m);
        unsafe {
            let offset = offset.as_mut_ptr();
            for &(v, _) in self.edge.iter() {
                *offset.add(v + 1) += 1;
            }
            for i in 1..=n {
                *offset.add(i) += *offset.add(i - 1);
            }
            edge.set_len(m);
            let edge = edge.as_mut_ptr() as *mut (usize, W);
            for u in 0..n {
                for &(v, w) in self.adj(u) {
                    let pos = *offset.add(v);
                    edge.add(pos).write((u, w));
                    *offset.add(v) += 1;
                }
            }
            std::ptr::copy(offset, offset.add(1), n);
            *offset = 0;
        }

        Self {
            offset: offset.into_boxed_slice(),
            edge: unsafe {
                Box::from_raw(Box::into_raw(edge.into_boxed_slice()) as *mut [(usize, W)])
            },
        }
    }

    /// Returns the number of vertices.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn num_vertices(&self) -> usize {
        self.offset.len() - 1
    }

    /// Returns the number of directed edges. Each undirected edge is counted as 2.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn num_edges(&self) -> usize {
        self.edge.len()
    }
}

impl Csr<()> {
    /// Creates a directed unweighted CSR from an edge slice.
    ///
    /// # Complexity
    /// Time: O(n + m)
    pub fn from_directed_unweighted(n: usize, edges: &[(usize, usize)]) -> Self {
        let m = edges.len();

        let mut offset = vec![0; n + 1];
        let mut edge: Vec<std::mem::MaybeUninit<(usize, ())>> = Vec::with_capacity(m);
        unsafe {
            let offset = offset.as_mut_ptr();
            for &(u, _) in edges {
                debug_assert!(u < n, "source vertex out of bounds: u={}, n={}", u, n);
                *offset.add(u + 1) += 1;
            }
            for i in 1..=n {
                *offset.add(i) += *offset.add(i - 1);
            }
            edge.set_len(m);
            let edge = edge.as_mut_ptr() as *mut (usize, ());
            for &(u, v) in edges {
                debug_assert!(v < n, "destination vertex out of bounds: v={}, n={}", v, n);
                let pos = *offset.add(u);
                edge.add(pos).write((v, ()));
                *offset.add(u) += 1;
            }
            std::ptr::copy(offset, offset.add(1), n);
            *offset = 0;
        }

        Self {
            offset: offset.into_boxed_slice(),
            edge: unsafe {
                Box::from_raw(Box::into_raw(edge.into_boxed_slice()) as *mut [(usize, ())])
            },
        }
    }
    /// Creates a undirected unweighted CSR from an edge slice.
    ///
    /// # Complexity
    /// Time: O(n + m)
    pub fn from_undirected_unweighted(n: usize, edges: &[(usize, usize)]) -> Self {
        let m = edges.len();

        let mut offset = vec![0; n + 1];
        let mut edge: Vec<std::mem::MaybeUninit<(usize, ())>> = Vec::with_capacity(2 * m);
        unsafe {
            let offset = offset.as_mut_ptr();
            for &(u, v) in edges {
                debug_assert!(u < n, "vertex out of bounds: u={}, n={}", u, n);
                debug_assert!(v < n, "vertex out of bounds: v={}, n={}", v, n);
                *offset.add(u + 1) += 1;
                *offset.add(v + 1) += 1;
            }
            for i in 1..=n {
                *offset.add(i) += *offset.add(i - 1);
            }
            edge.set_len(2 * m);
            let edge = edge.as_mut_ptr() as *mut (usize, ());
            for &(u, v) in edges {
                let pos = *offset.add(u);
                edge.add(pos).write((v, ()));
                *offset.add(u) += 1;
                let pos = *offset.add(v);
                edge.add(pos).write((u, ()));
                *offset.add(v) += 1;
            }
            std::ptr::copy(offset, offset.add(1), n);
            *offset = 0;
        }

        Self {
            offset: offset.into_boxed_slice(),
            edge: unsafe {
                Box::from_raw(Box::into_raw(edge.into_boxed_slice()) as *mut [(usize, ())])
            },
        }
    }
}
