use syn::*;

use crate::expression::Expr;

pub struct Prefix {
    pub name: String,
}

#[derive(Clone)]
pub struct DimensionEntry {
    pub ident: Ident,
    pub value: i32,
}

#[derive(Clone)]
pub struct Dimensions {
    pub fields: Vec<DimensionEntry>,
}

pub type QuantityExpression = Expr<Ident>;

#[derive(Clone)]
pub enum UnitFactor {
    UnitOrQuantity(Ident),
    Number(f64),
}

pub type UnitExpression = Expr<UnitFactor>;

pub enum QuantityDefinition {
    Dimensions(Dimensions),
    Expression(QuantityExpression),
}

pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<String>,
    pub prefixes: Vec<Prefix>,
    pub rhs: UnitExpression,
}

pub struct QuantityEntry {
    pub name: Ident,
    pub rhs: QuantityDefinition,
}

pub struct ConstantEntry {
    pub name: Ident,
    pub rhs: UnitExpression,
}

pub struct UnresolvedDefs {
    pub dimension_type: Type,
    pub quantity_type: Type,
    pub quantities: Vec<QuantityEntry>,
    pub units: Vec<UnitEntry>,
    pub constants: Vec<ConstantEntry>,
}

pub struct Quantity {
    pub name: Ident,
    pub dimension: Dimensions,
}

pub struct Unit {
    pub name: Ident,
    pub dimension: Dimensions,
    pub factor: f64,
    pub symbol: Option<String>,
}

pub struct Constant {
    pub name: Ident,
    pub dimension: Dimensions,
    pub factor: f64,
}

pub struct Defs {
    pub dimension_type: Type,
    pub quantity_type: Type,
    pub quantities: Vec<Quantity>,
    pub units: Vec<Unit>,
    pub constants: Vec<Constant>,
}
