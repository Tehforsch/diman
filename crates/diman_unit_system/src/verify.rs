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

impl<T: Verify> Verify for Expr<T> {
    type Verified = Expr<<T as Verify>::Verified>;

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

impl<T: Verify> Verify for Factor<T> {
    type Verified = Factor<<T as Verify>::Verified>;

    fn verify(self) -> Result<Self::Verified> {
        Ok(match self {
            Factor::Value(val) => Factor::Value(val.verify()?),
            Factor::ParenExpr(expr) => Factor::ParenExpr(Box::new(expr.verify()?)),
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

impl Verify for ptype::Factor {
    type Verified = f64;

    fn verify(self) -> Result<Self::Verified> {
        match self.0 {
            Lit::Float(s) => Ok(s.base10_parse()?),
            Lit::Int(s) => Ok(s.base10_parse()?),
            _ => Err(Error::new(
                self.0.span(),
                "Unexpected literal, expected a numerical value".to_string(),
            )),
        }
    }
}

impl Verify for ptype::DimensionInt {
    type Verified = i32;

    fn verify(self) -> Result<Self::Verified> {
        match self.0 {
            Lit::Int(s) => Ok(s.base10_parse()?),
            _ => Err(Error::new(
                self.0.span(),
                "Unexpected literal, expected an integer".to_string(),
            )),
        }
    }
}

impl Verify for ptype::Prefix {
    type Verified = Prefix;

    fn verify(self) -> Result<Self::Verified> {
        let name = match self {
            ptype::Prefix::Ident(s) => Ok(s.to_string()),
            ptype::Prefix::Lit(s) => match s {
                Lit::Str(s) => Ok(s.value()),
                _ => Err(Error::new(
                    s.span(),
                    "Unexpected literal, expected a str".to_string(),
                )),
            },
        }?;
        Ok(Prefix { name })
    }
}

impl Verify for ptype::Prefixes {
    type Verified = Vec<Prefix>;

    fn verify(self) -> Result<Self::Verified> {
        self.0.into_iter().map(|x| x.verify()).collect()
    }
}


impl Verify for ptype::QuantityIdent {
    type Verified = QuantityIdent;

    fn verify(self) -> Result<Self::Verified> {
        Ok(match self {
            ptype::QuantityIdent::Factor(factor) => {
                factor_is_one(factor)?;
                QuantityIdent::One
            }
            ptype::QuantityIdent::Quantity(quantity) => QuantityIdent::Quantity(quantity),
        })
    }
}

fn factor_is_one(factor: ptype::Factor) -> Result<()> {
    let val = factor.clone().verify()?;
    if val == 1.0 {
        Ok(())
    } else {
        Err(Error::new(
            factor.0.span(),
            "Only 1 and 1.0 are valid factors in quantity definitions.".to_string(),
        ))
    }
}

