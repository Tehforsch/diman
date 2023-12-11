pub mod types;

use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Brace, Bracket, Paren},
    Error, Ident, Lit, Result,
};

use crate::expression::{BinaryOperator, Expr, Factor, Operator};

use self::{
    tokens::{
        AssignmentToken, DimensionEntryAssignment, DimensionEntrySeparator, DivisionToken,
        ExponentiationToken, MultiplicationToken, StatementSeparator, TypeAnnotationToken,
        UnitDefDelimiter, UnitDefSeparator,
    },
    types::{
        ConstantEntry, Defs, DimensionEntry, DimensionInt, Dimensions, Entry, Exponent, LitFactor,
        Prefix, Prefixes, QuantityDefinition, QuantityEntry, QuantityIdent, Symbol, UnitEntry,
        UnitExpression, UnitFactor,
    },
};

pub mod keywords {
    syn::custom_keyword!(dimension);
    syn::custom_keyword!(unit);
    syn::custom_keyword!(quantity_type);
    syn::custom_keyword!(dimension_type);
    syn::custom_keyword!(constant);
}

pub mod tokens {
    pub type UnitDefDelimiter = syn::token::Paren;
    syn::custom_punctuation!(DimensionEntryAssignment, :);
    syn::custom_punctuation!(DimensionEntrySeparator, ,);
    syn::custom_punctuation!(DimensionSeparator, ,);
    syn::custom_punctuation!(UnitDefSeparator, ,);
    syn::custom_punctuation!(AssignmentToken, =);
    syn::custom_punctuation!(TypeAnnotationToken, :);
    syn::custom_punctuation!(PrefixSeparator, ,);
    syn::custom_punctuation!(MultiplicationToken, *);
    syn::custom_punctuation!(DivisionToken, /);
    syn::custom_punctuation!(ExponentiationToken, ^);
    syn::custom_punctuation!(StatementSeparator, ,);
}

impl Parse for Symbol {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for LitFactor {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for DimensionInt {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for Exponent {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for Prefix {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if lookahead.peek(Lit) {
            Ok(Self::Lit(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for Prefixes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _: Bracket = bracketed!( content in input );
        Ok(Prefixes(content.parse_terminated(Prefix::parse)?))
    }
}

impl Parse for DimensionEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let _: DimensionEntryAssignment = input.parse()?;
        let value: DimensionInt = input.parse()?;
        Ok(Self { ident, value })
    }
}

impl Parse for UnitFactor {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::UnitOrQuantity(input.parse()?))
        } else if lookahead.peek(Lit) {
            Ok(Self::Number(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for QuantityIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Lit) {
            Ok(Self::Factor(input.parse()?))
        } else {
            Ok(Self::Quantity(input.parse()?))
        }
    }
}

impl Parse for QuantityDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(AssignmentToken) {
            let _: AssignmentToken = input.parse()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(Brace) {
                Ok(Self::Dimensions(input.parse()?))
            } else {
                Ok(Self::Expression(input.parse()?))
            }
        } else {
            Ok(Self::Base)
        }
    }
}

impl Parse for UnitEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let name;
        let symbol;
        let mut prefixes = Prefixes(Punctuated::new());
        if lookahead.peek(Ident) {
            name = input.parse()?;
            symbol = None;
        } else if lookahead.peek(Paren) {
            let content;
            let _: UnitDefDelimiter = parenthesized! { content in input };
            name = content.parse()?;
            let _: UnitDefSeparator = content.parse()?;
            symbol = Some(content.parse()?);
            let lookahead = content.lookahead1();
            if lookahead.peek(UnitDefSeparator) {
                let _: UnitDefSeparator = content.parse()?;
                prefixes = content.parse()?;
            } else if !content.is_empty() {
                return Err(lookahead.error());
            }
        } else {
            return Err(lookahead.error());
        }
        let (rhs, dimension_annotation) = parse_unit_rhs_and_annotation(input)?;
        Ok(Self {
            name,
            symbol,
            prefixes,
            rhs,
            dimension_annotation,
        })
    }
}

fn parse_unit_rhs_and_annotation(input: ParseStream) -> Result<(UnitExpression, Option<Ident>)> {
    let lookahead = input.lookahead1();
    let dimension_annotation = if lookahead.peek(TypeAnnotationToken) {
        let _: TypeAnnotationToken = input.parse()?;
        Some(input.parse()?)
    } else {
        None
    };
    let _: AssignmentToken = input.parse()?;
    let rhs: UnitExpression = input.parse()?;
    Ok((rhs, dimension_annotation))
}

impl Parse for Dimensions {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _: token::Brace = braced!(content in input);
        let fields: Punctuated<DimensionEntry, DimensionEntrySeparator> =
            content.parse_terminated(DimensionEntry::parse)?;
        Ok(Self {
            fields: fields.into_iter().collect(),
        })
    }
}

