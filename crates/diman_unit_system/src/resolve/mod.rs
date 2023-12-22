mod error;
mod ident_storage;

use std::result::Result;

use proc_macro2::Span;
use syn::Ident;

use crate::{
    derive_dimension::to_snakecase,
    types::{Defs, UnresolvedDefs},
};

use self::{
    error::{emit_if_err, Emit, MultipleTypeDefinitionsError},
    ident_storage::IdentStorage,
};

fn default_dimension_type() -> Ident {
    Ident::new("Dimension", Span::call_site())
}

fn default_quantity_type() -> Ident {
    Ident::new("quantity", Span::call_site())
}

fn get_single_ident(
    mut dimension_types: Vec<Ident>,
    type_name: &'static str,
    default: impl Fn() -> Ident,
) -> (Ident, Result<(), MultipleTypeDefinitionsError>) {
    if dimension_types.len() == 1 {
        (dimension_types.remove(0), Ok(()))
    } else if dimension_types.is_empty() {
        (default(), Ok(()))
    } else {
        let dimension_type = dimension_types[0].clone();
        (
            dimension_type,
            Err(MultipleTypeDefinitionsError {
                idents: dimension_types,
                type_name,
            }),
        )
    }
}

/// A helper function for emitting all the errors contained in the
/// result types but continuing with a partial result anyways. This is
/// done so that we at least define all the quantities that can be
/// partially resolved in order to keep the amount of error messages
/// manageable.
fn emit_errors<T, E: Emit>((input, result): (T, Result<(), E>)) -> T {
    if let Err(err) = result {
        err.emit();
    }
    input
}

impl UnresolvedDefs {
    pub fn resolve(self) -> Defs {
        let quantity_type = emit_errors(get_single_ident(
            self.quantity_types,
            "quantity type",
            default_quantity_type,
        ));
        let dimension_type = emit_errors(get_single_ident(
            self.dimension_types,
            "dimension type",
            default_dimension_type,
        ));
        let mut idents = IdentStorage::default();
        let base_dimensions = self
            .dimensions
            .iter()
            .filter(|d| d.is_base_dimension())
            .map(|x| to_snakecase(&x.name))
            .collect();
        idents.add(self.dimensions);
        idents.add(self.units);
        idents.add(self.constants);
        emit_if_err(idents.filter_undefined());
        emit_if_err(idents.filter_multiply_defined());
        emit_if_err(idents.resolve());
        emit_if_err(idents.check_type_annotations());
        let dimensions = idents.get_items();
        let units = idents.get_items();
        let constants = idents.get_items();
        Defs {
            dimension_type,
            quantity_type,
            dimensions,
            units,
            constants,
            base_dimensions,
        }
    }
}
