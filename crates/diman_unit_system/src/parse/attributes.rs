use proc_macro2::Span;
use syn::{
    bracketed, parenthesized,
    parse::{ParseBuffer, ParseStream},
    Error, Result,
};

use crate::{
    parse::tokens,
    types::{Alias, BaseAttribute},
};

pub mod attribute_keywords {
    syn::custom_keyword!(base);
    syn::custom_keyword!(alias);
    syn::custom_keyword!(symbol);
    syn::custom_keyword!(metric_prefixes);
}

pub enum AttributeName {
    Base,
    Alias,
    Symbol,
    MetricPrefixes,
}

pub struct Attribute<'a> {
    name: AttributeName,
    span: Span,
    inner: Option<ParseBuffer<'a>>,
}

pub trait FromAttribute: Sized {
    fn is_correct_type(name: &AttributeName) -> bool;
    fn from_attribute(attribute: &Attribute) -> Result<Self>;
}

pub trait ParseWithAttributes: Sized {
    fn parse_with_attributes(input: ParseStream, attributes: Vec<Attribute>) -> Result<Self>;
}

impl<'a> Attribute<'a> {
    pub fn parse_all(input: ParseStream<'a>) -> Result<Vec<Self>> {
        use attribute_keywords as attr_kw;
        let mut attributes = vec![];
        while input.peek(tokens::AttributeToken) {
            let _: tokens::AttributeToken = input.parse()?;
            let content;
            let _ = bracketed!(content in input);
            let span = content.span().clone();
            let lookahead = content.lookahead1();
            let name = if lookahead.peek(attr_kw::base) {
                let _: attr_kw::base = content.parse()?;
                AttributeName::Base
            } else if lookahead.peek(attr_kw::alias) {
                let _: attr_kw::alias = content.parse()?;
                AttributeName::Alias
            } else if lookahead.peek(attr_kw::symbol) {
                let _: attr_kw::symbol = content.parse()?;
                AttributeName::Symbol
            } else if lookahead.peek(attr_kw::metric_prefixes) {
                let _: attr_kw::metric_prefixes = content.parse()?;
                AttributeName::MetricPrefixes
            } else {
                return Err(lookahead.error());
            };
            let inner = if content.peek(syn::token::Paren) {
                let inner;
                let _ = parenthesized!(inner in content);
                Some(inner)
            } else {
                None
            };
            attributes.push(Attribute { span, name, inner });
        }
        Ok(attributes)
    }

    fn inner_or_err(&self) -> Result<&ParseBuffer> {
        self.inner
            .as_ref()
            .ok_or_else(|| Error::new(self.span, format!("Attribute expects arguments.")))
    }
}

pub fn remove_attributes_of_type<T: FromAttribute>(
    attributes: &mut Vec<Attribute>,
) -> Result<Vec<T>> {
    let (ts, others): (Vec<_>, Vec<_>) = attributes
        .drain(..)
        .partition(|a| T::is_correct_type(&a.name));
    *attributes = others;
    ts.into_iter().map(|t| T::from_attribute(&t)).collect()
}

impl FromAttribute for Alias {
    fn is_correct_type(name: &AttributeName) -> bool {
        matches!(name, AttributeName::Alias | AttributeName::Symbol)
    }

    fn from_attribute(attribute: &Attribute) -> Result<Self> {
        let symbol = matches!(attribute.name, AttributeName::Symbol);
        let inner = attribute.inner_or_err()?;
        let name = inner.parse()?;
        Ok(Alias { name, symbol })
    }
}

impl FromAttribute for BaseAttribute {
    fn is_correct_type(name: &AttributeName) -> bool {
        matches!(name, AttributeName::Base)
    }

    fn from_attribute(attribute: &Attribute) -> Result<Self> {
        let dimension = attribute.inner_or_err()?.parse()?;
        Ok(Self {
            dimension,
            attribute_span: attribute.span,
        })
    }
}
