use crate::point2d::Point2D;

/// Computes convex hull of a set of points.
///
/// # Complexity
/// Time: O(n log n)
pub fn convex_hull<
    T: Copy
        + Default
        + PartialOrd
        + Ord
        + PartialEq
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + std::ops::Mul<Output = T>,
>(
    points: &mut [Point2D<T>],
) -> Vec<Point2D<T>> {
    if points.is_empty() {
        return Vec::new();
    }
    points.sort_unstable_by_key(|point| (point.x(), point.y()));
    let mut unique_len = 1;
    for i in 1..points.len() {
        if points[i] != points[unique_len - 1] {
            points[unique_len] = points[i];
            unique_len += 1;
        }
    }
    let points = &mut points[..unique_len];
    if points.len() <= 2 {
        return points.to_vec();
    }

    // To include collinear points on the hull boundary, change `points.len() + 1` to `2 *
    // points.len()`.
    let mut res: Vec<Point2D<T>> = Vec::with_capacity(points.len() + 1);
    unsafe {
        let res_ptr = res.as_mut_ptr();
        let mut len = 0;
        for point in points.iter() {
            // To include collinear points on the hull boundary, change `<=` to `<`.
            while len > 1
                && ((*res_ptr.add(len - 2)).to(*res_ptr.add(len - 1)))
                    .outer((*res_ptr.add(len - 2)).to(*point))
                    <= T::default()
            {
                len -= 1;
            }
            res_ptr.add(len).write(*point);
            len += 1;
        }
        let lower_len = len;
        for point in points.iter().rev().skip(1) {
            // To include collinear points on the hull boundary, change `<=` to `<`.
            while len > lower_len
                && ((*res_ptr.add(len - 2)).to(*res_ptr.add(len - 1)))
                    .outer((*res_ptr.add(len - 2)).to(*point))
                    <= T::default()
            {
                len -= 1;
            }
            res_ptr.add(len).write(*point);
            len += 1;
        }
        len -= 1;
        res.set_len(len);
    }
    res
}
