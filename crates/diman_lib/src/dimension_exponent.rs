use std::ops::{AddAssign, Mul, Neg};

use crate::magnitude::Magnitude;

pub trait DimensionExponent: Clone + PartialEq + Copy + Mul + AddAssign + Neg {
    fn float_pow(mag: Magnitude, exponent: Self) -> Magnitude;
    fn one() -> Self;
    fn zero() -> Self;
    fn from_int(i: i32) -> Self;
}

impl DimensionExponent for i64 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    fn float_pow(num: Magnitude, exponent: Self) -> Magnitude {
        num.powi(exponent as i32)
    }

    fn from_int(i: i32) -> Self {
        i as i64
    }
}
