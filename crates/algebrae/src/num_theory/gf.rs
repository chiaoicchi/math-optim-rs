/// A element of Galois field Z/pZ.
///
/// # Complexity
/// Space: O(1)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gf<const P: u32>(u32);

impl<const P: u32> Gf<P> {
    /// Creates a new element from a value, reduced modulo `P`.
    ///
    /// # Complexity
    /// Time: O(1)
    pub fn new(value: u32) -> Self {
        Self(value % P)
    }

    /// Returns `self^exp` computed by binary exponentiation.
    ///
    /// # Complexity
    /// Time: O(log exp)
    pub fn pow(&self, mut exp: u64) -> Self {
        let mut res = Self::new(1);
        let mut base = *self;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }
        res
    }

    /// Returns the multiplicative inverse `self^{-1}` in Z/pZ.
    ///
    /// # Complexity
    /// Time: O(log P)
    pub fn inv(&self) -> Self {
        debug_assert!(self.0 != 0, "zero has no inverse in Z/{}Z", P);
        self.pow(P as u64 - 2)
    }
}

macro_rules! impl_gf_new_from_signed {
    ($($src:ty), *) => {
        $(
            impl<const P: u32> From<$src> for Gf<P> {
                fn from(x: $src) -> Self {
                    if x < 0 {
                        - Self::new((x as i128).rem_euclid(P as i128) as u32)
                    } else {
                        Self::new(x as u32)
                    }
                }
            }
        )*
    };
}

macro_rules! impl_gf_new_from_unsigned {
    ($($src:ty), *) => {
        $(
            impl<const P: u32> From<$src> for Gf<P> {
                fn from(x: $src) -> Self {
                    Self::new((x % P as $src) as u32)
                }
            }
        )*
    };
}

impl_gf_new_from_signed!(i8, i16, i32, i64, i128, isize);
impl_gf_new_from_unsigned!(u8, u16, u32, u64, u128, usize);

impl<const P: u32> std::fmt::Debug for Gf<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const P: u32> std::fmt::Display for Gf<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const P: u32> std::ops::Neg for Gf<P> {
    type Output = Self;
    #[inline]
    fn neg(mut self) -> Self::Output {
        if self.0 > 0 {
            self.0 = P - self.0;
        }
        self
    }
}

impl<const P: u32> std::ops::Add for Gf<P> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        self.0 += rhs.0;
        if self.0 >= P {
            self.0 -= P;
        }
        self
    }
}

impl<const P: u32> std::ops::Sub for Gf<P> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self {
        if self.0 < rhs.0 {
            self.0 += P;
        }
        self.0 -= rhs.0;
        self
    }
}

impl<const P: u32> std::ops::Mul for Gf<P> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self((self.0 as u64 * rhs.0 as u64 % P as u64) as u32)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const P: u32> std::ops::Div for Gf<P> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

macro_rules! forward_ref_binop {
    ($($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl<const P:u32> std::ops::$trait<&Self> for Gf<P> {
                type Output = Self;
                #[inline]
                fn $method(self, rhs: &Self) -> Self {
                    std::ops::$trait::$method(self, *rhs)
                }
            }

            impl<const P:u32> std::ops::$trait<Gf<P>> for &Gf<P> {
                type Output = Gf<P>;
                #[inline]
                fn $method(self, rhs: Gf<P>) -> Gf<P> {
                    std::ops::$trait::$method(*self, rhs)
                }
            }

            impl<const P: u32> std::ops::$trait for &Gf<P> {
                type Output = Gf<P>;
                #[inline]
                fn $method(self, rhs: Self) -> Gf<P> {
                    std::ops::$trait::$method(*self, *rhs)
                }
            }
        )*
    };
}

forward_ref_binop! {
    Add, add;
    Sub, sub;
    Mul, mul;
    Div, div;
}

impl<const P: u32> std::ops::AddAssign for Gf<P> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        if self.0 >= P {
            self.0 -= P;
        }
    }
}

impl<const P: u32> std::ops::SubAssign for Gf<P> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        if self.0 < rhs.0 {
            self.0 += P;
        }
        self.0 -= rhs.0;
    }
}

impl<const P: u32> std::ops::MulAssign for Gf<P> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 as u64 * rhs.0 as u64 % P as u64) as u32;
    }
}

impl<const P: u32> std::ops::DivAssign for Gf<P> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

macro_rules! forward_ref_op_assign {
    ($($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl<const P:u32> std::ops::$trait<&Self> for Gf<P> {
                #[inline]
                fn $method(&mut self, rhs: &Self) {
                    std::ops::$trait::$method(self, *rhs);
                }
            }
        )*
    };
}

forward_ref_op_assign! {
    AddAssign, add_assign;
    SubAssign, sub_assign;
    MulAssign, mul_assign;
    DivAssign, div_assign;
}

use crate::algebra::Rig;

impl<const P: u32> Rig for Gf<P> {
    fn zero() -> Self {
        Self(0)
    }
    fn one() -> Self {
        Self(1)
    }
}
