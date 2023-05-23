use syn::*;

use crate::expression::Expr;

#[derive(Debug)]
pub struct Prefix {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DimensionEntry {
    pub ident: Ident,
    pub value: i32,
}

#[derive(Debug, Clone)]
pub struct Dimensions {
    pub fields: Vec<DimensionEntry>,
}

pub type QuantityExpression = Expr<Ident>;

#[derive(Debug)]
pub enum UnitFactor {
    UnitOrQuantity(Ident),
    Number(f64),
}

pub type UnitExpression = Expr<UnitFactor>;

#[derive(Debug)]
pub enum QuantityDefinition {
    Dimensions(Dimensions),
    Expression(QuantityExpression),
}

#[derive(Debug)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<String>,
    pub prefixes: Vec<Prefix>,
    pub rhs: UnitExpression,
}

#[derive(Debug)]
pub struct QuantityEntry {
    pub name: Ident,
    pub rhs: QuantityDefinition,
}

#[derive(Debug)]
pub struct UnresolvedDefs {
    pub dimension_type: Type,
    pub quantity_type: Type,
    pub quantities: Vec<QuantityEntry>,
    pub units: Vec<UnitEntry>,
}

#[derive(Debug)]
pub struct Quantity {
    pub name: Ident,
    pub dimension: Dimensions,
}

#[derive(Debug)]
pub struct Unit {
    pub name: Ident,
    pub dimension: Dimensions,
    pub factor: f64,
    pub symbol: Option<String>,
}

#[derive(Debug)]
pub struct Defs {
    pub dimension_type: Type,
    pub quantity_type: Type,
    pub quantities: Vec<Quantity>,
    pub units: Vec<Unit>,
}
