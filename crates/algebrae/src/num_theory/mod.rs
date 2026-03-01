mod gcd;
mod prime;
mod sieve;

pub use gcd::{ext_gcd, gcd, lcm};
pub use prime::is_prime;
pub use sieve::eratosthenes::SieveEratosthenes;
