#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Expr<T> {
    Value(Factor<T>),
    Times(Factor<T>, Box<Expr<T>>),
    Over(Factor<T>, Box<Expr<T>>),
}

#[derive(Debug)]
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

impl<T: MulDiv> Expr<T> {
    pub fn eval(&self) -> T {
        match self {
            Expr::Value(val) => val.eval(),
            Expr::Times(val, expr) => val.eval() * expr.eval(),
            Expr::Over(val, expr) => val.eval() / expr.eval(),
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
