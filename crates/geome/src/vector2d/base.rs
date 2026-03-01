/// A 2-dimensional vector over `T`.
///
/// # Complexity
/// Space: O(1)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2D<T>(T, T);

/// Shorthand for constructing a `Vector2D`.
pub fn v2<T: Copy, U: Into<T>>(x: U, y: U) -> Vector2D<T> {
    Vector2D::new(x.into(), y.into())
}

impl<T: Copy> Vector2D<T> {
    /// Creates a new 2-dimensional vector.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    /// Returns the x element of the vector.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn x(&self) -> T {
        self.0
    }

    /// Returns the y element of the vector.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn y(&self) -> T {
        self.1
    }
}

impl<T: Copy + Default> Vector2D<T> {
    /// Returns zero vector.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn zero() -> Self {
        Vector2D(T::default(), T::default())
    }
}

impl<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + std::ops::Mul<Output = T>,
> Vector2D<T>
{
    /// Returns inner product of vectors.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn inner(&self, rhs: Self) -> T {
        self.x() * self.y() + rhs.x() * rhs.y()
    }

    /// Returns outer product of vectors.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn outer(&self, rhs: Self) -> T {
        self.x() * rhs.y() - self.y() * rhs.x()
    }
}
