use syn::Ident;

use crate::{
    dimension_math::DimensionsAndFactor,
    types::{
        BaseDimensionEntry, BaseDimensions, Constant, ConstantEntry, Dimension,
        DimensionDefinition, DimensionEntry, DimensionIdent, Unit, UnitEntry, UnitFactor,
    },
};

use super::item::{IdentOrFactor, ResolvedItem, UnresolvedItem, ValueOrExpr};

pub trait ItemConversion {
    type Resolved;

    fn to_unresolved_item(&self) -> UnresolvedItem;
    fn into_resolved(self, item: ResolvedItem) -> Self::Resolved;
    fn ident(&self) -> &Ident;
}

impl ItemConversion for DimensionEntry {
    type Resolved = Dimension;

    fn to_unresolved_item(&self) -> UnresolvedItem {
        let val = match &self.rhs {
            DimensionDefinition::BaseDimensions(dimensions) => {
                ValueOrExpr::Value(DimensionsAndFactor {
                    dimensions: dimensions.clone(),
                    factor: 1.0,
                })
            }
            DimensionDefinition::Expression(expr) => {
                ValueOrExpr::Expr(expr.clone().map(|val| match val {
                    DimensionIdent::One => IdentOrFactor::Factor(DimensionsAndFactor {
                        dimensions: BaseDimensions { fields: vec![] },
                        factor: 1.0,
                    }),
                    DimensionIdent::Dimension(ident) => IdentOrFactor::Ident(ident),
                }))
            }
            DimensionDefinition::Base => ValueOrExpr::Value(DimensionsAndFactor {
                dimensions: BaseDimensions {
                    fields: vec![BaseDimensionEntry {
                        ident: self.dimension_entry_name(),
                        value: 1,
                    }],
                },
                factor: 1.0,
            }),
        };
        UnresolvedItem {
            name: self.name.clone(),
            val,
        }
    }

    fn into_resolved(self, item: ResolvedItem) -> Self::Resolved {
        Dimension {
            name: self.name,
            dimension: item.val.dimensions,
        }
    }

    fn ident(&self) -> &Ident {
        &self.name
    }
}

impl ItemConversion for UnitEntry {
    type Resolved = Unit;

    fn to_unresolved_item(&self) -> UnresolvedItem {
        let val = ValueOrExpr::Expr(self.rhs.clone().map(|x| match x {
            UnitFactor::UnitOrDimension(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: BaseDimensions::none(),
            }),
        }));
        UnresolvedItem {
            name: self.name.clone(),
            val,
        }
    }

    fn into_resolved(self, item: ResolvedItem) -> Self::Resolved {
        Unit {
            name: self.name,
            dimension: item.val.dimensions,
            factor: item.val.factor,
            symbol: self.symbol,
        }
    }

    fn ident(&self) -> &Ident {
        &self.name
    }
}

impl ItemConversion for ConstantEntry {
    type Resolved = Constant;

    fn to_unresolved_item(&self) -> UnresolvedItem {
        let val = ValueOrExpr::Expr(self.rhs.clone().map(|x| match x {
            UnitFactor::UnitOrDimension(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: BaseDimensions::none(),
            }),
        }));
        UnresolvedItem {
            name: self.name.clone(),
            val,
        }
    }

    fn into_resolved(self, item: ResolvedItem) -> Self::Resolved {
        Constant {
            name: self.name,
            dimension: item.val.dimensions,
            factor: item.val.factor,
        }
    }

    fn ident(&self) -> &Ident {
        &self.name
    }
}
