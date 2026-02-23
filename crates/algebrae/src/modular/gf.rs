macro_rules! impl_gf_new_from_signed {
    ($gf:ident, $t:ty, $wide:ty; $($src:ty), *) => {
        $(
            impl<const P: $t> From<$src> for $gf<P> {
                fn from(x: $src) -> Self {
                    if x < 0 {
                        - Self::new((P as $wide - x as $wide) as $t)
                    } else {
                        Self::new(x as $t)
                    }
                }
            }
        )*
    };
}

macro_rules! impl_gf_new_from_unsigned {
    ($gf:ident, $t:ty; $($src:ty), *) => {
        $(
            impl<const P: $t> From<$src> for $gf<P> {
                fn from(x: $src) -> Self {
                    Self::new(x as $t)
                }
            }
        )*
    };
}

macro_rules! forward_ref_op_assign {
    ($gf:ident, $t:ty; $($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl<const P: $t> std::ops::$trait<&Self> for $gf<P> {
                #[inline]
                fn $method(&mut self, rhs: &Self) {
                    std::ops::$trait::$method(self, *rhs);
                }
            }
        )*
    };
}

macro_rules! forward_ref_binop {
    ($gf:ident, $t:ty; $($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl<const P: $t> std::ops::$trait<&Self> for $gf<P> {
                type Output = Self;
                #[inline]
                fn $method(self, rhs: &Self) -> Self {
                    std::ops::$trait::$method(self, *rhs)
                }
            }

            impl<const P: $t> std::ops::$trait<$gf<P>> for &$gf<P> {
                type Output = $gf<P>;
                #[inline]
                fn $method(self, rhs: $gf<P>) -> $gf<P> {
                    std::ops::$trait::$method(*self, rhs)
                }
            }

            impl<const P: $t> std::ops::$trait for &$gf<P> {
                type Output = $gf<P>;
                #[inline]
                fn $method(self, rhs: Self) -> $gf<P> {
                    std::ops::$trait::$method(*self, *rhs)
                }
            }
        )*
    };
}

macro_rules! impl_gf {
    ($gf:ident, $t:ty, $wide:ty) => {
        /// A element of Galois field Z/pZ.
        ///
        /// # Complexity
        /// Space: O(1)
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $gf<const P: $t>($t);

        impl<const P: $t> $gf<P> {
            /// Creates a new element from a value, reduced modulo `P`.
            ///
            /// # Complexity
            /// Time: O(1)
            pub fn new(value: $t) -> Self {
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

        impl_gf_new_from_signed!($gf, $t, i128; i8, i16, i32, i64, i128, isize);
        impl_gf_new_from_unsigned!($gf, $t; u8, u16, u32, u64, u128, usize);

        impl<const P: $t> std::fmt::Debug for $gf<P> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<const P: $t> std::fmt::Display for $gf<P> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<const P: $t> std::ops::Neg for $gf<P> {
            type Output = Self;
            #[inline]
            fn neg(mut self) -> Self::Output {
                if self.0 > 0 {
                    self.0 = P - self.0;
                }
                self
            }
        }

        impl<const P: $t> std::ops::Add for $gf<P> {
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

        impl<const P: $t> std::ops::Sub for $gf<P> {
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

        impl<const P: $t> std::ops::Mul for $gf<P> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                Self((self.0 as $wide * rhs.0 as $wide % P as $wide) as $t)
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl<const P: $t> std::ops::Div for $gf<P> {
            type Output = Self;
            #[inline]
            fn div(self, rhs: Self) -> Self {
                self * rhs.inv()
            }
        }

        impl<const P: $t> std::ops::AddAssign for $gf<P> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
                if self.0 >= P {
                    self.0 -= P;
                }
            }
        }

        impl<const P: $t> std::ops::SubAssign for $gf<P> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                if self.0 < rhs.0 {
                    self.0 += P;
                }
                self.0 -= rhs.0;
            }
        }

        impl<const P: $t> std::ops::MulAssign for $gf<P> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 = (self.0 as $wide * rhs.0 as $wide % P as $wide) as $t;
            }
        }

        impl<const P: $t> std::ops::DivAssign for $gf<P> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        forward_ref_binop! {
            $gf, $t;
            Add, add;
            Sub, sub;
            Mul, mul;
            Div, div;
        }

        forward_ref_op_assign! {
            $gf, $t;
            AddAssign, add_assign;
            SubAssign, sub_assign;
            MulAssign, mul_assign;
            DivAssign, div_assign;
        }
    };
}

impl_gf!(Gf, u64, u128);
impl_gf!(Gf32, u32, u64);
