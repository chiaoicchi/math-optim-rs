use crate::csr::Csr;

/// Computes strongly connected components using Kosaraju's algorithm. Returns a vector `comp`
/// where `comp[v]` is the SCC index of vertex `v`. SCC indices are in topological order.
///
/// # Complexity
/// Time: O(n + m)
pub fn kosaraju(graph: &Csr<()>) -> Vec<usize> {
    const FLAG: usize = 1 << (usize::BITS - 1);

    let n = graph.num_vertices();

    let mut order = Vec::with_capacity(n);
    let mut flag = vec![false; n];
    let mut stack = Vec::new();

    unsafe {
        let f = flag.as_mut_ptr();
        for s in 0..n {
            if *f.add(s) {
                continue;
            }
            stack.push(s);

            while let Some(x) = stack.pop() {
                if x >= FLAG {
                    order.push(x ^ FLAG);
                } else if !*f.add(x) {
                    *f.add(x) = true;
                    stack.push(x | FLAG);
                    for &(y, _) in graph.adj(x) {
                        if !*f.add(y) {
                            stack.push(y);
                        }
                    }
                }
            }
        }
    }

    let mut comp = vec![0; n];
    let mut num_comp = 0;
    let rev = graph.reverse();

    unsafe {
        let f = flag.as_mut_ptr();
        let c = comp.as_mut_ptr();
        for &s in order.iter().rev() {
            if !*f.add(s) {
                continue;
            }
            *f.add(s) = false;
            *c.add(s) = num_comp;
            stack.push(s);

            while let Some(x) = stack.pop() {
                for &(y, _) in rev.adj(x) {
                    if *f.add(y) {
                        *f.add(y) = false;
                        *c.add(y) = num_comp;
                        stack.push(y);
                    }
                }
            }
            num_comp += 1;
        }
    }

    comp
}
