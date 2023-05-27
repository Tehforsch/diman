use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    *,
};

use crate::expression::Expr;
use crate::expression::Factor;

impl<T: Parse> Parse for Factor<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Paren) {
            let content;
            let _: token::Paren = parenthesized!(content in input);
            Ok(Self::ParenExpr(Box::new(content.parse()?)))
        } else {
            Ok(Self::Value(input.parse()?))
        }
    }
}

impl<T: Parse> Parse for Expr<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let first_factor: Factor<T> = input.parse()?;
        let lookahead = input.lookahead1();
        if input.is_empty() || lookahead.peek(Token![,]) {
            Ok(Self::Value(first_factor))
        } else if lookahead.peek(Token![*]) {
            let _: Token![*] = input.parse().unwrap();
            let second_expr: Expr<T> = input.parse()?;
            Ok(Self::Times(first_factor, Box::new(second_expr)))
        } else if lookahead.peek(Token![/]) {
            let _: Token![/] = input.parse().unwrap();
            let second_expr: Expr<T> = input.parse()?;
            Ok(Self::Over(first_factor, Box::new(second_expr)))
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use super::Expr;
    use syn::{
        parse::{self, Parse},
        Lit, Result,
    };

    #[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn parse_expr(input: TokenStream) -> Expr<MyInt> {
        syn::parse2(input).unwrap()
    }

    #[test]
    fn parse_exprs() {
        use super::Expr::{Over, Times};
        use super::Factor::*;
        let x = parse_expr(quote! { 1 });
        assert_eq!(x, Expr::Value(Value(MyInt(1))));
        let x = parse_expr(quote! { 1 * 2 });
        assert_eq!(
            x,
            Times(Value(MyInt(1)), Box::new(Expr::Value(Value(MyInt(2)))))
        );
        let x = parse_expr(quote! { 1 / 2 });
        assert_eq!(
            x,
            Over(Value(MyInt(1)), Box::new(Expr::Value(Value(MyInt(2)))))
        );
        let x = parse_expr(quote! { 1 / (2 * 3) });
        assert_eq!(
            x,
            Over(
                Value(MyInt(1)),
                Box::new(Expr::Value(ParenExpr(Box::new(Times(
                    Value(MyInt(2)),
                    Box::new(Expr::Value(Value(MyInt(3)))),
                )))))
            )
        );
    }

    #[test]
    fn parse_expr_with_multiple_factors() {
        use super::Expr::Times;
        use super::Factor::*;
        let x = parse_expr(quote! { 1 * 2 * 3 });
        assert_eq!(
            x,
            Times(
                Value(MyInt(1)),
                Box::new(Times(
                    Value(MyInt(2)),
                    Box::new(Expr::Value(Value(MyInt(3))))
                ))
            )
        );
    }
}
