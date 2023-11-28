use crate::types::IntExponent;

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Operator {
    Mul,
    Div,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Expr<T, E> {
    Value(Factor<T, E>),
    Binary(BinaryOperator<T, E>),
}

#[cfg(test)]
impl<T, E> Expr<T, E> {
    pub fn value(factor: Factor<T, E>) -> Box<Self> {
        Box::new(Self::Value(factor))
    }

    pub fn binary(bin: BinaryOperator<T, E>) -> Box<Self> {
        Box::new(Self::Binary(bin))
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct BinaryOperator<T, E> {
    pub lhs: Box<Expr<T, E>>,
    pub rhs: Factor<T, E>,
    pub operator: Operator,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Factor<T, E> {
    Value(T),
    Power(T, E),
    ParenExpr(Box<Expr<T, E>>),
}

pub trait MulDiv:
    std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> + Sized + Clone
{
    fn powi(self, pow: i32) -> Self;
}

impl<T, E> Expr<T, E> {
    pub fn map<T2, F>(self, f: F) -> Expr<T2, E>
    where
        F: Fn(T) -> T2 + Clone,
    {
        match self {
            Expr::Value(val) => Expr::Value(val.map(f)),
            Expr::Binary(bin) => {
                let bin = BinaryOperator {
                    lhs: Box::new(bin.lhs.map(f.clone())),
                    rhs: bin.rhs.map(f.clone()),
                    operator: bin.operator,
                };
                Expr::Binary(bin)
            }
        }
    }

    pub fn iter_vals<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        match self {
            Expr::Value(val) => Box::new(val.iter_vals()),
            Expr::Binary(bin) => Box::new(bin.lhs.iter_vals().chain(bin.rhs.iter_vals())),
        }
    }
}

impl<T, E> Factor<T, E> {
    pub fn map<T2, F>(self, f: F) -> Factor<T2, E>
    where
        F: Fn(T) -> T2 + Clone,
    {
        match self {
            Factor::Value(val) => Factor::Value(f(val)),
            Factor::ParenExpr(expr) => Factor::ParenExpr(Box::new(expr.map(f))),
            Factor::Power(val, exponent) => Factor::Power(f(val), exponent),
        }
    }

    pub fn iter_vals<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        match self {
            Factor::Value(val) => Box::new(std::iter::once(val)),
            Factor::ParenExpr(expr) => expr.iter_vals(),
            Factor::Power(val, _) => Box::new(std::iter::once(val)),
        }
    }
}

impl<T: MulDiv, I: Into<IntExponent> + Clone> Expr<T, I> {
    pub fn eval(&self) -> T {
        match self {
            Expr::Value(val) => val.eval(),
            Expr::Binary(bin) => {
                let lhs = bin.lhs.eval();
                let rhs = bin.rhs.eval();
                match bin.operator {
                    Operator::Mul => lhs * rhs,
                    Operator::Div => lhs / rhs,
                }
            }
        }
    }
}

impl<T: MulDiv, I: Into<IntExponent> + Clone> Factor<T, I> {
    pub fn eval(&self) -> T {
        match self {
            Factor::Value(val) => val.clone(),
            Factor::ParenExpr(expr) => expr.eval(),
            Factor::Power(val, exponent) => val.clone().powi(exponent.clone().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::tests::MyInt;
    use quote::quote;

    use super::{super::parse::tests::parse_expr, MulDiv};

    impl std::ops::Mul for MyInt {
        type Output = MyInt;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    impl std::ops::Div for MyInt {
        type Output = MyInt;

        fn div(self, rhs: Self) -> Self::Output {
            Self(self.0 / rhs.0)
        }
    }

    impl MulDiv for MyInt {
        fn powi(self, pow: i32) -> Self {
            Self(self.0.pow(pow as u32))
        }
    }

    impl From<MyInt> for i32 {
        fn from(value: MyInt) -> Self {
            value.0
        }
    }

    #[test]
    fn mul_expr() {
        assert_eq!(parse_expr(quote! { 1 * 2 * 3 * 4 / 2 }).eval(), MyInt(12));
        assert_eq!(parse_expr(quote! { 1 * 3 * 4 / 2 }).eval(), MyInt(6));
        assert_eq!(
            parse_expr(quote! { (1 * 3 * 4) / (2 * 3) }).eval(),
            MyInt(2)
        );
        assert_eq!(parse_expr(quote! { 1 * 2 ^ 3 / 4 }).eval(), MyInt(2));
        assert_eq!(parse_expr(quote! { 1 * 2 ^ 5 / 4^2 }).eval(), MyInt(2));
    }
}
