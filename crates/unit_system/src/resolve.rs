use std::{collections::HashMap, hash::Hash};

use syn::Ident;

use crate::types::{
    Quantity, QuantityDefinition, QuantityEntry, ResolvedDefs, Unit, UnitEntry, UnresolvedDefs};

impl UnresolvedDefs {
    pub fn resolve(self) -> Result<ResolvedDefs, UnresolvableError> {
        let quantities = Resolver::resolve(self.quantities)?;
        dbg!(&quantities);
        let units = todo!();
        Ok(ResolvedDefs {
            dimension_type: self.dimension_type,
            quantity_type: self.quantity_type,
            quantities,
            units,
        })
    }
}

trait Resolvable {
    type Resolved;
    type Name: Hash + PartialEq + Eq;

    fn resolve(self, others: &HashMap<Self::Name, Self::Resolved>) -> Self::Resolved;
    fn is_resolvable(&self, others: &HashMap<Self::Name, Self::Resolved>) -> bool;
    fn name(&self) -> Self::Name;
}

impl Resolvable for QuantityEntry {
    type Resolved = Quantity;
    type Name = Ident;

    fn name(&self) -> Self::Name {
        self.name.clone()
    }

    fn resolve(self, others: &HashMap<Ident, Quantity>) -> Self::Resolved {
        match self.rhs {
            QuantityDefinition::Dimensions(dim) => Quantity {
                name: self.name,
                dimension: dim,
            },
            QuantityDefinition::Expression(expr) => {
                Quantity {
                    name: self.name,
                    dimension: expr.map(|ident| others[&ident].dimension.clone()).eval(),
                }
            },
        }
    }

    fn is_resolvable(&self, others: &HashMap<Ident, Quantity>) -> bool {
        match &self.rhs {
            QuantityDefinition::Dimensions(_) => true,
            QuantityDefinition::Expression(expr) => {
                expr.iter_vals().all(|val| others.contains_key(val))
            }
        }
    }
}

struct Resolver<U, R, N> {
    unresolved: Vec<U>,
    resolved: HashMap<N, R>,
}

impl<U: Resolvable<Resolved = R, Name = N>, R, N: Hash + PartialEq + Eq> Resolver<U, R, N> {
    fn resolve(unresolved: Vec<U>) -> Result<Vec<R>, UnresolvableError> {
        let mut resolver = Self {
            unresolved,
            resolved: HashMap::new(),
        };
        resolver.run()?;
        Ok(resolver.resolved.into_iter().map(|(_, x)| x).collect())
    }

    fn run(&mut self) -> Result<(), UnresolvableError> {
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
            }
            else {
                return Err(UnresolvableError)
            }
        }
        Ok(())
    }

    fn resolvable(&self, u: &U) -> bool {
        u.is_resolvable(&self.resolved)
    }
}

pub struct UnresolvableError;
