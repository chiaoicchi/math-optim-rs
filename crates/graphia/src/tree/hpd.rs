use crate::csr::Csr;

/// A Heavy Path Decomposition data structure.
///
/// # Complexity
/// Space: O(n)
pub struct Hpd {
    pos: Box<[u32]>,
    order: Box<[u32]>,
    parent: Box<[u32]>,
    depth: Box<[u32]>,
    head: Box<[u32]>,
    size: Box<[u32]>,
}

impl Hpd {
    /// Creates a Heavy Path Decomposition from CSR.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn from_csr<W: Copy>(root: usize, tree: &Csr<W>) -> Self {
        let n = tree.num_vertices();
        debug_assert!(root < n, "root is out of bounds: root={}, n={}", root, n);

        let mut parent: Vec<u32> = Vec::with_capacity(n);
        let mut depth: Vec<u32> = Vec::with_capacity(n);
        let mut vec_queue: Vec<u32> = Vec::with_capacity(n);
        vec_queue.push(root as u32);
        let mut t = 0;
        unsafe {
            let parent_ptr = parent.as_mut_ptr();
            let depth_ptr = depth.as_mut_ptr();
            parent_ptr.add(root).write(!0);
            depth_ptr.add(root).write(0);
            while t < n {
                let u = *vec_queue.get_unchecked(t) as usize;
                t += 1;
                for &(v, _) in tree.adj(u) {
                    if v as u32 != *parent_ptr.add(u) {
                        parent_ptr.add(v).write(u as u32);
                        depth_ptr.add(v).write(*depth_ptr.add(u) + 1);
                        vec_queue.push(v as u32);
                    }
                }
            }
            parent.set_len(n);
            depth.set_len(n);
        }

        let mut size = vec![1; n];
        unsafe {
            let parent_ptr = parent.as_ptr();
            let size_ptr = size.as_mut_ptr();
            for &v in vec_queue[1..].iter().rev() {
                let v = v as usize;
                let p = *parent_ptr.add(v) as usize;
                *size_ptr.add(p) += *size_ptr.add(v);
            }
        }

        vec_queue.clear();
        let mut pos: Vec<u32> = Vec::with_capacity(n);
        let mut order: Vec<u32> = Vec::with_capacity(n);
        let mut head: Vec<u32> = Vec::with_capacity(n);
        vec_queue.push(root as u32);
        let mut t = 0;
        unsafe {
            let pos_ptr = pos.as_mut_ptr();
            let order_ptr = order.as_mut_ptr();
            let head_ptr = head.as_mut_ptr();
            let parent_ptr = parent.as_ptr();
            let size_ptr = size.as_ptr();
            head_ptr.add(root).write(root as u32);
            while let Some(u) = vec_queue.pop() {
                pos_ptr.add(u as usize).write(t);
                order_ptr.add(t as usize).write(u as u32);
                t += 1;
                let mut heavy = !0;
                let mut max_size = 0;
                for &(v, _) in tree.adj(u as usize) {
                    if v as u32 != *parent_ptr.add(u as usize) && *size_ptr.add(v) > max_size {
                        heavy = v as u32;
                        max_size = *size_ptr.add(v);
                    }
                }

                for &(v, _) in tree.adj(u as usize) {
                    if v as u32 != *parent_ptr.add(u as usize) && v as u32 != heavy {
                        head_ptr.add(v).write(v as u32);
                        vec_queue.push(v as u32);
                    }
                }

                if heavy != !0 {
                    *head_ptr.add(heavy as usize) = *head_ptr.add(u as usize);
                    vec_queue.push(heavy);
                }
            }
            pos.set_len(n);
            order.set_len(n);
            head.set_len(n);
        }

