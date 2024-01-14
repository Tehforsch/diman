use std::ops::{Div, Mul};

#[derive(Clone, Copy, PartialEq)]
pub struct Magnitude(f64);

impl Magnitude {
    pub fn new(val: f64) -> Self {
        Self(val)
    }

    pub const fn as_f64(self) -> f64 {
        self.0
    }

    pub fn is_one(&self) -> bool {
        self.0 == 1.0
    }

    pub(crate) fn powi(&self, exponent: i32) -> Magnitude {
        Self(self.0.powi(exponent))
    }

    pub(crate) fn pow_rational(&self, num: i64, denom: i64) -> Self {
        Self(self.0.powf(num as f64 / denom as f64))
    }
}

impl Mul for Magnitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Magnitude {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
