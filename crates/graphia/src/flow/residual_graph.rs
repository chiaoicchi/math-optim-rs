/// A residual graph structure.
///
/// # Complexity
/// Space: O(n + m)
pub struct ResidualGraph<Cap> {
    n: usize,
    pub(crate) offset: Box<[u32]>,
    pub(crate) edge: Box<[(u32, u32, Cap)]>,
    csr_idx: Box<[u32]>,
    pending: Vec<(usize, usize, Cap)>,
}

impl<Cap: Copy + Default> ResidualGraph<Cap> {
    /// Creates a new empty residual graph with `n` vertices.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn new(n: usize) -> Self {
        Self {
            n,
            offset: Box::new([]),
            edge: Box::new([]),
            csr_idx: Box::new([]),
            pending: Vec::new(),
        }
    }

    /// Creates a new empty residual graph with `n` vertices and pending capacity `m`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn with_capacity(n: usize, m: usize) -> Self {
        Self {
            n,
            offset: Box::new([]),
            edge: Box::new([]),
            csr_idx: Box::new([]),
            pending: Vec::with_capacity(m),
        }
    }

    /// Creates a builded residual graph from directed edge lists.
    ///
    /// # Complexity
    /// Time: O(n + m)
    pub fn from_directed(n: usize, edges: &[(usize, usize, Cap)]) -> Self {
        let m = edges.len();

        let mut offset = vec![0; n + 1];
        let mut edge: Vec<std::mem::MaybeUninit<(u32, u32, Cap)>> = Vec::with_capacity(m << 1);
        let mut csr_idx = Vec::with_capacity(m);
        unsafe {
            let offset = offset.as_mut_ptr();
            for &(u, v, _) in edges {
                debug_assert!(u < n, "source vertex out of bounds: u={}, n={}", u, n);
                debug_assert!(v < n, "destination vertex out of bounds: v={}, n={}", v, n);
                *offset.add(u + 1) += 1;
                *offset.add(v + 1) += 1;
            }
            for i in 1..=n {
                *offset.add(i) += *offset.add(i - 1);
            }
            edge.set_len(m << 1);
            let edge = edge.as_mut_ptr() as *mut (u32, u32, Cap);
            for &(u, v, c) in edges {
                let pos_u = *offset.add(u);
                *offset.add(u) += 1;
                let pos_v = *offset.add(v);
                *offset.add(v) += 1;
                edge.add(pos_u as usize).write((v as u32, pos_v, c));
                edge.add(pos_v as usize)
                    .write((u as u32, pos_u, Cap::default()));
                csr_idx.push(pos_u);
            }
            std::ptr::copy(offset, offset.add(1), n);
            *offset = 0;
        }
        Self {
            n,
            offset: offset.into_boxed_slice(),
            edge: unsafe {
                Box::from_raw(Box::into_raw(edge.into_boxed_slice()) as *mut [(u32, u32, Cap)])
            },
            csr_idx: csr_idx.into_boxed_slice(),
            pending: Vec::new(),
        }
    }

    /// Adds an edge from `u` to `v` with given capacity and its reverse edge with zero capacity.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn add_edge(&mut self, u: usize, v: usize, cap: Cap) -> usize {
        debug_assert!(
            u < self.num_vertices(),
            "source vertex out of bounds: u={}, n={}",
            u,
            self.num_vertices()
        );
        debug_assert!(
            v < self.num_vertices(),
            "destination vertex out of bounds: v={}, n={}",
            v,
            self.num_vertices()
        );

        let idx = self.csr_idx.len() + self.pending.len();
        self.pending.push((u, v, cap));
        idx
    }

    /// Builds the CSR pending edges. Existing CSR is rebuilt.
    ///
    /// # Complexity
    /// Time: O(n + m)
    pub fn build(&mut self) {
        todo!();
    }

    /// Returns the flow on the forward edge `e`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn flow(&self, e: usize) -> Cap {
        debug_assert!(
            e < self.csr_idx.len(),
            "edge is out of bounds: v={}, num_vertices={}",
            e,
            self.csr_idx.len(),
        );

        unsafe {
            let csr_idx = self.csr_idx.as_ptr();
            let edge = self.edge.as_ptr();
            let idx = *csr_idx.add(e) as usize;
            let rev = (*edge.add(idx)).1 as usize;
            (*edge.add(rev)).2
        }
    }

    /// Returns the number of vertices.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn num_vertices(&self) -> usize {
        self.n
    }

    /// Returns the number of directed edges (including reverse edges).
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn num_edges(&self) -> usize {
        self.edge.len() + (self.pending.len() << 1)
    }
}

impl<Cap: Copy + Default + std::ops::Add<Output = Cap>> ResidualGraph<Cap> {
    /// Resets a residual graph before calculating flow.
    ///
    /// # Complexity
    /// Time: O(m)
    pub fn reset(&mut self) {
        unsafe {
            let edge = self.edge.as_mut_ptr();
            for &idx in self.csr_idx.iter() {
                let (_, rev, _) = *edge.add(idx as usize);
                (*edge.add(idx as usize)).2 =
                    (*edge.add(idx as usize)).2 + (*edge.add(rev as usize)).2;
                (*edge.add(rev as usize)).2 = Cap::default();
            }
        }
        if !self.pending.is_empty() {
            self.build();
        }
    }

    /// Returns the initial capacity on the forward edge `e`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn initial_cap(&self, e: usize) -> Cap {
        debug_assert!(
            e < self.csr_idx.len(),
            "edge is out of bounds: v={}, num_vertices={}",
            e,
            self.csr_idx.len(),
        );

        unsafe {
            let csr_idx = self.csr_idx.as_ptr();
            let edge = self.edge.as_ptr();
            let idx = *csr_idx.add(e) as usize;
            let rev = (*edge.add(idx)).1 as usize;
            (*edge.add(idx)).2 + (*edge.add(rev)).2
        }
    }
}
