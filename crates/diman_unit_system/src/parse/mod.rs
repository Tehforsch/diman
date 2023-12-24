use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::{self, Paren},
    Attribute, Error, Ident, Lit, Result,
};

use crate::{
    expression::{BinaryOperator, Expr, Factor, Operator},
    types::{Alias, Definition, IntExponent, UnresolvedDefs},
};

use self::tokens::{
    AssignmentToken, DivisionToken, ExponentiationToken, MultiplicationToken, StatementSeparator,
    TypeAnnotationToken,
};

use super::types::{ConstantEntry, DimensionEntry, DimensionFactor, UnitEntry};

pub mod keywords {
    syn::custom_keyword!(quantity_type);
    syn::custom_keyword!(dimension_type);
    syn::custom_keyword!(dimension);
    syn::custom_keyword!(unit);
    syn::custom_keyword!(constant);
}

pub mod tokens {
    syn::custom_punctuation!(DimensionEntryAssignment, :);
    syn::custom_punctuation!(DimensionEntrySeparator, ,);
    syn::custom_punctuation!(DimensionSeparator, ,);
    syn::custom_punctuation!(AssignmentToken, =);
    syn::custom_punctuation!(TypeAnnotationToken, :);
    syn::custom_punctuation!(MultiplicationToken, *);
    syn::custom_punctuation!(DivisionToken, /);
    syn::custom_punctuation!(ExponentiationToken, ^);
    syn::custom_punctuation!(AttributeToken, #);
    syn::custom_punctuation!(StatementSeparator, ,);
    syn::custom_punctuation!(AliasAnnotationToken, :);
}

pub struct Number {
    pub lit: Lit,
    pub float: f64,
}

pub struct Int {
    pub lit: Lit,
    pub int: i32,
}

#[derive(Clone)]
pub struct One;

pub struct Exponent(i32);

pub enum Entry {
    QuantityType(Ident),
    DimensionType(Ident),
    Dimension(DimensionEntry),
    Unit(UnitEntry),
    Constant(ConstantEntry),
}

impl Number {
    fn as_one(&self) -> Result<One> {
        if self.float == 1.0 {
            Ok(One)
        } else {
            Err(Error::new(
                self.lit.span(),
                "Only 1 and 1.0 are valid factors in dimension definitions.".to_string(),
            ))
        }
    }
}

impl Parse for Number {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse()?;
        let float = match lit {
            Lit::Int(ref int) => int.base10_parse::<i64>().map(|x| x as f64),
            Lit::Float(ref float) => float.base10_parse::<f64>(),
            _ => Err(Error::new(
                lit.span(),
                "Unexpected literal, expected a numerical value".to_string(),
            )),
        }?;
        Ok(Self { lit, float })
    }
}

impl Parse for Int {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse()?;
        let int = match lit {
            Lit::Int(ref int) => int.base10_parse::<i32>(),
            _ => Err(Error::new(
                lit.span(),
                "Unexpected literal, expected a numerical value".to_string(),
            )),
        }?;
        Ok(Self { lit, int })
    }
}

impl Parse for One {
    fn parse(input: ParseStream) -> Result<Self> {
        let n: Number = input.parse()?;
        n.as_one()
    }
}

impl Parse for Exponent {
    fn parse(input: ParseStream) -> Result<Self> {
        let int: Int = input.parse()?;
        Ok(Self(int.int))
    }
}

impl Parse for crate::types::Factor<f64> {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::Other(input.parse()?))
        } else if lookahead.peek(Lit) {
            let factor: Number = input.parse()?;
            Ok(Self::Concrete(factor.float))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for DimensionFactor {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Lit) {
            let one: One = input.parse()?;
            Ok(Self::Concrete(one))
        } else {
            Ok(Self::Other(input.parse()?))
        }
    }
}

fn parse_int_exponent_expr<T: Parse>(input: ParseStream) -> Result<Expr<T, IntExponent>> {
    let expr: Expr<T, Exponent> = input.parse()?;
    Ok(expr.map_exp(|e| e.0))
}

impl Parse for Definition<(), One> {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(AssignmentToken) {
            let _: AssignmentToken = input.parse()?;
            Ok(Self::Expression(parse_int_exponent_expr(input)?))
        } else {
            Ok(Self::Base(()))
        }
    }
}

trait ParseWithAttributes: Sized {
    fn parse_with_attributes(input: ParseStream, attributes: Vec<Attribute>) -> Result<Self>;
}

impl ParseWithAttributes for UnitEntry {
    fn parse_with_attributes(input: ParseStream, mut attributes: Vec<Attribute>) -> Result<Self> {
        let _ = input.parse::<keywords::unit>()?;
        let name = input.parse()?;
        let dimension_annotation = parse_annotation(input)?;
        let lookahead = input.lookahead1();
        let base_attributes: Vec<BaseAttribute> = remove_attributes_of_type(&mut attributes);
        let definition = if lookahead.peek(AssignmentToken) {
            let _: AssignmentToken = input.parse()?;
            if base_attributes.is_empty() {
                Ok(Definition::Expression(parse_int_exponent_expr(input)?))
            } else {
                Err(syn::Error::new_spanned(
                    &base_attributes[0].base_ident,
                    format!("Unit declared as base unit, but an expression is given."),
                ))
            }
        } else {
            if base_attributes.len() == 1 {
                Ok(Definition::Base(base_attributes[0].dimension.clone()))
            } else if base_attributes.len() == 0 {
                Err(syn::Error::new_spanned(
                    &name,
                    format!("Unit declared as base unit, but the base dimension is not specified."),
                ))
            } else {
                Err(syn::Error::new_spanned(
                    &base_attributes[0].base_ident,
                    format!("Base dimension is specified multiple times."),
                ))
            }
        }?;
        let aliases = remove_attributes_of_type(&mut attributes);
        Ok(Self {
            name,
            aliases,
            dimension_annotation,
            definition,
        })
    }
}

