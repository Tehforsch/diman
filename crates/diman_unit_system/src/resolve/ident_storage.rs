use std::collections::HashMap;

use proc_macro2::Ident;

use crate::{
    dimension_math::{BaseDimensions, DimensionsAndFactor},
    expression::{self, Expr},
    parse::Symbol,
    types::{
        Constant, ConstantEntry, Dimension, DimensionDefinition, DimensionEntry, DimensionFactor,
        IntExponent, Unit, UnitDefinition, UnitEntry, UnitFactor,
    },
};

use super::error::UnresolvableError;

/// The kind of an identifier
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Dimension,
    Unit,
    Constant,
}

#[derive(Clone)]
pub struct Item {
    pub ident: Ident,
    pub kind: Kind,
    pub annotation: Option<Ident>,
    pub expr: Expr<Factor<DimensionsAndFactor>, IntExponent>,
    pub symbol: Option<Symbol>,
}

pub struct ResolvedItem {
    pub item: Item,
    pub dimensions: DimensionsAndFactor,
}

#[derive(Clone)]
pub enum Factor<D> {
    Concrete(D),
    Other(Ident),
}

#[derive(Default)]
pub struct IdentStorage {
    unresolved: HashMap<Ident, Item>,
    resolved: HashMap<Ident, ResolvedItem>,
}

impl IdentStorage {
    fn is_resolvable(&self, item: &Item) -> bool {
        let all_factors_concrete_or_given = (&item.expr).iter_vals().all(|val| match val {
            Factor::Concrete(_) => true,
            Factor::Other(ident) => self.resolved.contains_key(&ident),
        });
        all_factors_concrete_or_given
    }

    fn resolve_dimensions(&self, item: &Item) -> DimensionsAndFactor {
        item.expr
            .clone()
            .map(|val_or_expr| match val_or_expr {
                Factor::Concrete(factor) => factor,
                Factor::Other(ident) => self.resolved[&ident].dimensions.clone(),
            })
            .eval()
    }

    pub fn add<I: Into<Item>>(&mut self, items: Vec<I>) {
        self.unresolved.extend(items.into_iter().map(|x| {
            let item = x.into();
            let ident = item.ident.clone();
            (ident, item)
        }));
    }

    pub fn resolve(&mut self) -> Result<(), UnresolvableError> {
        // This is a very inefficient topological sort.
        while !self.unresolved.is_empty() {
            let next_resolvable = self
                .unresolved
                .iter()
                .find(|(_, x)| self.is_resolvable(x))
                .map(|(ident, _)| ident.clone());
            if let Some(ident) = next_resolvable {
                let next_resolvable = self.unresolved.remove(&ident).unwrap();
                let name = ident.clone();
                let dimensions = self.resolve_dimensions(&next_resolvable);
                self.resolved.insert(
                    name,
                    ResolvedItem {
                        dimensions,
                        item: next_resolvable,
                    },
                );
            } else {
                return Err(UnresolvableError(
                    self.unresolved.drain().map(|(ident, _)| ident).collect(),
                ));
            }
        }
        Ok(())
    }

    pub fn get_items<I: FromItem>(&self) -> Vec<I> {
        self.resolved
            .values()
            .filter(|resolved| resolved.item.kind == I::kind())
            .map(|resolved| {
                I::from_item_and_dimensions(resolved.item.clone(), resolved.dimensions.clone())
            })
            .collect()
    }
}

impl From<DimensionEntry> for Item {
    fn from(entry: DimensionEntry) -> Self {
        Item {
            ident: entry.name.clone(),
            annotation: None,
            symbol: None,
            kind: Kind::Dimension,
            expr: match &entry.rhs {
                DimensionDefinition::Expression(expr) => expr.clone().map(|e| match e {
                    DimensionFactor::One => {
                        Factor::Concrete(DimensionsAndFactor::dimensions(BaseDimensions::none()))
                    }
                    DimensionFactor::Dimension(ident) => Factor::Other(ident),
                }),
                DimensionDefinition::Base => {
                    let mut fields = HashMap::default();
                    fields.insert(entry.dimension_entry_name(), 1);
                    Expr::Value(expression::Factor::Value(Factor::Concrete(
                        DimensionsAndFactor::dimensions(BaseDimensions { fields }),
                    )))
                }
            },
        }
    }
}

impl From<UnitEntry> for Item {
    fn from(entry: UnitEntry) -> Self {
        Item {
            ident: entry.name.clone(),
            annotation: None,
            kind: Kind::Unit,
            symbol: entry.symbol,
            expr: match &entry.definition {
                UnitDefinition::Expression(rhs) => rhs.clone().map(|e| match e {
                    UnitFactor::Unit(ident) => Factor::Other(ident),
                    UnitFactor::Number(num) => Factor::Concrete(DimensionsAndFactor::factor(num)),
                }),
                UnitDefinition::Base(dimension) => {
                    Expr::Value(expression::Factor::Value(Factor::Other(dimension.clone())))
                }
            },
        }
    }
}

impl From<ConstantEntry> for Item {
    fn from(entry: ConstantEntry) -> Self {
        Item {
            ident: entry.name.clone(),
            annotation: None,
            kind: Kind::Constant,
            symbol: None,
            expr: entry.rhs.clone().map(|e| match e {
                UnitFactor::Unit(ident) => Factor::Other(ident),
                UnitFactor::Number(num) => Factor::Concrete(DimensionsAndFactor::factor(num)),
            }),
        }
    }
}

pub trait FromItem {
    fn kind() -> Kind;
    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self;
}

impl FromItem for Dimension {
    fn kind() -> Kind {
        Kind::Dimension
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self {
        Dimension {
            dimensions: dimensions.dimensions,
            name: item.ident,
        }
    }
}

impl FromItem for Unit {
    fn kind() -> Kind {
        Kind::Unit
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self {
        Unit {
            dimensions: dimensions.dimensions,
            name: item.ident,
            factor: dimensions.factor,
            symbol: item.symbol,
        }
    }
}

impl FromItem for Constant {
    fn kind() -> Kind {
        Kind::Constant
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self {
        Constant {
            dimensions: dimensions.dimensions,
            name: item.ident,
            factor: dimensions.factor,
        }
    }
}
