use crate::vector2d::Vector2D;

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
