/// A 2-dimensional space point over `T`.
///
/// # Complexity
/// Space: O(1)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D<T>(T, T);

/// Shorthand for constructing a `Point2D`.
pub fn p2<T: Copy, U: Into<T>>(x: U, y: U) -> Point2D<T> {
    Point2D::new(x.into(), y.into())
}

impl<T: Copy> Point2D<T> {
    /// Creates a new 2-dimensional space point.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    /// Returns the x element of the point.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn x(&self) -> T {
        self.0
    }

    /// Returns the y element of the point.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn y(&self) -> T {
        self.1
    }
}

impl<T: Copy + Default> Point2D<T> {
    /// Returns the origin point.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn zero() -> Self {
        Point2D(T::default(), T::default())
    }
}
