use proc_macro2::Span;
use syn::*;

use crate::{
    derive_dimension::to_snakecase, dimension_math::BaseDimensions, expression::Expr, parse::One,
    prefixes::Prefix,
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

pub struct BaseAttribute {
    pub attribute_span: Span,
    pub dimension: Ident,
}

#[derive(Clone)]
pub struct Alias {
    pub name: Ident,
}

#[derive(Clone)]
pub struct Symbol(pub Ident);

#[derive(Clone)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<Symbol>,
    pub aliases: Vec<Alias>,
    pub prefixes: Vec<Prefix>,
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

#[derive(Clone)]
pub struct UnitTemplate {
    pub name: Ident,
    pub dimensions: BaseDimensions,
    pub factor: f64,
    pub symbol: Option<Symbol>,
    pub aliases: Vec<Alias>,
    pub prefixes: Vec<Prefix>,
}

#[derive(Clone)]
pub struct Unit {
    pub name: Ident,
    pub dimensions: BaseDimensions,
    pub factor: f64,
    pub symbol: Option<Symbol>,
}

impl UnitTemplate {
    fn expand_prefix_and_alias(&self, prefix: Option<&Prefix>, alias: Option<&Alias>) -> Unit {
        let name = match alias {
            None => &self.name,
            Some(alias) => &alias.name,
        };
        let (name, symbol, factor) = match prefix {
            Some(prefix) => {
                let name = Ident::new(&format!("{}{}", prefix.name(), name), name.span());
                let symbol = self.symbol.as_ref().map(|symbol| {
                    Symbol(Ident::new(
                        &format!("{}{}", prefix.short(), symbol.0),
                        self.name.span(),
                    ))
                });
                let factor = self.factor * prefix.factor();
                (name, symbol, factor)
            }
            None => (name.clone(), self.symbol.clone(), self.factor),
        };
        Unit {
            name,
            dimensions: self.dimensions.clone(),
            factor,
            symbol,
        }
    }

    fn expand(mut self) -> Vec<Unit> {
        let mut prefixes: Vec<_> = self.prefixes.drain(..).map(|prefix| Some(prefix)).collect();
        prefixes.push(None);
        let mut aliases: Vec<_> = self.aliases.drain(..).map(|alias| Some(alias)).collect();
        aliases.push(None);
        prefixes
            .iter()
            .flat_map(|prefix| {
                aliases
                    .iter()
                    .map(|alias| self.expand_prefix_and_alias(prefix.as_ref(), alias.as_ref()))
            })
            .collect()
    }
}

pub struct Constant {
    pub name: Ident,
    pub dimensions: BaseDimensions,
    pub factor: f64,
}

pub struct GenericDefs<U> {
    pub dimension_type: Ident,
    pub quantity_type: Ident,
    pub dimensions: Vec<Dimension>,
    pub units: Vec<U>,
    pub constants: Vec<Constant>,
    pub base_dimensions: Vec<Ident>,
}

pub type TemplateDefs = GenericDefs<UnitTemplate>;
pub type Defs = GenericDefs<Unit>;

impl Defs {
    pub fn base_dimensions(&self) -> impl Iterator<Item = &Ident> + '_ {
        self.base_dimensions.iter()
    }
}

impl TemplateDefs {
    pub fn expand_templates(self) -> Defs {
        let units = self
            .units
            .into_iter()
            .flat_map(|template| template.expand())
            .collect();
        Defs {
            dimension_type: self.dimension_type,
            quantity_type: self.quantity_type,
            dimensions: self.dimensions,
            units,
            constants: self.constants,
            base_dimensions: self.base_dimensions,
        }
    }
}
