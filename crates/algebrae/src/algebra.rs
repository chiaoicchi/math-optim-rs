/// A band trait.
pub trait Band: Clone {
    /// Performs the binary operation of the band.
    fn op(&self, other: &Self) -> Self;
}

/// A monoid trait.
pub trait Monoid: Clone {
    /// Returns the identity element of the monoid.
    fn id() -> Self;
    /// Performs the binary operation of the monoid.
    fn op(&self, rhs: &Self) -> Self;
}

/// A group trait.
pub trait Group: Clone {
    /// Returns the identity element of the group.
    fn id() -> Self;
    /// Performs the binary operation of the group.
    fn op(&self, rhs: &Self) -> Self;
    /// Returns the inverse of the element.
    fn inv(&self) -> Self;
}

/// An Abelian group trait.
pub trait AbelianGroup: Clone {
    /// Returns the identity element of the Abelian group.
    fn id() -> Self;
    /// Performs the binary operation of the Abelian group.
    fn op(&self, rhs: &Self) -> Self;
    /// Returns the inverse of the element.
    fn inv(&self) -> Self;
}

/// A semi ring trait which is called rig.
pub trait Rig: Copy + std::ops::Add<Output = Self> + std::ops::Mul<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
}

/// A ring trait.
pub trait Ring: Rig + std::ops::Sub<Output = Self> + std::ops::Neg<Output = Self> {}
impl<T: Rig + std::ops::Sub<Output = Self> + std::ops::Neg<Output = Self>> Ring for T {}

/// A field trait.
pub trait Field: Ring + std::ops::Div<Output = Self> {}
impl<T: Ring + std::ops::Div<Output = Self>> Field for T {}

/// A set of actions on a set `S`.
pub trait Action<S: Clone> {
    /// Returns self acting on s.
    fn act(&self, s: &S) -> S;
}
