use crate::types::*;
use crate::{expression::BinaryOperator, parse::types as ptype};
use syn::{Error, Lit, Result};

use crate::expression::{Expr, Factor};

pub trait Verify: Sized {
    type Verified;

    fn verify(self) -> Result<Self::Verified>;
}

impl<T: Verify> Verify for Vec<T> {
    type Verified = Vec<<T as Verify>::Verified>;
    fn verify(self) -> Result<Self::Verified> {
        self.into_iter().map(|x| x.verify()).collect()
    }
}

impl<T: Verify> Verify for Option<T> {
    type Verified = Option<<T as Verify>::Verified>;
    fn verify(self) -> Result<Self::Verified> {
        self.map(|x| x.verify()).transpose()
    }
}

macro_rules! verify_endpoint {
    ($t: ty) => {
        impl Verify for $t {
            type Verified = $t;
            fn verify(self) -> Result<Self::Verified> {
                Ok(self)
            }
        }
    };
}

verify_endpoint!(syn::Type);
verify_endpoint!(syn::Ident);
verify_endpoint!(f64);

impl<T: Verify, E: Verify> Verify for Expr<T, E> {
    type Verified = Expr<<T as Verify>::Verified, <E as Verify>::Verified>;

    fn verify(self) -> Result<Self::Verified> {
        Ok(match self {
            Expr::Value(val) => Expr::Value(val.verify()?),
            Expr::Binary(bin) => Expr::Binary(BinaryOperator {
                lhs: Box::new(bin.lhs.verify()?),
                rhs: bin.rhs.verify()?,
                operator: bin.operator,
            }),
        })
    }
}

impl<T: Verify, E: Verify> Verify for Factor<T, E> {
    type Verified = Factor<<T as Verify>::Verified, <E as Verify>::Verified>;

    fn verify(self) -> Result<Self::Verified> {
        Ok(match self {
            Factor::Value(val) => Factor::Value(val.verify()?),
            Factor::ParenExpr(expr) => Factor::ParenExpr(Box::new(expr.verify()?)),
            Factor::Power(val, exponent) => Factor::Power(val.verify()?, exponent.verify()?),
        })
    }
}

impl Verify for ptype::Symbol {
    type Verified = String;

    fn verify(self) -> Result<Self::Verified> {
        match self.0 {
            Lit::Str(s) => Ok(s.value()),
            _ => Err(Error::new(
                self.0.span(),
                "Unexpected literal, expected a str".to_string(),
            )),
        }
    }
}

impl Verify for ptype::Exponent {
    type Verified = IntExponent;

    fn verify(self) -> Result<Self::Verified> {
        match self.0 {
            Lit::Int(s) => Ok(s.base10_parse()?),
            _ => Err(Error::new(
                self.0.span(),
                "Unexpected literal, expected an integer value".to_string(),
            )),
        }
    }
}
