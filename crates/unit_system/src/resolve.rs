use std::{collections::HashMap, hash::Hash, fmt::Display};

use syn::Ident;

use crate::{
    dimension_math::DimensionsAndFactor,
    expression::Expr,
    types::{
        Dimensions, Quantity, QuantityDefinition, QuantityEntry, Defs, Unit, UnitEntry,
        UnitFactor, UnresolvedDefs, Constant,
    },
};

impl UnresolvedDefs {
    pub fn resolve(self) -> Result<Defs, UnresolvableError<Ident>> {
        let items: Vec<UnresolvedItem> = self
            .quantities
            .into_iter()
            .map(|q| q.into())
            .chain(self.units.into_iter().map(|u| u.into()))
            .collect();
        let items = Resolver::resolve(items)?;
        let (quantities, units): (Vec<_>, Vec<_>) = items
            .into_iter()
            .partition(|x| matches!(x.type_, Type::Quantity));
        let constants = self.constants.into_iter().map(|constant| {
            // Very inefficient, but hopefully irrelevant for now
            let unit = units.iter().find(|unit| unit.name == constant.unit).ok_or_else(|| UnresolvableError(vec![constant.unit]))?;
            Ok(Constant {
                name: constant.name,
                dimension: unit.val.dimensions.clone(),
                factor: unit.val.factor,
            })
        }).collect::<Result<_, _>>()?;
        Ok(Defs {
            dimension_type: self.dimension_type,
            quantity_type: self.quantity_type,
            quantities: quantities.into_iter().map(|i| i.into()).collect(),
            units: units.into_iter().map(|i| i.into()).collect(),
            constants,
        })
    }
}

enum Type {
    Unit(Option<String>),
    Quantity,
}

enum IdentOrFactor {
    Factor(DimensionsAndFactor),
    Ident(Ident),
}

enum ValueOrExpr {
    Value(DimensionsAndFactor),
    Expr(Expr<IdentOrFactor>),
}

struct UnresolvedItem {
    type_: Type,
    name: Ident,
    val: ValueOrExpr,
}

struct ResolvedItem {
    type_: Type,
    name: Ident,
    val: DimensionsAndFactor,
}

impl From<QuantityEntry> for UnresolvedItem {
    fn from(q: QuantityEntry) -> Self {
        let val = match q.rhs {
            QuantityDefinition::Dimensions(dimensions) => ValueOrExpr::Value(DimensionsAndFactor {
                dimensions,
                factor: 1.0,
            }),
            QuantityDefinition::Expression(expr) => {
                ValueOrExpr::Expr(expr.map(|x| IdentOrFactor::Ident(x)))
            }
        };
        Self {
            type_: Type::Quantity,
            name: q.name,
            val,
        }
    }
}

impl From<UnitEntry> for UnresolvedItem {
    fn from(q: UnitEntry) -> Self {
        let val = ValueOrExpr::Expr(q.rhs.map(|x| match x {
            UnitFactor::UnitOrQuantity(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: Dimensions::none(),
            }),
        }));
        Self {
            type_: Type::Unit(q.symbol),
            name: q.name,
            val,
        }
    }
}

impl From<ResolvedItem> for Quantity {
    fn from(q: ResolvedItem) -> Self {
        Quantity {
            name: q.name,
            dimension: q.val.dimensions,
        }
    }
}

impl From<ResolvedItem> for Unit {
    fn from(q: ResolvedItem) -> Self {
        let symbol = match q.type_ {
            Type::Unit(symbol) => symbol,
            Type::Quantity => unreachable!(),
        };
        Unit {
            name: q.name,
            dimension: q.val.dimensions,
            factor: q.val.factor,
            symbol: symbol,
        }
    }
}

trait Resolvable {
    type Resolved;
    type Name: Hash + PartialEq + Eq;

    fn resolve(self, others: &HashMap<Self::Name, Self::Resolved>) -> Self::Resolved;
    fn is_resolvable(&self, others: &HashMap<Self::Name, Self::Resolved>) -> bool;
    fn name(&self) -> Self::Name;
}

impl Resolvable for UnresolvedItem {
    type Resolved = ResolvedItem;
    type Name = Ident;

    fn name(&self) -> Self::Name {
        self.name.clone()
    }

    fn resolve(self, others: &HashMap<Ident, ResolvedItem>) -> Self::Resolved {
        match self.val {
            ValueOrExpr::Value(val) => ResolvedItem {
                type_: self.type_,
                name: self.name,
                val,
            },
            ValueOrExpr::Expr(expr) => {
                let val = expr
                    .map(|val_or_expr| match val_or_expr {
                        IdentOrFactor::Factor(factor) => factor,
                        IdentOrFactor::Ident(ident) => others[&ident].val.clone(),
                    })
                    .eval();
                ResolvedItem {
                    type_: self.type_,
                    name: self.name,
                    val,
                }
            }
        }
    }

    fn is_resolvable(&self, others: &HashMap<Ident, ResolvedItem>) -> bool {
        match &self.val {
            ValueOrExpr::Value(_) => true,
            ValueOrExpr::Expr(expr) => expr.iter_vals().all(|val| match val {
                IdentOrFactor::Factor(_) => true,
                IdentOrFactor::Ident(ident) => others.contains_key(ident),
            }),
        }
    }
}

struct Resolver<U, R, N> {
    unresolved: Vec<U>,
    resolved: HashMap<N, R>,
}

impl<U: Resolvable<Resolved = R, Name = N>, R, N: Hash + PartialEq + Eq> Resolver<U, R, N> {
    fn resolve_with(
        unresolved: Vec<U>,
        resolved: HashMap<N, R>,
    ) -> Result<Vec<R>, UnresolvableError<N>> {
        let mut resolver = Self {
            unresolved,
            resolved,
        };
        resolver.run()?;
        Ok(resolver.resolved.into_iter().map(|(_, x)| x).collect())
    }

    fn resolve(unresolved: Vec<U>) -> Result<Vec<R>, UnresolvableError<N>> {
        Self::resolve_with(unresolved, HashMap::new())
    }

    fn run(&mut self) -> Result<(), UnresolvableError<N>> {
        while !self.unresolved.is_empty() {
            let next_resolvable = self
                .unresolved
                .iter()
                .enumerate()
                .find(|(_, x)| self.resolvable(x));
            if let Some((index, _)) = next_resolvable {
                let next_resolvable = self.unresolved.remove(index);
                let name = next_resolvable.name();
                let resolved = next_resolvable.resolve(&self.resolved);
                self.resolved.insert(name, resolved);
            } else {
                return Err(UnresolvableError(self.unresolved.drain(..).map(|x| x.name()).collect()));
            }
        }
        Ok(())
    }

    fn resolvable(&self, u: &U) -> bool {
        u.is_resolvable(&self.resolved)
    }
}

pub struct UnresolvableError<N>(Vec<N>);

impl Display for UnresolvableError<Ident> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ident in self.0.iter() {
            ident.span().unwrap().error(format!("Unresolvable definition: \"{}\"", ident)).emit();
        }
        write!(f, "Unable to resolve. Unresolvable definitions: {}", self.0.iter().map(|x| format!("\"{}\"", x.to_string())).collect::<Vec<_>>().join(", "))
    }
}
