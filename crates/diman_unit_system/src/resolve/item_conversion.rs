use syn::Ident;

use crate::{
    dimension_math::DimensionsAndFactor,
    types::{
        Constant, ConstantEntry, DimensionEntry, Dimensions, Quantity, QuantityDefinition,
        QuantityEntry, QuantityIdent, Unit, UnitEntry, UnitFactor,
    },
};

use super::item::{IdentOrFactor, ResolvedItem, UnresolvedItem, ValueOrExpr};

pub trait ItemConversion {
    type Resolved;

    fn to_unresolved_item(&self) -> UnresolvedItem;
    fn into_resolved(self, item: ResolvedItem) -> Self::Resolved;
    fn ident(&self) -> &Ident;
}

impl ItemConversion for QuantityEntry {
    type Resolved = Quantity;

    fn to_unresolved_item(&self) -> UnresolvedItem {
        let val = match &self.rhs {
            QuantityDefinition::Dimensions(dimensions) => ValueOrExpr::Value(DimensionsAndFactor {
                dimensions: dimensions.clone(),
                factor: 1.0,
            }),
            QuantityDefinition::Expression(expr) => {
                ValueOrExpr::Expr(expr.clone().map(|val| match val {
                    QuantityIdent::One => IdentOrFactor::Factor(DimensionsAndFactor {
                        dimensions: Dimensions { fields: vec![] },
                        factor: 1.0,
                    }),
                    QuantityIdent::Quantity(ident) => IdentOrFactor::Ident(ident),
                }))
            }
            QuantityDefinition::Base => ValueOrExpr::Value(DimensionsAndFactor {
                dimensions: Dimensions {
                    fields: vec![DimensionEntry {
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
        Quantity {
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
            UnitFactor::UnitOrQuantity(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: Dimensions::none(),
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
            UnitFactor::UnitOrQuantity(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: Dimensions::none(),
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