        Self {
            pos: pos.into_boxed_slice(),
            order: order.into_boxed_slice(),
            parent: parent.into_boxed_slice(),
            depth: depth.into_boxed_slice(),
            head: head.into_boxed_slice(),
            size: size.into_boxed_slice(),
        }
    }

    /// Returns the position of vertex `v` in the decomposition order.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn pos(&self, v: usize) -> usize {
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        unsafe { *self.pos.get_unchecked(v) as usize }
    }

    /// Returns the vertex at position `i` in the decomposition order.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn vertex(&self, i: usize) -> usize {
        debug_assert!(
            i < self.len(),
            "i is out of bounds: i={}, n={}",
            i,
            self.len()
        );
        unsafe { *self.order.get_unchecked(i) as usize }
    }

    /// Returns the parent of vertex `v`. Returns `!0` for the root.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn parent(&self, v: usize) -> usize {
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        unsafe { *self.parent.get_unchecked(v) as usize }
    }

    /// Returns the depth of vertex `v`.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn depth(&self, v: usize) -> usize {
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        unsafe { *self.depth.get_unchecked(v) as usize }
    }

    /// Returns the subtree interval of vertex `v` in decomposition order including `v`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn subtree(&self, v: usize) -> std::ops::Range<usize> {
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        unsafe {
            let l = *self.pos.get_unchecked(v) as usize;
            l..l + *self.size.get_unchecked(v) as usize
        }
    }

    /// Returns the LCA of `u` and `v`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        debug_assert!(
            u < self.len(),
            "u is out of bounds: u={}, n={}",
            u,
            self.len()
        );
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        unsafe {
            let head = self.head.as_ptr();
            let depth = self.depth.as_ptr();
            let parent = self.parent.as_ptr();
            while *head.add(u) != *head.add(v) {
                if *depth.add(*head.add(u) as usize) < *depth.add(*head.add(v) as usize) {
                    std::mem::swap(&mut u, &mut v);
                }
                u = *parent.add(*head.add(u) as usize) as usize;
            }
            if *depth.add(u) <= *depth.add(v) { u } else { v }
        }
    }

    /// Returns the distance (number of edges) between `u` and `v`.
    ///
    /// # Complexity
    /// Time: O(log n)
    pub fn dist(&self, u: usize, v: usize) -> usize {
        debug_assert!(
            u < self.len(),
            "u is out of bounds: u={}, n={}",
            u,
            self.len()
        );
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        self.depth(u) + self.depth(v) - 2 * self.depth(self.lca(u, v))
    }

    /// Decomposes the path from `u` to `v` into O(log n) vertex intervals.
    /// Calls `f(l, r, forward)` for each interval `[l, r)` in decomposition order.
    /// Calls are ordered along the path from `u` to `v`.
    ///
    /// # Complexity
    /// Time: O(log n) calls to `f`
    pub fn path_vertex<F: FnMut(usize, usize, bool)>(&self, mut u: usize, mut v: usize, mut f: F) {
        debug_assert!(
            u < self.len(),
            "u is out of bounds: u={}, n={}",
            u,
            self.len()
        );
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        let mut v_buf = [(0usize, 0usize); 32];
        let mut v_cnt = 0usize;
        unsafe {
            let pos = self.pos.as_ptr();
            let head = self.head.as_ptr();
            let depth = self.depth.as_ptr();
            let parent = self.parent.as_ptr();
            while *head.add(u) != *head.add(v) {
                if *depth.add(*head.add(u) as usize) > *depth.add(*head.add(v) as usize) {
                    let hu = *head.add(u) as usize;
                    f(*pos.add(hu) as usize, *pos.add(u) as usize + 1, false);
                    u = *parent.add(hu) as usize;
                } else {
                    let hv = *head.add(v) as usize;
                    v_buf[v_cnt] = (*pos.add(hv) as usize, *pos.add(v) as usize + 1);
                    v_cnt += 1;
                    v = *parent.add(hv) as usize;
                }
            }
            if *pos.add(u) <= *pos.add(v) {
                f(*pos.add(u) as usize, *pos.add(v) as usize + 1, true);
            } else {
                f(*pos.add(v) as usize, *pos.add(u) as usize + 1, false);
            }
            for i in (0..v_cnt).rev() {
                f(v_buf[i].0, v_buf[i].1, true);
            }
        }
    }

    /// Decomposes the path from `u` to `v` into O(log n) edge intervals.
    /// Calls `f(l, r, forward)` for each interval `[l, r)` in decomposition order, excluding
    /// the LCA vertex. Edges are identified with their child endpoint.
    /// Calls are ordered along the path from `u` to `v`.
    ///
    /// # Complexity
    /// Time: O(log n) calls to `f`
    pub fn path_edge<F: FnMut(usize, usize, bool)>(&self, mut u: usize, mut v: usize, mut f: F) {
        debug_assert!(
            u < self.len(),
            "u is out of bounds: u={}, n={}",
            u,
            self.len()
        );
        debug_assert!(
            v < self.len(),
            "v is out of bounds: v={}, n={}",
            v,
            self.len()
        );
        let mut v_buf = [(0usize, 0usize); 32];
        let mut v_cnt = 0usize;
        unsafe {
            let pos = self.pos.as_ptr();
            let head = self.head.as_ptr();
            let depth = self.depth.as_ptr();
            let parent = self.parent.as_ptr();
            while *head.add(u) != *head.add(v) {
                if *depth.add(*head.add(u) as usize) > *depth.add(*head.add(v) as usize) {
                    let hu = *head.add(u) as usize;
                    f(*pos.add(hu) as usize, *pos.add(u) as usize + 1, false);
                    u = *parent.add(hu) as usize;
                } else {
                    let hv = *head.add(v) as usize;
                    v_buf[v_cnt] = (*pos.add(hv) as usize, *pos.add(v) as usize + 1);
                    v_cnt += 1;
                    v = *parent.add(hv) as usize;
                }
            }
            let (l, r, forward) = if *pos.add(u) <= *pos.add(v) {
                (*pos.add(u) as usize + 1, *pos.add(v) as usize + 1, true)
            } else {
                (*pos.add(v) as usize + 1, *pos.add(u) as usize + 1, false)
            };
            if l < r {
                f(l, r, forward);
            }
            for i in (0..v_cnt).rev() {
                f(v_buf[i].0, v_buf[i].1, true);
            }
        }
    }

    /// Returns the number of vertices in tree.
    ///
    /// # Complexity
    /// Time: O(1)
    #[allow(clippy::len_without_is_empty)]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.pos.len()
    }
}
