use syn::{*, punctuated::Punctuated};

use crate::expression::MultiplicativeExpr;

#[derive(Debug)]
pub enum Prefix {
    Ident(Ident),
    Lit(Lit),
}

#[derive(Debug)]
pub struct Prefixes(pub Punctuated<Prefix, Token![,]>);

#[derive(Debug)]
pub struct DimensionEntry {
    pub ident: Ident,
    pub value: Lit,
}

#[derive(Debug)]
pub struct Dimensions {
    pub fields: Vec<DimensionEntry>,
}


#[derive(Debug)]
pub enum QuantityFactor {
    Quantity(Ident),
    Number(Lit),
}

pub type QuantityExpression = MultiplicativeExpr<QuantityFactor>;

#[derive(Debug)]
pub enum UnitFactor {
    UnitOrQuantity(Ident),
    Number(Lit),
}

pub type UnitExpression = MultiplicativeExpr<UnitFactor>;

#[derive(Debug)]
pub enum QuantityDefinition {
    Dimensions(Dimensions),
    Expression(QuantityExpression),
}

#[derive(Debug)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<Lit>,
    pub prefixes: Prefixes,
    pub rhs: UnitExpression,
}

#[derive(Debug)]
pub struct QuantityEntry {
    pub name: Ident,
    pub rhs: QuantityDefinition,
}

#[derive(Debug)]
pub enum QuantityOrUnit {
    Quantity(QuantityEntry),
    Unit(UnitEntry),
}

#[derive(Debug)]
pub struct Defs {
    pub dimension_type: Type,
    pub quantity_type: Type,
    pub quantities: Vec<QuantityEntry>,
    pub units: Vec<UnitEntry>,
}
