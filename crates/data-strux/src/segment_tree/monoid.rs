/// A commutative monoid trait.
pub trait Monoid: Clone {
    /// Returns the identity element of the monoid.
    fn id() -> Self;
    /// Performs the binary operation of the monoid.
    fn op(&self, other: &Self) -> Self;
}

/// An action of a monoid `F` on a monoid `S`.
pub trait Action<S: Monoid>: Monoid {
    /// Returns self acting on s.
    fn act(&self, s: &S) -> S;
}
