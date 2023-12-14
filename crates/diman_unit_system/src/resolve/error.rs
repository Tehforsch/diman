use std::collections::HashSet;

use proc_macro::{Diagnostic, Level};
use syn::Ident;

use crate::types::BaseDimensions;

pub enum Error {
    Unresolvable(Vec<Ident>),
    Undefined(Vec<Ident>),
    Multiple(Vec<Vec<Ident>>),
}

pub struct MultipleTypeDefinitionsError {
    pub type_name: &'static str,
    pub idents: Vec<Ident>,
}

pub struct ViolatedAnnotationError<'a> {
    pub annotation: &'a Ident,
    pub lhs_dims: &'a BaseDimensions,
    pub rhs_dims: &'a BaseDimensions,
}

pub struct UndefinedAnnotationDimensionError<'a>(pub &'a Ident);

pub type Result<T> = std::result::Result<T, Error>;

pub trait Emit {
    fn emit(self);
}

impl Emit for MultipleTypeDefinitionsError {
    fn emit(self) {
        for ident in self.idents {
            ident
                .span()
                .unwrap()
                .error(format!(
                    "Multiple definitions for {} \"{}\".",
                    self.type_name, ident
                ))
                .emit();
        }
    }
}

/// This formats the two base dimensions such that all
/// entries appearing in one will appear in the formatted
/// string of the other.
fn format_lhs_rhs_dimensions(lhs: &BaseDimensions, rhs: &BaseDimensions) -> (String, String) {
    let available_dims: HashSet<_> = lhs.fields.keys().chain(rhs.fields.keys()).collect();
    // Make sure to sort identifiers alphabetically, to make sure
    // we get deterministic error messages.
    let mut available_dims: Vec<_> = available_dims.into_iter().collect();
    available_dims.sort();
    let format = |dims: &BaseDimensions| {
        available_dims
            .iter()
            .map(|dim| {
                let value = *dims.fields.get(dim).unwrap_or(&0);
                format!("{}^{}", dim, value)
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    (format(lhs), format(rhs))
}

impl<'a> Emit for ViolatedAnnotationError<'a> {
    fn emit(self) {
        // In the future, it would be nice to have a proper span for the
        // second help text that points to the rhs. Unfortunately, joining
        // spans of the expressions on the rhs is a little more difficult
        // than it initially seems, so I'll postpone this for now.
        let (lhs, rhs) = format_lhs_rhs_dimensions(self.lhs_dims, self.rhs_dims);
        self.annotation
            .span()
            .unwrap()
            .error(format!("Dimension mismatch in expression."))
            .help(format!(
                "The annotation on the left-hand side has dimensions {}",
                lhs
            ))
            .help(format!(
                "but the expression on the right-hand side has dimensions {}",
                rhs
            ))
            .emit();
    }
}

impl<'a> Emit for UndefinedAnnotationDimensionError<'a> {
    fn emit(self) {
        self.0
            .span()
            .unwrap()
            .error(format!("Undefined dimension {} in annotation.", self.0))
            .note(format!(
                "Annotations using units and constants are not allowed."
            ))
            .emit();
    }
}

impl Emit for Error {
    fn emit(self) {
        match self {
            Error::Unresolvable(idents) => emit_unresolvable(idents),
            Error::Undefined(idents) => emit_undefined(idents),
            Error::Multiple(idents) => emit_multiple(idents),
        }
    }
}

fn emit_undefined(idents: Vec<Ident>) {
    for ident in idents {
        ident
            .span()
            .unwrap()
            .error(format!("Undefined identifier \"{}\".", ident))
            .emit();
    }
}

fn emit_unresolvable(idents: Vec<Ident>) {
    for ident in idents {
        ident
            .span()
            .unwrap()
            .error(format!("Unresolvable definition \"{}\".", ident))
            .help("Remove recursive definitions.")
            .emit();
    }
}

fn emit_multiple(idents: Vec<Vec<Ident>>) {
    for idents in idents {
        let name = &idents[0];
        Diagnostic::spanned(
            idents
                .iter()
                .map(|ident| ident.span().unwrap())
                .collect::<Vec<_>>(),
            Level::Error,
            format!("Identifier \"{}\" defined multiple times.", name),
        )
        .emit();
    }
}
