use crate::modular::pow_mod;
use crate::num_theory::gcd;

/// Returns whether given value is prime.
///
/// # Complexity
/// Time: O(log^2 n), Space: O(1)
pub fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    } else if n == 2 {
        return true;
    } else if n & 1 == 0 {
        return false;
    }
    let r = (n - 1).trailing_zeros();
    let d = (n - 1) >> r;

    const SMALL: [u64; 3] = [2, 7, 61];
    const LARGE: [u64; 7] = [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022];

    if n < 4_759_123_141 {
        miller_rabin(n, d, r, &SMALL)
    } else {
        miller_rabin(n, d, r, &LARGE)
    }
}

#[inline(always)]
fn miller_rabin<const N: usize>(n: u64, d: u64, r: u32, witnesses: &[u64; N]) -> bool {
    for &x in witnesses {
        if x >= n {
            break;
        }
        let mut pow = pow_mod(x, d, n);
        if pow == 1 || pow == n - 1 {
            continue;
        }
        let mut found = false;
        for _ in 1..r {
            pow = (pow as u128 * pow as u128 % n as u128) as u64;
            if pow == n - 1 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

/// Factorizes given value.
///
/// # Complexity
/// Time: O(n^{1/4} log n), Space: O(log n)
pub fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    assert!(n > 0, "n must not be zero");
    if n == 1 {
        return Vec::new();
    }
    let two = n.trailing_zeros();
    let mut res = Vec::new();
    if two > 0 {
        res.push((2, two));
        n >>= two;
    }
    let mut three = 0;
    while n % 3 == 0 {
        three += 1;
        n /= 3;
    }
    if three > 0 {
        res.push((3, three));
    }
    if n == 1 {
        return res;
    }

    let mut factors = vec![n];
    let mut i = 0;
    unsafe {
        while i < factors.len() {
            let n = *factors.get_unchecked(i);
            if is_prime(n) {
                i += 1;
                continue;
            }

            'LOOP: for t in 1.. {
                let mut x: u64 = t;
                let mut y = ((x as u128 * x as u128 + t as u128) % n as u128) as u64;
                loop {
                    let g = gcd(x.abs_diff(y), n);
                    if g == n {
                        break;
                    }
                    if g != 1 {
                        *factors.get_unchecked_mut(i) /= g;
                        factors.push(g);
                        break 'LOOP;
                    }
                    x = ((x as u128 * x as u128 + t as u128) % n as u128) as u64;
                    y = ((y as u128 * y as u128 + t as u128) % n as u128) as u64;
                    y = ((y as u128 * y as u128 + t as u128) % n as u128) as u64;
                }
            }
        }
    }

    factors.sort_unstable();
    let mut i = 0;
    let len = factors.len();
    unsafe {
        let f = factors.as_ptr();
        while i < len {
            let p = *f.add(i);
            let mut j = i + 1;
            while j < len && *f.add(j) == p {
                j += 1;
            }
            res.push((p, (j - i) as u32));
            i = j;
        }
    }
    res
}

/// Returns primitive root of prime number p.
///
/// # Complexity
/// Time: practically small, Space: O(log p)
pub fn primitive_root(p: u64) -> u64 {
    if p == 2 {
        return 1;
    }
    let factors = factorize(p - 1);
    'LOOP: for g in 2..p {
        for (f, _) in &factors {
            if pow_mod(g, (p - 1) / f, p) == 1 {
                continue 'LOOP;
            }
        }
        return g;
    }
    unreachable!();
}
