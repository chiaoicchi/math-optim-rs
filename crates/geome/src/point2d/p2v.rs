use crate::{point2d::Point2D, vector2d::Vector2D};

impl<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T>>
    Point2D<T>
{
    /// Returns the vector from a point to another point.
    ///
    /// # Complexity
    /// Time: O(1)
    #[inline(always)]
    pub fn to(&self, rhs: Self) -> Vector2D<T> {
        Vector2D::new(rhs.x() - self.x(), rhs.y() - self.y())
    }
}