trait FromAttribute: Sized {
    fn is_correct_ident(ident: &Ident) -> bool;
    fn from_attribute(attribute: &Attribute) -> Option<Self>;
}

fn remove_attributes_of_type<T: FromAttribute>(attributes: &mut Vec<Attribute>) -> Vec<T> {
    //TODO(major): Unwrapping the get_ident here needs to be handled properly. There is probably a helpful method on the attribute itself, i.e. .require_??
    let (ts, others): (Vec<_>, Vec<_>) = attributes
        .drain(..)
        .partition(|a| T::is_correct_ident(&get_ident(a).unwrap()));
    *attributes = others;
    ts.into_iter()
        .map(|t| T::from_attribute(&t).unwrap())
        .collect()
}

fn get_ident(attribute: &Attribute) -> Result<&Ident> {
    let path = attribute.meta.path();
    path.get_ident().ok_or_else(|| {
        syn::Error::new_spanned(
            path.segments.first().unwrap(),
            format!("Expected identifier."),
        )
    })
}

impl FromAttribute for Alias {
    fn is_correct_ident(ident: &Ident) -> bool {
        ident.to_string() == "alias" || ident.to_string() == "short"
    }

    fn from_attribute(attribute: &Attribute) -> Option<Self> {
        //TODO(major): do not unwrap here. In principle, this method should return Result instead of Option. We also need to make sure to check that no attributes are left over at the end
        let type_ = get_ident(attribute).unwrap();
        let short = if type_.to_string() == "alias" {
            false
        } else {
            true
        };
        let name = attribute.parse_args().unwrap();
        Some(Alias { name, short })
    }
}

struct BaseAttribute {
    base_ident: Ident,
    dimension: Ident,
}

impl FromAttribute for BaseAttribute {
    fn is_correct_ident(ident: &Ident) -> bool {
        ident.to_string() == "base"
    }

    fn from_attribute(attribute: &Attribute) -> Option<Self> {
        let args = attribute.parse_args().unwrap();
        Some(Self {
            dimension: args,
            base_ident: get_ident(attribute).unwrap().clone(),
        })
    }
}

fn parse_annotation(input: ParseStream) -> Result<Option<Ident>> {
    let lookahead = input.lookahead1();
    let dimension_annotation = if lookahead.peek(TypeAnnotationToken) {
        let _: TypeAnnotationToken = input.parse()?;
        Some(input.parse()?)
    } else {
        None
    };
    Ok(dimension_annotation)
}

impl Parse for DimensionEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: keywords::dimension = input.parse()?;
        let name = input.parse()?;
        let rhs = input.parse()?;
        Ok(Self { name, rhs })
    }
}

impl Parse for ConstantEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: keywords::constant = input.parse()?;
        let name = input.parse()?;
        let dimension_annotation = parse_annotation(input)?;
        let _: AssignmentToken = input.parse()?;
        let rhs = parse_int_exponent_expr(input)?;
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
        let attributes = input.call(Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::quantity_type) {
            let _: kw::quantity_type = input.parse()?;
            Ok(Self::QuantityType(input.parse()?))
        } else if lookahead.peek(kw::dimension_type) {
            let _: kw::dimension_type = input.parse()?;
            Ok(Self::DimensionType(input.parse()?))
        } else if lookahead.peek(kw::dimension) {
            Ok(Self::Dimension(input.parse()?))
        } else if lookahead.peek(kw::unit) {
            Ok(Self::Unit(UnitEntry::parse_with_attributes(
                input, attributes,
            )?))
        } else if lookahead.peek(kw::constant) {
            Ok(Self::Constant(input.parse()?))
        } else {
            Err(lookahead.error())
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

impl Parse for UnresolvedDefs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut dimensions = vec![];
        let mut units = vec![];
        let mut constants = vec![];
        let mut quantity_types = vec![];
        let mut dimension_types = vec![];
        let pt = input.parse_terminated(Entry::parse, StatementSeparator);
        for item in pt?.into_iter() {
            match item {
                Entry::Dimension(q) => dimensions.push(q),
                Entry::Unit(u) => units.push(u),
                Entry::Constant(c) => constants.push(c),
                Entry::QuantityType(q) => quantity_types.push(q),
                Entry::DimensionType(d) => dimension_types.push(d),
            }
        }
        Ok(Self {
            dimension_types,
            quantity_types,
            dimensions,
            units,
            constants,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::{
        expression::{BinaryOperator, Expr, Factor, Operator},
        parse::Entry,
    };

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

    #[test]
    fn parse_unit_entry() {
        let entry = syn::parse2::<Entry>(quote! {
            #[alias(foo)]
            unit bar = meters
        })
        .unwrap();
        if let Entry::Unit(entry) = entry {
            assert_eq!(entry.name.to_string(), "bar");
            assert_eq!(entry.aliases.len(), 1);
            assert_eq!(entry.aliases[0].name, "foo");
            assert!(!entry.aliases[0].short);
        } else {
            panic!()
        }
        let entry = syn::parse2::<Entry>(quote! {
            #[short(b)]
            unit bar = meters
        })
        .unwrap();
        if let Entry::Unit(entry) = entry {
            assert_eq!(entry.name.to_string(), "bar");
            assert_eq!(entry.aliases.len(), 1);
            assert_eq!(entry.aliases[0].name, "b");
            assert!(entry.aliases[0].short);
        } else {
            panic!()
        }
    }
}
