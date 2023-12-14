use diman_derive_verify::Verify;
use syn::{punctuated::Punctuated, *};

use super::tokens::PrefixSeparator;
use crate::expression::Expr;

pub enum Prefix {
    Ident(Ident),
    Lit(Lit),
}

pub struct Prefixes(pub Punctuated<Prefix, PrefixSeparator>);

pub struct DimensionInt(pub Lit);

#[derive(Clone)]
pub struct LitFactor(pub Lit);

pub struct Symbol(pub Lit);

pub struct Exponent(pub Lit);

pub enum DimensionIdent {
    // This will be verified to only be 1.0 or 1
    One(LitFactor),
    Dimension(Ident),
}

pub type DimensionExpression = Expr<DimensionIdent, Exponent>;

#[derive(Verify)]
#[verified(crate::types::UnitFactor)]
pub enum UnitFactor {
    Unit(Ident),
    Number(LitFactor),
}

pub type UnitExpression = Expr<UnitFactor, Exponent>;

#[derive(Verify)]
#[verified(crate::types::DimensionDefinition)]
pub enum DimensionDefinition {
    Expression(DimensionExpression),
    Base,
}

#[derive(Verify)]
#[verified(crate::types::UnitEntry)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<Symbol>,
    pub prefixes: Prefixes,
    pub rhs: Option<UnitExpression>,
    pub dimension_annotation: Option<Ident>,
}

#[derive(Verify)]
#[verified(crate::types::DimensionEntry)]
pub struct DimensionEntry {
    pub name: Ident,
    pub rhs: DimensionDefinition,
}

#[derive(Verify)]
#[verified(crate::types::ConstantEntry)]
pub struct ConstantEntry {
    pub name: Ident,
    pub rhs: UnitExpression,
    pub dimension_annotation: Option<Ident>,
}

pub enum Entry {
    QuantityType(Ident),
    DimensionType(Ident),
    Dimension(DimensionEntry),
    Unit(UnitEntry),
    Constant(ConstantEntry),
}

#[derive(Verify)]
#[verified(crate::types::UnresolvedDefs)]
pub struct Defs {
    pub dimension_types: Vec<Ident>,
    pub quantity_types: Vec<Ident>,
    pub dimensions: Vec<DimensionEntry>,
    pub units: Vec<UnitEntry>,
    pub constants: Vec<ConstantEntry>,
}
