use syn::*;

use crate::{derive_dimension::to_snakecase, expression::Expr};

#[derive(Clone)]
pub struct Prefix {
    pub name: String,
}

#[derive(Clone)]
pub struct BaseDimensionEntry {
    pub ident: Ident,
    pub value: i32,
}

#[derive(Clone)]
pub struct BaseDimensions {
    pub fields: Vec<BaseDimensionEntry>,
}

#[derive(Clone)]
pub enum DimensionIdent {
    One,
    Dimension(Ident),
}

pub type IntExponent = i32;

pub type DimensionExpression = Expr<DimensionIdent, IntExponent>;

#[derive(Clone)]
pub enum UnitFactor {
    Unit(Ident),
    Number(f64),
}

pub type UnitExpression = Expr<UnitFactor, IntExponent>;

#[derive(Clone)]
pub enum DimensionDefinition {
    Expression(DimensionExpression),
    Base,
}

#[derive(Clone)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<String>,
    pub prefixes: Vec<Prefix>,
    pub rhs: Option<UnitExpression>,
    pub dimension_annotation: Option<Ident>,
}

impl UnitEntry {
    pub fn is_base_unit(&self) -> bool {
        self.rhs.is_none()
    }
}

#[derive(Clone)]
pub struct DimensionEntry {
    pub name: Ident,
    pub rhs: DimensionDefinition,
}

impl DimensionEntry {
    pub fn is_base_dimension(&self) -> bool {
        matches!(self.rhs, DimensionDefinition::Base)
    }

    pub fn dimension_entry_name(&self) -> Ident {
        to_snakecase(&self.name)
    }
}

#[derive(Clone)]
pub struct ConstantEntry {
    pub name: Ident,
    pub rhs: UnitExpression,
    pub dimension_annotation: Option<Ident>,
}

pub struct UnresolvedDefs {
    pub dimension_types: Vec<Ident>,
    pub quantity_types: Vec<Ident>,
    pub dimensions: Vec<DimensionEntry>,
    pub units: Vec<UnitEntry>,
    pub constants: Vec<ConstantEntry>,
}

pub struct Dimension {
    pub name: Ident,
    pub dimension: BaseDimensions,
}

pub struct Unit {
    pub name: Ident,
    pub dimensions: BaseDimensions,
    pub factor: f64,
    pub symbol: Option<String>,
}

pub struct Constant {
    pub name: Ident,
    pub dimension: BaseDimensions,
    pub factor: f64,
}

pub struct Defs {
    pub dimension_type: Ident,
    pub quantity_type: Ident,
    pub dimensions: Vec<Dimension>,
    pub units: Vec<Unit>,
    pub constants: Vec<Constant>,
    pub base_dimensions: Vec<Ident>,
}

impl Defs {
    pub fn base_dimensions(&self) -> impl Iterator<Item = &Ident> + '_ {
        self.base_dimensions.iter()
    }
}
