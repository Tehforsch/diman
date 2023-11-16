mod error;
mod item;
mod item_conversion;
mod resolver;

use std::collections::{HashMap, HashSet};

use syn::Ident;

use crate::types::{Defs, UnresolvedDefs};

use self::{
    error::{Error, Result},
    item::{IdentOrFactor, ResolvedItem, UnresolvedItem, ValueOrExpr},
    item_conversion::ItemConversion,
    resolver::Resolver,
};

impl UnresolvedDefs {
    pub fn resolve(self) -> Result<Defs> {
        let items: Vec<UnresolvedItem> = self
            .quantities
            .iter()
            .map(|q| q.to_unresolved_item())
            .chain(self.units.iter().map(|u| u.to_unresolved_item()))
            .chain(self.constants.iter().map(|u| u.to_unresolved_item()))
            .collect();
        check_no_undefined_identifiers(&items)?;
        check_no_multiply_defined_identifiers(&items)?;
        let mut items = Resolver::resolve(items)?;
        let quantities = convert_vec_to_resolved(self.quantities, &mut items);
        let units = convert_vec_to_resolved(self.units, &mut items);
        let constants = convert_vec_to_resolved(self.constants, &mut items);
        Ok(Defs {
            dimension_type: self.dimension_type,
            quantity_type: self.quantity_type,
            quantities,
            units,
            constants,
        })
    }
}

fn convert_vec_to_resolved<T: ItemConversion>(
    ts: Vec<T>,
    items: &mut HashMap<Ident, ResolvedItem>,
) -> Vec<T::Resolved> {
    ts.into_iter()
        .map(|t| {
            let item = items.remove(t.ident()).unwrap();
            t.into_resolved(item)
        })
        .collect()
}

fn check_no_multiply_defined_identifiers(items: &[UnresolvedItem]) -> Result<()> {
    let mut counter: HashMap<_, usize> = items.iter().map(|item| (&item.name, 0)).collect();
    for item in items.iter() {
        *counter.get_mut(&item.name).unwrap() += 1;
    }
    if items.as_ref().iter().any(|item| counter[&item.name] > 1) {
        Err(Error::Multiple(
            counter
                .iter()
                .filter(|(_, count)| **count > 1)
                .map(|(multiply_defined_name, _)| {
                    items
                        .iter()
                        .map(|item| item.name.clone())
                        .filter(|name| &name == multiply_defined_name)
                        .collect()
                })
                .collect(),
        ))
    } else {
        Ok(())
    }
}

fn check_no_undefined_identifiers(items: &[UnresolvedItem]) -> Result<()> {
    let lhs_idents: HashSet<Ident> = items.iter().map(|item| item.name.clone()).collect();
    let mut undefined_rhs_idents = vec![];
    for item in items.iter() {
        match &item.val {
            ValueOrExpr::Value(_) => {}
            ValueOrExpr::Expr(expr) => {
                for val in expr.iter_vals() {
                    match val {
                        IdentOrFactor::Factor(_) => {}
                        IdentOrFactor::Ident(ident) => {
                            if !lhs_idents.contains(ident) {
                                undefined_rhs_idents.push(ident.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    if undefined_rhs_idents.is_empty() {
        Ok(())
    } else {
        Err(Error::Undefined(undefined_rhs_idents))
    }
}
