/// A disjoint set union (DSU) data structure.
/// Uses path splitting and union by size.
///
/// # Complexity
/// Space: O(n)
#[derive(Clone, Debug)]
pub struct Dsu {
    /// If negative, this node is a root and the absolute value is the size of the set.
    /// If non-negative, this is the index of the parent node.
    parent: Box<[i32]>,
    count: usize,
}

impl Dsu {
    /// Creates a new DSU with `n` elements, where each element is initially in its own set.
    ///
    /// # Complexity
    /// Time: O(n)
    pub fn new(n: usize) -> Self {
        debug_assert!(n < (1 << 31), "n must be less than 1<<31, n={}", n);
        Self {
            parent: vec![-1; n].into_boxed_slice(),
            count: n,
        }
    }

    /// Returns the representative (root) of the set containing `x`.
    ///
    /// # Complexity
    /// Time: Amortized O(α(n)), where α is the inverse Ackermann function.
    #[inline(always)]
    pub fn root(&mut self, mut x: usize) -> usize {
        debug_assert!(
            x < self.len(),
            "index out of bounds: x={}, len={}",
            x,
            self.len()
        );
        unsafe {
            let p = self.parent.as_mut_ptr();
            while *p.add(x) >= 0 {
                let px = *p.add(x) as usize;
                if *p.add(px) >= 0 {
                    *p.add(x) = *p.add(px);
                }
                x = px;
            }
        }
        x
    }

    /// Unites the sets containing `x` and `y` and returns whether `x` and `y` were in different
    /// sets.
    ///
    /// # Complexity
    /// Time: Amortized O(α(n)), where α is the inverse Ackermann function.
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
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
        let (mut rx, mut ry) = (self.root(x), self.root(y));
        if rx == ry {
            return false;
        }
        unsafe {
            let p = self.parent.as_mut_ptr();
            if *p.add(rx) > *p.add(ry) {
                std::mem::swap(&mut rx, &mut ry);
            }
            *p.add(rx) += *p.add(ry);
            *p.add(ry) = rx as i32;
        }
        self.count -= 1;
        true
    }

    /// Returns whether `x` and `y` belong to the same set.
    ///
    /// # Complexity
    /// Time: Amortized O(α(n)), where α is the inverse Ackermann function.
    pub fn same(&mut self, x: usize, y: usize) -> bool {
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
        self.root(x) == self.root(y)
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
        let root = self.root(x);
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
