use core::{
    marker::ConstParamTy,
    ops::{Div, Mul},
};

use crate::num_traits_reexport::*; // either Float or FloatCore

pub const MAX_NUM_FACTORS: usize = 10;

#[derive(Copy, Clone, PartialEq, Eq, Debug, ConstParamTy)]
pub struct Magnitude {
    pub mantissa: u64,
    pub exponent: i16,
    pub sign: i8,
}

// From num-traits
fn integer_decode_f64(f: f64) -> (u64, i16, i8) {
    let bits: u64 = f.to_bits();
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    // Exponent bias + mantissa shift
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

impl Magnitude {
    pub fn from_f64(f: f64) -> Self {
        let (mantissa, exponent, sign) = integer_decode_f64(f);
        Self {
            mantissa,
            exponent,
            sign,
        }
    }

    pub fn into_f64(self) -> f64 {
        self.sign as f64 * self.mantissa as f64 * 2.0f64.powi(self.exponent as i32)
    }

    pub fn into_f32(self) -> f32 {
        self.into_f64() as f32
    }

    #[cfg(any(feature = "std", feature = "num-traits-libm"))]
    pub fn pow_rational(&self, num: i64, denom: i64) -> Magnitude {
        Self::from_f64(self.into_f64().powf(num as f64 / denom as f64))
    }

    pub fn is_one(&self) -> bool {
        self.into_f64() == 1.0
    }
}

impl Mul<Magnitude> for Magnitude {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        Self::from_f64(self.into_f64() * rhs.into_f64())
    }
}

impl Div<Magnitude> for Magnitude {
    type Output = Self;

    fn div(self, rhs: Magnitude) -> Self::Output {
        Self::from_f64(self.into_f64() / rhs.into_f64())
    }
}

impl Mul<f64> for Magnitude {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_f64(self.into_f64() * rhs)
    }
}

impl Div<f64> for Magnitude {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::from_f64(self.into_f64() / rhs)
    }
}

impl Mul<Magnitude> for f64 {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        self * rhs.into_f64()
    }
}

impl Div<Magnitude> for f64 {
    type Output = Self;

    fn div(self, rhs: Magnitude) -> Self::Output {
        self / rhs.into_f64()
    }
}

impl Mul<f32> for Magnitude {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_f64((self.into_f32() * rhs) as f64)
    }
}

impl Div<f32> for Magnitude {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::from_f64((self.into_f32() / rhs) as f64)
    }
}

impl Mul<Magnitude> for f32 {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        self * rhs.into_f32()
    }
}

impl Div<Magnitude> for f32 {
    type Output = Self;

    fn div(self, rhs: Magnitude) -> Self::Output {
        self / rhs.into_f32()
    }
}

#[cfg(test)]
mod tests {
    use crate::magnitude::Magnitude;

    #[test]
    fn magnitude_as_f64_round_trip() {
        let check_equality = |x: f64| {
            assert_eq!(Magnitude::from_f64(x).into_f64(), x);
        };
        for x in 0..10000 {
            let x = (x as f64) * 0.01;
            check_equality(x);
        }
        for exp in -50..50 {
            let x = 2.0f64.powi(exp);
            check_equality(x);
        }
    }
}
