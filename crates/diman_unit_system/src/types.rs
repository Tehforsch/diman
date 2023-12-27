use syn::*;

use crate::{
    derive_dimension::to_snakecase, dimension_math::BaseDimensions, expression::Expr, parse::One,
};

pub type IntExponent = i32;

#[derive(Clone)]
pub enum Factor<C> {
    Concrete(C),
    Other(Ident),
}

impl<C1: Clone> Factor<C1> {
    pub fn map_concrete<C2>(&self, f: impl Fn(C1) -> C2) -> Factor<C2> {
        match self {
            Factor::Concrete(c1) => Factor::Concrete(f(c1.clone())),
            Factor::Other(x) => Factor::Other(x.clone()),
        }
    }
}

#[derive(Clone)]
pub enum Definition<Base, C> {
    Base(Base),
    Expression(Expr<Factor<C>, IntExponent>),
}

pub type DimensionFactor = Factor<One>;

#[derive(Clone)]
pub struct DimensionEntry {
    pub name: Ident,
    pub rhs: Definition<(), One>,
}

impl DimensionEntry {
    pub fn is_base_dimension(&self) -> bool {
        matches!(self.rhs, Definition::Base(()))
    }

    pub fn dimension_entry_name(&self) -> Ident {
        to_snakecase(&self.name)
    }
}

#[derive(Clone)]
pub struct Alias {
    pub name: Ident,
    pub symbol: bool,
}

#[derive(Clone)]
pub struct UnitEntry {
    pub name: Ident,
    pub aliases: Vec<Alias>,
    pub dimension_annotation: Option<Ident>,
    pub definition: Definition<Ident, f64>,
}

#[derive(Clone)]
pub struct ConstantEntry {
    pub name: Ident,
    pub rhs: Expr<Factor<f64>, IntExponent>,
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
    pub aliases: Vec<Alias>,
}

impl Unit {
    pub fn symbol(&self) -> Option<&Ident> {
        self.aliases
            .iter()
            .filter(|alias| alias.symbol)
            .map(|alias| &alias.name)
            .next()
    }
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
