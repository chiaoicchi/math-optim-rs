use algebrae::algebra::Group;

/// A potential disjoint set union (DSU) data structure.
///
/// # Complexity
/// Space: O(n)
pub struct PotentialDsu<S: PartialEq + Group> {
    /// If negative, this node is a root and the absolute value is the size of the set.
    /// If non-negative, this is the index of the parent node.
    parent: Box<[i32]>,
    potential: Box<[std::mem::MaybeUninit<S>]>,
    count: usize,
}

impl<S: PartialEq + Group> PotentialDsu<S> {
    /// Creates a new Potential DSU with `n` elements, where each element is initially in its own
    /// set.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn new(n: usize) -> Self {
        debug_assert!(n < (1 << 31), "n must be less than 1<<31, n={}", n);
        let mut potential = Vec::with_capacity(n);
        unsafe {
            potential.set_len(n);
        }
        Self {
            parent: vec![-1; n].into_boxed_slice(),
            potential: potential.into_boxed_slice(),
            count: n,
        }
    }

    /// Returns the representative (root) of the set containing `x` and the potential from `root`
    /// to `x`.
    ///
    /// # Complexity
    /// Time: Amortized O(α(n)), where α is the inverse Ackermann function.
    #[inline(always)]
    pub fn root(&mut self, mut x: usize) -> (usize, S) {
        debug_assert!(
            x < self.len(),
            "index out of bounds: x={}, len={}",
            x,
            self.len()
        );
        let mut acc = S::id();

        unsafe {
            let parent = self.parent.as_mut_ptr();
            let potential = self.potential.as_mut_ptr() as *mut S;

            while *parent.add(x) >= 0 {
                let px = *parent.add(x) as usize;
                if *parent.add(px) >= 0 {
                    let np = (*potential.add(px)).op(&*potential.add(x));
                    potential.add(x).write(np);
                    *parent.add(x) = *parent.add(px);
                }
                acc = (*potential.add(x)).op(&acc);
                x = *parent.add(x) as usize;
            }
        }
        (x, acc)
    }

    /// Unites the sets containing `x` and `y` with `p` potential from `x` to `y`. When this union
    /// is illegal, return false.
    ///   
    /// # Complexity
    /// Time: Amortized O(α(n)), where α is the inverse Ackermann function.
    pub fn unite(&mut self, x: usize, y: usize, p: S) -> bool {
        debug_assert!(
            x < self.len(),
            "index out of bounds: x={}, len={}",
            x,
            self.len()
        );
        debug_assert!(
            y < self.len(),
            "index out of bounds: y={}, len={}",
            y,
            self.len()
        );

        let (mut rx, px) = self.root(x);
        let (mut ry, py) = self.root(y);

        if rx == ry {
            return px.op(&p) == py;
        }

        unsafe {
            let parent = self.parent.as_mut_ptr();
            let potential = self.potential.as_mut_ptr() as *mut S;
            let mut p = px.op(&p).op(&py.inv());
            if *parent.add(rx) > *parent.add(ry) {
                std::mem::swap(&mut rx, &mut ry);
                p = p.inv();
            }
            *parent.add(rx) += *parent.add(ry);
            *parent.add(ry) = rx as i32;
            potential.add(ry).write(p);
        }
        self.count -= 1;
        true
    }

    /// Returns potential from `x` to `y`. When `x` and `y` are not same, return `None`.
    ///
    /// # Complexity
    /// Time: Amortized O(α(n)), where α is the inverse Ackermann function.
    pub fn potential(&mut self, x: usize, y: usize) -> Option<S> {
        debug_assert!(
            x < self.len(),
            "index out of bounds: x={}, len={}",
            x,
            self.len()
        );
        debug_assert!(
            y < self.len(),
            "index out of bounds: y={}, len={}",
            y,
            self.len()
        );

        let (rx, px) = self.root(x);
        let (ry, py) = self.root(y);
        if rx == ry {
            Some(px.inv().op(&py))
        } else {
            None
        }
    }

    /// Returns the size of the set containing `x`.
    ///
    /// # Complexity
    /// Time: Amortized O(α(n)), where α is the inverse Ackermann function.
    pub fn set_size(&mut self, x: usize) -> usize {
        debug_assert!(
            x < self.len(),
            "index out of bounds: x={}, len={}",
            x,
            self.len()
        );
        let root = self.root(x).0;
        unsafe { (-self.parent.get_unchecked(root)) as usize }
    }

    /// Returns the number of disjoint sets.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn num_sets(&self) -> usize {
        self.count
    }

    /// Returns the total number of elements.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Returns whether the DSU contains no elements.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}
