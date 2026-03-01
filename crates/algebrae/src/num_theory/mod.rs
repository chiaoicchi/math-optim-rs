mod gcd;
mod prime;
mod sieve;

pub use gcd::{ext_gcd, gcd, lcm};
pub use prime::{factorize, is_prime, primitive_root};
pub use sieve::eratosthenes::SieveEratosthenes;
