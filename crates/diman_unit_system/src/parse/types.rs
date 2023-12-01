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

#[derive(Verify)]
#[verified(crate::types::DimensionEntry)]
pub struct DimensionEntry {
    pub ident: Ident,
    pub value: DimensionInt,
}

#[derive(Verify)]
#[verified(crate::types::Dimensions)]
pub struct Dimensions {
    pub fields: Vec<DimensionEntry>,
}

pub enum QuantityIdent {
    // This will be verified to only be 1.0 or 1
    Factor(LitFactor),
    Quantity(Ident),
}

pub type QuantityExpression = Expr<QuantityIdent, Exponent>;

#[derive(Verify)]
#[verified(crate::types::UnitFactor)]
pub enum UnitFactor {
    UnitOrQuantity(Ident),
    Number(LitFactor),
}

pub type UnitExpression = Expr<UnitFactor, Exponent>;

#[derive(Verify)]
#[verified(crate::types::QuantityDefinition)]
pub enum QuantityDefinition {
    Dimensions(Dimensions),
    Expression(QuantityExpression),
}

#[derive(Verify)]
#[verified(crate::types::UnitEntry)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<Symbol>,
    pub prefixes: Prefixes,
    pub rhs: UnitExpression,
}

#[derive(Verify)]
#[verified(crate::types::QuantityEntry)]
pub struct QuantityEntry {
    pub name: Ident,
    pub rhs: QuantityDefinition,
}

pub type DimensionEntry2 = Ident;

#[derive(Verify)]
#[verified(crate::types::ConstantEntry)]
pub struct ConstantEntry {
    pub name: Ident,
    pub rhs: UnitExpression,
}

pub enum Entry {
    Dimension(DimensionEntry2),
    QuantityType(Ident),
    DimensionType(Ident),
    Quantity(QuantityEntry),
    Unit(UnitEntry),
    Constant(ConstantEntry),
}

#[derive(Verify)]
#[verified(crate::types::UnresolvedDefs)]
pub struct Defs {
    pub dimension_types: Vec<Ident>,
    pub quantity_types: Vec<Ident>,
    pub dimensions: Vec<DimensionEntry2>,
    pub quantities: Vec<QuantityEntry>,
    pub units: Vec<UnitEntry>,
    pub constants: Vec<ConstantEntry>,
}
