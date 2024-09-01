use core::ops::{AddAssign, Mul, Neg};

use crate::magnitude::Magnitude;

// For some reason, this use statement is never recognized as used
// even when I build the crates with no std, where removing the use
// statement means that powi below cannot be used.
#[allow(unused)]
#[cfg(not(any(feature = "std")))]
use num_traits::float::FloatCore;

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
        Magnitude::from_f64(num.into_f64().powi(exponent as i32))
    }

    fn from_int(i: i32) -> Self {
        i as i64
    }
}
