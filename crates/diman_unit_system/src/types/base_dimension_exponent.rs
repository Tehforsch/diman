#[cfg(feature = "rational-dimensions")]
pub use diman_lib::ratio::Ratio as BaseDimensionExponent;

#[cfg(not(feature = "rational-dimensions"))]
pub use reexport::BaseDimensionExponent;

#[cfg(not(feature = "rational-dimensions"))]
mod reexport {
    #[derive(Clone, PartialEq, Copy)]
    pub struct BaseDimensionExponent(pub i64);

    impl BaseDimensionExponent {
        pub fn one() -> BaseDimensionExponent {
            Self(1)
        }

        pub fn zero() -> BaseDimensionExponent {
            Self(0)
        }

        pub fn float_pow(num: f64, exponent: Self) -> f64 {
            num.powi(exponent.0 as i32)
        }
    }

    impl core::fmt::Display for BaseDimensionExponent {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl core::ops::Mul for BaseDimensionExponent {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    impl core::ops::AddAssign for BaseDimensionExponent {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0
        }
    }

    impl core::ops::Neg for BaseDimensionExponent {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self(-self.0)
        }
    }
}
