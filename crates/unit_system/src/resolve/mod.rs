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
            t.from_resolved_item(item)
        })
        .collect()
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
        Err(Error::undefined(undefined_rhs_idents))
    }
}
