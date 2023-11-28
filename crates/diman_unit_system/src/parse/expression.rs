use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    *,
};

use crate::expression::Factor;
use crate::expression::{BinaryOperator, Expr, Operator};

impl Parse for Operator {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            Ok(Self::Mul)
        } else if lookahead.peek(Token![/]) {
            let _: Token![/] = input.parse()?;
            Ok(Self::Div)
        } else {
            Err(lookahead.error())
        }
    }
}

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
        let mut lhs = Expr::Value(input.parse()?);
        while {
            let lookahead = input.lookahead1();
            !(input.is_empty() || lookahead.peek(Token![,]))
        } {
            let operator = input.parse()?;
            let rhs = input.parse()?;
            lhs = Expr::Binary(BinaryOperator {
                lhs: Box::new(lhs),
                operator,
                rhs,
            });
        }
        Ok(lhs)
    }
}

#[cfg(test)]
pub mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::expression::BinaryOperator;

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
        use super::Expr::Binary;
        use super::Factor::*;
        use super::Operator::*;
        let x = parse_expr(quote! { 1 });
        assert_eq!(x, Expr::Value(Value(MyInt(1))));
        let x = parse_expr(quote! { 1 * 2 });
        assert_eq!(
            x,
            Binary(BinaryOperator {
                lhs: Expr::value(Value(MyInt(1))),
                operator: Mul,
                rhs: Value(MyInt(2))
            })
        );
        let x = parse_expr(quote! { 1 / 2 });
        assert_eq!(
            x,
            Binary(BinaryOperator {
                lhs: Expr::value(Value(MyInt(1))),
                operator: Div,
                rhs: Value(MyInt(2))
            })
        );
        let x = parse_expr(quote! { 1 / (2 * 3) });
        assert_eq!(
            x,
            Binary(BinaryOperator {
                lhs: Expr::value(Value(MyInt(1))),
                operator: Div,
                rhs: ParenExpr(Expr::binary(BinaryOperator {
                    lhs: Expr::value(Value(MyInt(2))),
                    rhs: Value(MyInt(3)),
                    operator: Mul
                })),
            })
        );
    }

    #[test]
    fn parse_expr_with_multiple_factors() {
        use super::Expr::Binary;
        use super::Factor::*;
        use super::Operator::Mul;
        let x = parse_expr(quote! { 1 * 2 * 3 });
        assert_eq!(
            x,
            Binary(BinaryOperator {
                lhs: Expr::binary(BinaryOperator {
                    operator: Mul,
                    lhs: Expr::value(Value(MyInt(1))),
                    rhs: Value(MyInt(2)),
                }),
                rhs: Value(MyInt(3)),
                operator: Mul,
            })
        );
    }

    #[test]
    fn parse_expr_left_associativity() {
        use super::Expr::Binary;
        use super::Factor::*;
        use super::Operator::{Div, Mul};
        let x = parse_expr(quote! { 1 * 2 / 3 });
        assert_eq!(
            x,
            Binary(BinaryOperator {
                lhs: Expr::binary(BinaryOperator {
                    operator: Mul,
                    lhs: Expr::value(Value(MyInt(1))),
                    rhs: Value(MyInt(2)),
                }),
                rhs: Value(MyInt(3)),
                operator: Div,
            })
        );
    }
}