impl Parse for QuantityEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let rhs = input.parse()?;
        Ok(Self { name, rhs })
    }
}

impl Parse for ConstantEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let (rhs, dimension_annotation) = parse_unit_rhs_and_annotation(input)?;
        Ok(Self {
            name,
            rhs,
            dimension_annotation,
        })
    }
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> Result<Self> {
        use keywords as kw;
        if input.peek(kw::quantity_type) {
            let _ = input.parse::<kw::quantity_type>()?;
            Ok(Self::QuantityType(input.parse()?))
        } else if input.peek(kw::dimension_type) {
            let _ = input.parse::<kw::dimension_type>()?;
            Ok(Self::DimensionType(input.parse()?))
        } else if input.peek(kw::dimension) {
            let _ = input.parse::<kw::dimension>()?;
            Ok(Self::Quantity(input.parse()?))
        } else if input.peek(kw::unit) {
            let _ = input.parse::<kw::unit>()?;
            Ok(Self::Unit(input.parse()?))
        } else if input.peek(kw::constant) {
            let _ = input.parse::<kw::constant>()?;
            Ok(Self::Constant(input.parse()?))
        } else {
            Err(Error::new(
                input.span(),
                format!("Unexpected token. Expected \"def\", \"unit\" or \"constant\"",),
            ))
        }
    }
}

impl Parse for Operator {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(MultiplicationToken) {
            let _: MultiplicationToken = input.parse()?;
            Ok(Self::Mul)
        } else if lookahead.peek(DivisionToken) {
            let _: DivisionToken = input.parse()?;
            Ok(Self::Div)
        } else {
            Err(lookahead.error())
        }
    }
}

impl<T: Parse, E: Parse> Parse for Factor<T, E> {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Paren) {
            let content;
            let _: token::Paren = parenthesized!(content in input);
            return Ok(Self::ParenExpr(Box::new(content.parse()?)));
        }
        let val = input.parse()?;
        if input.peek(ExponentiationToken) {
            let _: ExponentiationToken = input.parse()?;
            let exponent: E = input.parse()?;
            Ok(Self::Power(val, exponent))
        } else {
            Ok(Self::Value(val))
        }
    }
}

impl<T: Parse, E: Parse> Parse for Expr<T, E> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut lhs = Expr::Value(input.parse()?);
        while {
            let lookahead = input.lookahead1();
            !(input.is_empty() || lookahead.peek(StatementSeparator))
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

impl Parse for Defs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut quantities = vec![];
        let mut units = vec![];
        let mut constants = vec![];
        let mut quantity_types = vec![];
        let mut dimension_types = vec![];
        for item in input
            .parse_terminated::<_, StatementSeparator>(Entry::parse)?
            .into_iter()
        {
            match item {
                Entry::Quantity(q) => quantities.push(q),
                Entry::Unit(u) => units.push(u),
                Entry::Constant(c) => constants.push(c),
                Entry::QuantityType(q) => quantity_types.push(q),
                Entry::DimensionType(d) => dimension_types.push(d),
            }
        }
        Ok(Self {
            dimension_types,
            quantity_types,
            quantities,
            units,
            constants,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::expression::{BinaryOperator, Expr, Factor, Operator};

    use syn::{
        parse::{self, Parse},
        Lit, Result,
    };

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MyInt(pub i32);

    impl Parse for MyInt {
        fn parse(input: parse::ParseStream) -> Result<Self> {
            let val: Lit = input.parse()?;
            match val {
                Lit::Int(x) => Ok(MyInt(x.base10_parse().unwrap())),
                _ => panic!(),
            }
        }
    }

    pub fn parse_expr(input: TokenStream) -> Expr<MyInt, MyInt> {
        syn::parse2(input).unwrap()
    }

    #[test]
    fn parse_exprs() {
        use Expr::Binary;
        use Factor::*;
        use Operator::*;
        let x = parse_expr(quote! { 1 });
        assert_eq!(x, Expr::Value(Factor::Value(MyInt(1))));
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
        use Expr::Binary;
        use Factor::*;
        use Operator::Mul;
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
        use Expr::Binary;
        use Factor::*;
        use Operator::{Div, Mul};
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

    #[test]
    fn parse_expr_exponent() {
        use Factor::*;
        use Operator::{Div, Mul};
        let x = parse_expr(quote! { 1 ^ 2 });
        assert_eq!(x, Expr::Value(Power(MyInt(1), MyInt(2))),);
        let x = parse_expr(quote! { 1 * 2 ^ 3 / 4 });
        assert_eq!(
            x,
            Expr::Binary(BinaryOperator {
                lhs: Expr::binary(BinaryOperator {
                    lhs: Expr::value(Value(MyInt(1))),
                    rhs: Power(MyInt(2), MyInt(3)),
                    operator: Mul,
                }),
                rhs: Value(MyInt(4)),
                operator: Div,
            }),
        );
    }
}
