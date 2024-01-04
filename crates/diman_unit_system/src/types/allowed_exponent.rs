#[cfg(feature = "rational-dimensions")]
mod reexport {
    #[derive(Clone, PartialEq, Copy)]
    pub struct BaseDimensionExponent {
        pub num: i32,
        pub denom: i32,
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }

    impl BaseDimensionExponent {
        pub fn one() -> BaseDimensionExponent {
            Self { num: 1, denom: 1 }
        }

        pub fn zero() -> BaseDimensionExponent {
            Self { num: 0, denom: 1 }
        }

        pub fn pow(num: f64, exponent: Self) -> f64 {
            num.powf(exponent.num as f64 / exponent.denom as f64)
        }

        fn new(num: i32, denom: i32) -> Self {
            let gcd = gcd(num, denom);
            Self {
                num: num / gcd,
                denom: denom / gcd,
            }
        }
    }

    impl std::fmt::Display for BaseDimensionExponent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.denom == 1 {
                write!(f, "{}", self.num)
            } else {
                write!(f, "{}/{}", self.num, self.denom)
            }
        }
    }

    impl std::ops::Mul for BaseDimensionExponent {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.num * rhs.num, self.denom * rhs.denom)
        }
    }

    impl std::ops::AddAssign for BaseDimensionExponent {
        fn add_assign(&mut self, rhs: Self) {
            let num = self.num * rhs.denom + rhs.num * self.denom;
            let denom = self.denom * rhs.denom;
            *self = Self::new(num, denom)
        }
    }

    impl std::ops::Neg for BaseDimensionExponent {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self {
                num: -self.num,
                denom: self.denom,
            }
        }
    }
}

#[cfg(not(feature = "rational-dimensions"))]
mod reexport {
    #[derive(Clone, PartialEq, Copy)]
    pub struct BaseDimensionExponent(pub i32);

    impl BaseDimensionExponent {
        pub fn one() -> BaseDimensionExponent {
            Self(1)
        }

        pub fn zero() -> BaseDimensionExponent {
            Self(0)
        }

        pub fn pow(num: f64, exponent: Self) -> f64 {
            num.powi(exponent.0)
        }
    }

    impl std::fmt::Display for BaseDimensionExponent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::ops::Mul for BaseDimensionExponent {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    impl std::ops::AddAssign for BaseDimensionExponent {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0
        }
    }

    impl std::ops::Neg for BaseDimensionExponent {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self(-self.0)
        }
    }
}

pub use reexport::BaseDimensionExponent;
