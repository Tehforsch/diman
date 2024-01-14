use std::ops::{AddAssign, Mul, Neg};

pub trait BaseDimensionExponent: Clone + PartialEq + Copy + Mul + AddAssign + Neg {
    fn float_pow(num: f64, exponent: Self) -> f64;
    fn one() -> Self;
    fn zero() -> Self;
}

impl BaseDimensionExponent for i64 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    fn float_pow(num: f64, exponent: Self) -> f64 {
        num.powi(exponent as i32)
    }
}
