mod error;
mod item;
mod item_conversion;
mod resolver;

use std::collections::{HashMap, HashSet};

use syn::{Ident, Type};

use crate::types::{Defs, UnresolvedDefs};

use self::{
    error::{Error, Result},
    item::{IdentOrFactor, ResolvedItem, UnresolvedItem, ValueOrExpr},
    item_conversion::ItemConversion,
    resolver::Resolver,
};

/// A helper function for emitting all the errors contained in the
/// result types but continuing with a partial result anyways This is
/// done so that we at least define all the quantities that can be
/// partially resolved in order to keep the amount of error messages
/// manageable.
fn emit_errors<T>((input, result): (T, Result<()>)) -> T {
    if let Err(err) = result {
        err.emit();
    }
    input
}

impl UnresolvedDefs {
    pub fn resolve(self, dimension_type: &Type) -> Defs {
        let items: Vec<UnresolvedItem> = self
            .quantities
            .iter()
            .map(|q| q.to_unresolved_item())
            .chain(self.units.iter().map(|u| u.to_unresolved_item()))
            .chain(self.constants.iter().map(|u| u.to_unresolved_item()))
            .collect();
        let items = emit_errors(filter_undefined_identifiers(items));
        let items = emit_errors(filter_multiply_defined_identifiers(items));
        let mut resolved_items = emit_errors(Resolver::resolve(items));
        let quantities = convert_vec_to_resolved(self.quantities, &mut resolved_items);
        let units = convert_vec_to_resolved(self.units, &mut resolved_items);
        let constants = convert_vec_to_resolved(self.constants, &mut resolved_items);
        Defs {
            dimension_type: dimension_type.clone(),
            quantity_type: self.quantity_type,
            quantities,
            units,
            constants,
        }
    }
}

fn convert_vec_to_resolved<T: ItemConversion>(
    ts: Vec<T>,
    items: &mut HashMap<Ident, ResolvedItem>,
) -> Vec<T::Resolved> {
    ts.into_iter()
        .filter_map(|t| {
            let item = items.remove(t.ident());
            item.map(|item| t.into_resolved(item))
        })
        .collect()
}

fn filter_multiply_defined_identifiers(
    items: Vec<UnresolvedItem>,
) -> (Vec<UnresolvedItem>, Result<()>) {
    let mut counter: HashMap<_, usize> = items.iter().map(|item| (&item.name, 0)).collect();
    for item in items.iter() {
        *counter.get_mut(&item.name).unwrap() += 1;
    }
    let err = if items.iter().any(|item| counter[&item.name] > 1) {
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
    };
    (items, err)
}

fn get_rhs_idents(item: &UnresolvedItem) -> Vec<&Ident> {
    let mut rhs_idents = vec![];
    match &item.val {
        ValueOrExpr::Value(_) => {}
        ValueOrExpr::Expr(expr) => {
            for val in expr.iter_vals() {
                match val {
                    IdentOrFactor::Factor(_) => {}
                    IdentOrFactor::Ident(ident) => {
                        rhs_idents.push(ident);
                    }
                }
            }
        }
    }
    rhs_idents
}

fn filter_undefined_identifiers(items: Vec<UnresolvedItem>) -> (Vec<UnresolvedItem>, Result<()>) {
    let lhs_idents: HashSet<Ident> = items.iter().map(|item| item.name.clone()).collect();
    let (defined, undefined): (Vec<_>, Vec<_>) = items.into_iter().partition(|item| {
        get_rhs_idents(item)
            .iter()
            .all(|rhs_ident| lhs_idents.contains(rhs_ident))
    });

    let err = if undefined.is_empty() {
        Ok(())
    } else {
        let mut undefined_rhs = vec![];
        for item in undefined {
            let rhs_idents = get_rhs_idents(&item);
            for rhs_ident in rhs_idents {
                if !lhs_idents.contains(rhs_ident) {
                    undefined_rhs.push(rhs_ident.clone());
                }
            }
        }
        Err(Error::Undefined(undefined_rhs))
    };
    (defined, err)
}
