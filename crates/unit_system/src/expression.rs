#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MultiplicativeExpr<T> {
    Factor(Factor<T>),
    Times(Factor<T>, Factor<T>),
    Over(Factor<T>, Factor<T>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Factor<T> {
    Value(T),
    ParenExpr(Box<MultiplicativeExpr<T>>),
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::expression::Factor;

    use super::MultiplicativeExpr;
    use syn::{
        parse::{self, Parse},
        Lit, Result,
    };

    #[derive(Debug, PartialEq, Eq)]
    pub struct MyInt(pub isize);

    impl Parse for MyInt {
        fn parse(input: parse::ParseStream) -> Result<Self> {
            let val: Lit = input.parse()?;
            match val {
                Lit::Int(x) => Ok(MyInt(x.base10_parse().unwrap())),
                _ => panic!(),
            }
        }
    }
    fn parse_expr(input: TokenStream) -> MultiplicativeExpr<MyInt> {
        syn::parse2(input).unwrap()
    }

    #[test]
    fn parse_exprs() {
        let x = parse_expr(quote! { 1 ;});
        assert_eq!(x, MultiplicativeExpr::Factor(Factor::Value(MyInt(1))));
        let x = parse_expr(quote! { 1 });
        assert_eq!(x, MultiplicativeExpr::Factor(Factor::Value(MyInt(1))));
        let x = parse_expr(quote! { 1 * 2 });
        assert_eq!(
            x,
            MultiplicativeExpr::Times(Factor::Value(MyInt(1)), Factor::Value(MyInt(2)))
        );
        let x = parse_expr(quote! { 1 / 2 });
        assert_eq!(
            x,
            MultiplicativeExpr::Over(Factor::Value(MyInt(1)), Factor::Value(MyInt(2)))
        );
        let x = parse_expr(quote! { 1 / (2 * 3) });
        assert_eq!(
            x,
            MultiplicativeExpr::Over(
                Factor::Value(MyInt(1)),
                Factor::ParenExpr(Box::new(MultiplicativeExpr::Times(
                    Factor::Value(MyInt(2)),
                    Factor::Value(MyInt(3))
                )))
            )
        );
    }
}
