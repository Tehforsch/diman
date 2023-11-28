#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Operator {
    Mul,
    Div,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Expr<T> {
    Value(Factor<T>),
    Binary(BinaryOperator<T>),
}

#[cfg(test)]
impl<T> Expr<T> {
    pub fn value(factor: Factor<T>) -> Box<Self> {
        Box::new(Self::Value(factor))
    }

    pub fn binary(bin: BinaryOperator<T>) -> Box<Self> {
        Box::new(Self::Binary(bin))
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct BinaryOperator<T> {
    pub lhs: Box<Expr<T>>,
    pub rhs: Factor<T>,
    pub operator: Operator,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Factor<T> {
    Value(T),
    ParenExpr(Box<Expr<T>>),
}

pub trait MulDiv:
    std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> + Sized + Clone
{
}

impl<T> MulDiv for T where
    T: std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> + Sized + Clone
{
}

impl<T> Expr<T> {
    pub fn map<U, F>(self, f: F) -> Expr<U>
    where
        F: Fn(T) -> U + Clone,
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

impl<T> Factor<T> {
    pub fn map<U, F>(self, f: F) -> Factor<U>
    where
        F: Fn(T) -> U + Clone,
    {
        match self {
            Factor::Value(val) => Factor::Value(f(val)),
            Factor::ParenExpr(expr) => Factor::ParenExpr(Box::new(expr.map(f))),
        }
    }
    pub fn iter_vals<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        match self {
            Factor::Value(val) => Box::new(std::iter::once(val)),
            Factor::ParenExpr(expr) => expr.iter_vals(),
        }
    }
}

impl<T: MulDiv> Expr<T> {
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

impl<T: MulDiv> Factor<T> {
    pub fn eval(&self) -> T {
        match self {
            Factor::Value(val) => val.clone(),
            Factor::ParenExpr(expr) => expr.eval(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::expression::tests::MyInt;
    use quote::quote;

    use super::super::parse::expression::tests::parse_expr;

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

    #[test]
    fn mul_expr() {
        assert_eq!(parse_expr(quote! { 1 * 2 * 3 * 4 / 2 }).eval(), MyInt(12));
        assert_eq!(parse_expr(quote! { 1 * 3 * 4 / 2 }).eval(), MyInt(6));
        assert_eq!(
            parse_expr(quote! { (1 * 3 * 4) / (2 * 3) }).eval(),
            MyInt(2)
        );
    }
}
