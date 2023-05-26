use crate::parse::types as ptype;
use crate::types::*;
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
            Expr::Times(val, expr) => Expr::Times(val.verify()?, Box::new(expr.verify()?)),
            Expr::Over(val, expr) => Expr::Over(val.verify()?, Box::new(expr.verify()?)),
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

// impl Verify for ptype::Prefixes {
//     type Verified = Vec<Prefix>;

//     fn verify(self) -> Result<Self::Verified> {
//         Ok(self.0.into_iter().map(|x| x.verify()).collect::<Result<_>>()?)
//     }
// }

// impl Verify for ptype::QuantityEntry {
//     type Verified = QuantityEntry;

//     fn verify(self) -> Result<Self::Verified> {
//         Ok(QuantityEntry {
//             name: self.name,
//             rhs: self.rhs.verify(),
//         })
//     }
// }

// impl Verify for ptype::UnitEntry {
//     type Verified = UnitEntry;

//     fn verify(self) -> Result<Self::Verified> {
//         Ok(UnitEntry {
//             name: self.name,
//             rhs: self.rhs.verify(),
//             symbol: self.symbol.map(|x| verify_symbol(x)),
//             prefixes: self.prefixes.verify(),
//         })
//     }
// }

// impl Verify for ptype::Defs {
//     type Verified = Defs;

//     fn verify(self) -> Result<Self::Verified> {
//         Ok(Defs {
//             dimension_type: self.dimension_type,
//             quantity_type: self.quantity_type,
//             quantities: verify_vec(self.quantities)?,
//             units: verify_vec(self.units)?,
//         })
//     }
// }
