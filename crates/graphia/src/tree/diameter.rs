use crate::csr::{Csr, EdgeWeight};

/// Returns `(dist, u, v)` where `u` and `v` are endpoints of a diameter and `dist` is the maximum
/// path weight.
///
/// # Complexity
/// Time: O(n)
pub fn diameter<W: EdgeWeight>(tree: &Csr<W>) -> (W::Dist, usize, usize) {
    let n = tree.num_vertices();
    debug_assert!(n > 0, "tree must not be empty tree");
    if n == 1 {
        return (W::Dist::default(), 0, 0);
    }

    let mut dist = vec![W::Dist::default(); n];
    let mut flag = vec![false; n];
    let mut stack = Vec::new();
    let mut max_dist = W::Dist::default();
    let mut p = 0;
    let mut q = 0;

    unsafe {
        let d = dist.as_mut_ptr();
        let f = flag.as_mut_ptr();
        *f = true;
        stack.push(0);
        while let Some(u) = stack.pop() {
            for &(v, w) in tree.adj(u) {
                if !*f.add(v) {
                    *f.add(v) = true;
                    *d.add(v) = *d.add(u) + w.dist();
                    if *d.add(v) > max_dist {
                        max_dist = *d.add(v);
                        p = v;
                    }
                    stack.push(v);
                }
            }
        }

        *f.add(p) = false;
        *d.add(p) = W::Dist::default();
        stack.push(p);
        while let Some(u) = stack.pop() {
            for &(v, w) in tree.adj(u) {
                if *f.add(v) {
                    *f.add(v) = false;
                    *d.add(v) = *d.add(u) + w.dist();
                    if *d.add(v) > max_dist {
                        max_dist = *d.add(v);
                        q = v;
                    }
                    stack.push(v);
                }
            }
        }
    }
    (max_dist, p, q)
}

/// Returns `(dist, path)` where `path` is a vertex sequence from one endpoint to the other and
/// `dist` is the diameter weight.
///
/// # Complexity
/// Time: O(n)
pub fn diameter_path<W: EdgeWeight>(tree: &Csr<W>) -> (W::Dist, Vec<usize>) {
    let n = tree.num_vertices();
    debug_assert!(n > 0, "tree must not be empty tree");
    if n == 1 {
        return (W::Dist::default(), vec![0]);
    }

    let mut dist = vec![W::Dist::default(); n];
    let mut flag = vec![false; n];
    let mut stack = Vec::new();
    let mut max_dist = W::Dist::default();
    let mut p = 0;
    let mut prev = vec![!0; n];
    let mut q = 0;

    unsafe {
        let d = dist.as_mut_ptr();
        let f = flag.as_mut_ptr();
        *f = true;
        stack.push(0);
        while let Some(u) = stack.pop() {
            for &(v, w) in tree.adj(u) {
                if !*f.add(v) {
                    *f.add(v) = true;
                    *d.add(v) = *d.add(u) + w.dist();
                    if *d.add(v) > max_dist {
                        max_dist = *d.add(v);
                        p = v;
                    }
                    stack.push(v);
                }
            }
        }

        let prev = prev.as_mut_ptr();
        *f.add(p) = false;
        *d.add(p) = W::Dist::default();
        stack.push(p);
        while let Some(u) = stack.pop() {
            for &(v, w) in tree.adj(u) {
                if *f.add(v) {
                    *f.add(v) = false;
                    *d.add(v) = *d.add(u) + w.dist();
                    *prev.add(v) = u;
                    if *d.add(v) > max_dist {
                        max_dist = *d.add(v);
                        q = v;
                    }
                    stack.push(v);
                }
            }
        }
        let mut path = Vec::new();
        while q != !0 {
            path.push(q);
            q = *prev.add(q);
        }
        (max_dist, path)
    }
}
