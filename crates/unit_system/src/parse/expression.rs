use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    *,
};

use crate::expression::Factor;
use crate::expression::MultiplicativeExpr;

impl<T: Parse + std::fmt::Debug> Parse for Factor<T> {
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

impl<T: Parse + std::fmt::Debug> Parse for MultiplicativeExpr<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let first_factor: Factor<T> = input.parse()?;
        let lookahead = input.lookahead1();
        if input.is_empty() {
            Ok(Self::Factor(first_factor))
        } else if lookahead.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            Ok(Self::Factor(first_factor))
        } else if lookahead.peek(Token![;]) {
            let _: Token![;] = input.parse()?;
            Ok(Self::Factor(first_factor))
        } else if lookahead.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            let second_factor: Factor<T> = input.parse()?;
            Ok(Self::Times(first_factor, second_factor))
        } else if lookahead.peek(Token![/]) {
            let _: Token![/] = input.parse()?;
            let second_factor: Factor<T> = input.parse()?;
            Ok(Self::Over(first_factor, second_factor))
        } else {
            Err(lookahead.error())
        }
    }
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
