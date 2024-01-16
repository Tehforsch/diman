use std::{
    marker::ConstParamTy,
    ops::{Div, Mul},
};

#[derive(Clone, Copy, PartialEq, Eq, ConstParamTy, Debug)]
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
    pub fn new(val: f64) -> Self {
        let (mantissa, exponent, sign) = integer_decode_f64(val);
        Self {
            mantissa,
            exponent,
            sign,
        }
    }

    pub fn as_f64(self) -> f64 {
        let sign_f = self.sign as f64;
        let mantissa_f = self.mantissa as f64;
        let exponent_f = 2.0f64.powf(self.exponent as f64);

        sign_f * mantissa_f * exponent_f
    }

    pub fn as_f32(self) -> f32 {
        self.as_f64() as f32
    }

    pub fn is_one(&self) -> bool {
        self.as_f64() == 1.0
    }

    pub fn powi(&self, exponent: i64) -> Magnitude {
        Self::new(self.as_f64().powi(exponent as i32))
    }

    pub(crate) fn pow_rational(&self, num: i64, denom: i64) -> Self {
        Self::new(self.as_f64().powf(num as f64 / denom as f64))
    }

    pub const fn mul(self, other: Magnitude) -> Self {
        let m1: u32 = (self.mantissa >> 26) as u32;
        let m2: u32 = (other.mantissa >> 26) as u32;
        let mantissa = (m1 as u64) * (m2 as u64);
        Self {
            mantissa,
            exponent: (self.exponent + 52) + (other.exponent + 52) - 52,
            sign: self.sign * other.sign,
        }
    }

    pub const fn div(self, other: Magnitude) -> Self {
        let m1: u32 = (self.mantissa >> 26) as u32;
        let m2: u32 = (other.mantissa >> 26) as u32;
        let mantissa = (m1 as u64) / (m2 as u64);
        let mantissa = self.mantissa / other.mantissa;
        Self {
            mantissa,
            exponent: (self.exponent + 52) - (other.exponent + 52),
            sign: self.sign * other.sign,
        }
    }
}

impl Mul for Magnitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl Div for Magnitude {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::div(self, rhs)
    }
}

impl Mul<Magnitude> for f64 {
    type Output = Self;
    fn mul(self, rhs: Magnitude) -> Self::Output {
        self * rhs.as_f64()
    }
}

impl Div<Magnitude> for f64 {
    type Output = Self;
    fn div(self, rhs: Magnitude) -> Self::Output {
        self / rhs.as_f64()
    }
}

impl Mul<Magnitude> for f32 {
    type Output = Self;
    fn mul(self, rhs: Magnitude) -> Self::Output {
        self * rhs.as_f32()
    }
}

impl Div<Magnitude> for f32 {
    type Output = Self;
    fn div(self, rhs: Magnitude) -> Self::Output {
        self / rhs.as_f32()
    }
}

#[cfg(test)]
mod tests {
    use crate::magnitude::Magnitude;

    fn operator_test_cases() -> impl Iterator<Item = (f64, f64)> {
        let mut vals = vec![(1.0, 1.0), (1.5, 1.0), (1.0, 1.5), (2.0, 2.0)];
        for exp in -100..100 {
            let x = 2.0f64.powi(exp);
            let y = 2.0f64.powi(-exp);
            vals.push((x, x));
            vals.push((x, x));
            vals.push((x, y));
            vals.push((y, x));
            vals.push((1.1 * x, y));
        }
        vals.into_iter()
    }

    #[test]
    fn magnitude_mul() {
        let check_equality = |x: f64, y: f64| {
            let product = (Magnitude::new(x) * Magnitude::new(y)).as_f64();
            assert_eq!(product, x * y);
        };
        for (x, y) in operator_test_cases() {
            check_equality(x, y);
        }
    }

    #[test]
    fn magnitude_div() {
        let check_equality = |x: f64, y: f64| {
            let product = (Magnitude::new(x) / Magnitude::new(y)).as_f64();
            dbg!(Magnitude::new(x));
            dbg!(Magnitude::new(y));
            dbg!(Magnitude::new(x) / Magnitude::new(y));
            assert_eq!(product, x / y);
        };
        for (x, y) in operator_test_cases() {
            check_equality(x, y);
        }
    }

    #[test]
    fn magnitude_as_f64_round_trip() {
        let check_equality = |x: f64| {
            assert_eq!(Magnitude::new(x).as_f64(), x);
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
