use syn::*;

use crate::{
    derive_dimension::to_snakecase,
    dimension_math::BaseDimensions,
    expression::Expr,
    parse::{One, Symbol},
};

pub type IntExponent = i32;

#[derive(Clone)]
pub enum Factor<C> {
    Concrete(C),
    Other(Ident),
}

#[derive(Clone)]
pub enum Definition<Base, C> {
    Base(Base),
    Expression(Expr<Factor<C>, IntExponent>),
}

pub type DimensionFactor = Factor<One>;
pub type DimensionDefinition = Definition<(), One>;
pub type UnitFactor = Factor<f64>;
pub type UnitExpression = Expr<UnitFactor, IntExponent>;
pub type UnitDefinition = Definition<Ident, f64>;

#[derive(Clone)]
pub struct DimensionEntry {
    pub name: Ident,
    pub rhs: DimensionDefinition,
}

impl DimensionEntry {
    pub fn is_base_dimension(&self) -> bool {
        matches!(self.rhs, DimensionDefinition::Base(()))
    }

    pub fn dimension_entry_name(&self) -> Ident {
        to_snakecase(&self.name)
    }
}

#[derive(Clone)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<Symbol>,
    pub dimension_annotation: Option<Ident>,
    pub definition: UnitDefinition,
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
    pub dimensions: BaseDimensions,
}

pub struct Unit {
    pub name: Ident,
    pub dimensions: BaseDimensions,
    pub factor: f64,
    pub symbol: Option<Symbol>,
}

pub struct Constant {
    pub name: Ident,
    pub dimensions: BaseDimensions,
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
