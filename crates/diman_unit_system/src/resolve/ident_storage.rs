use std::collections::HashMap;

use proc_macro2::Ident;

use super::{
    emit_errors,
    error::{MultipleDefinitionsError, UndefinedError},
    resolver::{Factor, Resolvable},
};

/// A type that represents the identifier of a dimension.
/// Can only be turned into an Ident by retrieving it from
/// the set of all defined dimensions.
#[derive(Clone)]
pub struct DimensionIdent(Ident);

/// A type that represents the identifier of a unit.
/// Can only be turned into an Ident by retrieving it from
/// the set of all defined units.
#[derive(Clone)]
pub struct UnitIdent(Ident);

/// A type that represents the identifier of a constant.
/// Can only be turned into an Ident by retrieving it from
/// the set of all defined constants.
#[derive(Clone)]
pub struct ConstantIdent(Ident);

/// The kind of an identifier
#[derive(Debug)]
pub enum Kind {
    Dimension,
    Unit,
    Constant,
}

pub trait IdentKind {
    fn ident(&self) -> &Ident;
    fn kind(&self) -> Kind;
}

#[derive(Default)]
pub struct IdentStorage {
    entries: HashMap<Ident, Kind>,
}

impl IdentStorage {
    pub fn remember_valid_and_filter_invalid<T: Resolvable>(&mut self, items: Vec<T>) -> Vec<T> {
        let items = emit_errors(self.filter_multiply_defined_identifiers(items));
        self.entries
            .extend(items.iter().map(|item| (item.ident().clone(), item.kind())));
        let (items, undefined) = emit_errors(self.filter_undefined_identifiers(items));
        // Make sure we remove all undefined identifiers at this point
        // so we can't ever mistake them for existing identifiers later.
        for undefined_item in undefined {
            self.entries.remove(undefined_item.ident());
        }
        items
    }

    fn filter_undefined_identifiers<R: Resolvable>(
        &self,
        items: Vec<R>,
    ) -> ((Vec<R>, Vec<R>), Result<(), UndefinedError>) {
        let (defined, undefined): (Vec<_>, Vec<_>) = items.into_iter().partition(|item| {
            get_rhs_idents(item)
                .iter()
                .all(|rhs_ident| self.entries.contains_key(rhs_ident))
        });

        let err = if undefined.is_empty() {
            Ok(())
        } else {
            let mut undefined_rhs = vec![];
            for item in undefined.iter() {
                let rhs_idents = get_rhs_idents(item);
                for rhs_ident in rhs_idents {
                    if !self.entries.contains_key(&rhs_ident) {
                        undefined_rhs.push(rhs_ident.clone());
                    }
                }
            }
            Err(UndefinedError(undefined_rhs))
        };
        ((defined, undefined), err)
    }

    fn filter_multiply_defined_identifiers<R: Resolvable>(
        &self,
        items: Vec<R>,
    ) -> (Vec<R>, Result<(), MultipleDefinitionsError>) {
        let mut counter: HashMap<_, usize> = items.iter().map(|item| (item.ident(), 0)).collect();
        for item in items.iter() {
            *counter.get_mut(item.ident()).unwrap() += 1;
        }
        let err = if items.iter().any(|item| counter[item.ident()] > 1) {
            Err(MultipleDefinitionsError(
                counter
                    .iter()
                    .filter(|(_, count)| **count > 1)
                    .map(|(multiply_defined_name, _)| {
                        items
                            .iter()
                            .map(|item| item.ident().clone())
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
}

fn get_rhs_idents(item: &impl Resolvable) -> Vec<Ident> {
    let mut rhs_idents = vec![];
    let expr = item.expr();
    for val in expr.iter_vals() {
        match val {
            Factor::Concrete(_) => {}
            Factor::Other(ident) => {
                rhs_idents.push(ident.clone());
            }
        }
    }
    rhs_idents
}
