use crate::flow::ResidualGraph;

/// Computes the maximum flow from `s` to `t` on the residual graph using Dinic's algorithm.
///
/// # Complexity
/// Time: P(V^2 E) in general, O(E sqrt(V)) for unit-capacity graphs.
pub fn dinic<
    Cap: Copy + Default + Ord + std::ops::Add<Output = Cap> + std::ops::Sub<Output = Cap>,
>(
    g: &mut ResidualGraph<Cap>,
    s: usize,
    t: usize,
    flow_limit: Cap,
) -> Cap {
    let n = g.num_vertices();
    debug_assert!(s < n, "source vertex out of bounds: s={}, n={}", s, n);
    debug_assert!(t < n, "destination vertex out of bounds: t={}, n={}", t, n);
    let mut level = vec![u32::MAX; n];
    let mut iter = vec![0u32; n];
    let mut queue = Vec::with_capacity(n);

    let mut flow = Cap::default();
    while flow < flow_limit {
        unsafe {
            let offset = g.offset.as_ptr();
            let edge = g.edge.as_ptr();

            level.fill(u32::MAX);
            let lev = level.as_mut_ptr();
            let iter = iter.as_mut_ptr();

            *lev.add(s) = 0;
            queue.clear();
            queue.push(s as u32);
            let mut head = 0;
            let que = queue.as_mut_ptr();
            while head < queue.len() {
                let v = *que.add(head) as usize;
                head += 1;
                for e in *offset.add(v) as usize..*offset.add(v + 1) as usize {
                    let (to, _, cap) = *edge.add(e);
                    let to = to as usize;
                    if cap > Cap::default() && *lev.add(to) == u32::MAX {
                        *lev.add(to) = *lev.add(v) + 1;
                        if to == t {
                            break;
                        }
                        queue.push(to as u32);
                    }
                }
            }
            if *lev.add(t) == u32::MAX {
                break;
            }

            for v in 0..n {
                *iter.add(v) = *offset.add(v);
            }

            let f = dfs(g, lev, iter, s, t, flow_limit - flow);
            if f == Cap::default() {
                break;
            }
            flow = flow + f;
        }
    }
    flow
}

fn dfs<Cap: Copy + Default + Ord + std::ops::Add<Output = Cap> + std::ops::Sub<Output = Cap>>(
    g: &mut ResidualGraph<Cap>,
    lev: *mut u32,
    iter: *mut u32,
    s: usize,
    v: usize,
    up: Cap,
) -> Cap {
    if v == s {
        return up;
    }
    let mut res = Cap::default();
    unsafe {
        let lv = *lev.add(v);
        let hi = *g.offset.as_ptr().add(v + 1);
        let edge = g.edge.as_mut_ptr();
        while *iter.add(v) < hi {
            let e = *iter.add(v) as usize;
            let (to, re, _) = *edge.add(e);
            let to = to as usize;
            let re = re as usize;
            let rev_cap = (*edge.add(re)).2;
            if lv <= *lev.add(to) || rev_cap == Cap::default() {
                *iter.add(v) += 1;
                continue;
            }
            let limit = {
                let rem = up - res;
                if rem < rev_cap { rem } else { rev_cap }
            };
            let d = dfs(g, lev, iter, s, to, limit);
            if d == Cap::default() {
                *iter.add(v) += 1;
                continue;
            }
            (*edge.add(e)).2 = (*edge.add(e)).2 + d;
            (*edge.add(re)).2 = (*edge.add(re)).2 - d;
            res = res + d;
            if res == up {
                return res;
            }
        }
        *lev.add(v) = g.num_vertices() as u32;
    }
    res
}
