/// A commutative monoid trait.
pub trait Monoid: Clone {
    /// Returns the identity element of the monoid.
    fn id() -> Self;
    /// Performs the binary operation of the monoid.
    fn op(&self, other: &Self) -> Self;
}
