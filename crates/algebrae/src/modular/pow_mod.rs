/// Computes `base^exp % modulus` using binary exponentiation.
///
/// # Complexity
/// Time: O(log exp)
pub fn pow_mod(base: u64, mut exp: u64, modulus: u64) -> u64 {
    debug_assert!(modulus > 0, "modulus must not be zero");
    let mut base = base as u128;
    let modulus = modulus as u128;
    base %= modulus;
    let mut res = 1 % modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            res *= base;
            res %= modulus;
        }
        base *= base;
        base %= modulus;
        exp >>= 1;
    }
    res as u64
}
