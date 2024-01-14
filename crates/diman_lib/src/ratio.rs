use crate::{dimension_exponent::DimensionExponent, magnitude::Magnitude};

#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::marker::ConstParamTy,
)]
pub struct Ratio {
    num: i64,
    denom: i64,
}

const fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

impl Ratio {
    pub const fn int(num: i64) -> Self {
        Self { num, denom: 1 }
    }

    pub const fn num(&self) -> i64 {
        self.num
    }

    pub const fn denom(&self) -> i64 {
        self.denom
    }

    pub const fn new(num: i64, denom: i64) -> Self {
        let gcd = gcd(num, denom);
        Self {
            num: num / gcd,
            denom: denom / gcd,
        }
    }

    pub const fn powi(self, exp: i32) -> Self {
        let num = self.num * exp as i64;
        let denom = self.denom * exp as i64;
        Self::new(num, denom)
    }

    pub const fn add(self, rhs: Self) -> Self {
        let num = self.num * rhs.denom + rhs.num * self.denom;
        let denom = self.denom * rhs.denom;
        Self::new(num, denom)
    }

    pub const fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }

    pub const fn neg(self) -> Self {
        Self {
            num: -self.num,
            denom: self.denom,
        }
    }

    pub const fn mul(self, rhs: Self) -> Self {
        let num = self.num * rhs.num;
        let denom = self.denom * rhs.denom;
        Self::new(num, denom)
    }

    pub const fn div(self, rhs: Self) -> Self {
        self.mul(rhs.inv())
    }

    const fn inv(self) -> Self {
        Self {
            num: self.denom,
            denom: self.num,
        }
    }
}

impl DimensionExponent for Ratio {
    fn one() -> Self {
        Self { num: 1, denom: 1 }
    }

    fn zero() -> Self {
        Self { num: 0, denom: 1 }
    }

    fn float_pow(num: Magnitude, exponent: Self) -> Magnitude {
        num.pow_rational(exponent.num, exponent.denom)
    }

    fn from_int(i: i32) -> Self {
        Ratio::int(i as i64)
    }
}

impl core::fmt::Display for Ratio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.denom)
        }
    }
}

impl core::ops::Mul for Ratio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.denom * rhs.denom)
    }
}

impl core::ops::AddAssign for Ratio {
    fn add_assign(&mut self, rhs: Self) {
        let num = self.num * rhs.denom + rhs.num * self.denom;
        let denom = self.denom * rhs.denom;
        *self = Self::new(num, denom)
    }
}

impl core::ops::Neg for Ratio {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            num: -self.num,
            denom: self.denom,
        }
    }
}
