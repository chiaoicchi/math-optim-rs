/// Computes gcd(a, b) using the binary GCD (Stein's) algorithm.
///
/// # Complexity
/// Time: O(log(a + b))
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 || b == 0 {
        return a + b;
    }
    let x = a.trailing_zeros();
    let y = b.trailing_zeros();
    a >>= x;
    b >>= y;
    while a != b {
        let x = (a ^ b).trailing_zeros();
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a = (a - b) >> x;
    }
    a << x.min(y)
}

/// Returns (g, x, y) such that ax + by = g = gcd(|a|, |b|). When both `a` and `b` are zero,
/// returns (0, 0, 0).
///
/// # Complexity
/// Time: O(log(a + b))
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 && b == 0 {
        return (0, 0, 0);
    }
    if a == 0 {
        return if b > 0 { (b, 0, 1) } else { (-b, 0, -1) };
    }
    if b == 0 {
        return if a > 0 { (a, 1, 0) } else { (-a, -1, 0) };
    }

    let (mut prev_r, mut r) = (a.abs(), b.abs());
    let (mut prev_x, mut x) = (1, 0);

    while r != 0 {
        let q = prev_r / r;
        (prev_r, r) = (r, prev_r - q * r);
        (prev_x, x) = (x, prev_x - q * x);
    }

    (
        prev_r,
        if a > 0 { prev_x } else { -prev_x },
        ((prev_r as i128 - a.abs() as i128 * prev_x as i128) / b as i128) as i64,
    )
}
