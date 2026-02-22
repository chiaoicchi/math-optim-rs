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

impl<T: Ord + std::ops::Mul<Output = T> + Copy + Default> Vector2D<T> {
    /// Compares `self` and `other` by their argument (polar angle). The argument is measured
    /// counter-clockwise from the positive x-axis, ranging over [0, 2pi). The positive x-axis has
    /// argument 0.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn arg_cmp_unsigned(&self, other: &Self) -> std::cmp::Ordering {
        debug_assert!(*self != Self::zero(), "self must not be zero",);
        debug_assert!(*other != Self::zero(), "other must not be zero",);
        ((self.y(), self.x()) < (T::default(), T::default()))
            .cmp(&((other.y(), other.x()) < (T::default(), T::default())))
            .then_with(|| (other.x() * self.y()).cmp(&(self.x() * other.y())))
    }

    /// Compares `self` and `other` by their argument (polar angle). The argument is measured
    /// counter-clockwise from the positive x-axis, ranging over (-pi, pi]. The positive x-axis has
    /// argument 0.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn arg_cmp_signed(&self, other: &Self) -> std::cmp::Ordering {
        debug_assert!(*self != Self::zero(), "self must not be zero",);
        debug_assert!(*other != Self::zero(), "other must not be zero",);
        (
            self.y() >= T::default(),
            self.y() == T::default() && self.x() < T::default(),
        )
            .cmp(&(
                other.y() >= T::default(),
                other.y() == T::default() && other.x() < T::default(),
            ))
            .then_with(|| (other.x() * self.y()).cmp(&(self.x() * other.y())))
    }
}

