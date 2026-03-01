use crate::modular::pow_mod;

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
