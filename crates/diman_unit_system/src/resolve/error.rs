use std::collections::HashSet;

use proc_macro::{Diagnostic, Level};
use proc_macro2::Span;
use syn::Ident;

use crate::{dimension_math::BaseDimensions, types::BaseDimensionExponent};

use super::ident_storage::Kind;

pub struct UnresolvableError(pub Vec<Ident>);
pub struct UndefinedError(pub Vec<Ident>);
pub struct MultipleDefinitionsError(pub Vec<Vec<Ident>>);

pub enum TypeDefinitionsError {
    Multiple {
        type_name: &'static str,
        idents: Vec<Ident>,
    },
    None {
        type_name: &'static str,
        default_name: &'static str,
    },
}

pub struct ViolatedAnnotationError<'a> {
    pub annotation: &'a Ident,
    pub annotation_dims: &'a BaseDimensions,
    pub expr_dims: &'a BaseDimensions,
}

pub struct UndefinedAnnotationDimensionError<'a>(pub &'a Ident);

pub struct KindNotAllowedError<'a> {
    pub lhs_kind: Kind,
    pub rhs_kind: Kind,
    pub lhs_ident: &'a Ident,
    pub rhs_ident: &'a Ident,
}

pub struct WrongTypeInAnnotationError<'a> {
    pub annotation_ident: &'a Ident,
    pub annotation_kind: Kind,
}

pub struct MultipleBaseUnitsForDimensionError<'a> {
    pub dimension: &'a Ident,
    pub unit: &'a Ident,
}

pub struct BaseUnitForNonBaseDimensionError<'a> {
    pub dimension: &'a Ident,
    pub unit: &'a Ident,
}

pub struct SymbolDefinedMultipleTimes<'a> {
    pub symbol: &'a Ident,
    pub units: Vec<&'a Ident>,
}

pub trait Emit {
    fn emit(self);
}

pub fn emit_if_err<T, E: Emit>(result: Result<T, E>) {
    if let Err(err) = result {
        err.emit();
    }
}

impl Emit for TypeDefinitionsError {
    fn emit(self) {
        match self {
            Self::Multiple { type_name, idents } => {
                for ident in idents {
                    ident
                        .span()
                        .unwrap()
                        .error(format!(
                            "Multiple definitions for {} \"{}\".",
                            type_name, ident
                        ))
                        .emit();
                }
            }
            Self::None {
                type_name,
                default_name,
            } => {
                Span::call_site()
                    .unwrap()
                    .error(format!("No definition for {}", type_name))
                    .note("In order to provide readable error messages, the name must be specified in the macro call.")
                    .help(format!(
                        "Consider adding a definition inside the unit_system! macro:\n\t{} {};",
                        type_name, default_name
                    ))
                    .emit();
            }
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
                let value = *dims
                    .fields
                    .get(dim)
                    .unwrap_or(&BaseDimensionExponent::zero());
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
        let (lhs, rhs) = format_lhs_rhs_dimensions(self.annotation_dims, self.expr_dims);
        self.annotation
            .span()
            .unwrap()
            .error("Dimension mismatch in expression.")
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
            .note("Annotations using units and constants are not allowed.")
            .emit();
    }
}

impl Emit for UndefinedError {
    fn emit(self) {
        for ident in self.0 {
            ident
                .span()
                .unwrap()
                .error(format!("Undefined identifier \"{}\".", ident))
                .emit();
        }
    }
}

impl Emit for UnresolvableError {
    fn emit(self) {
        for ident in self.0 {
            ident
                .span()
                .unwrap()
                .error(format!("Unresolvable definition \"{}\".", ident))
                .help("Remove recursive definitions.")
                .emit();
        }
    }
}

impl Emit for MultipleDefinitionsError {
    fn emit(self) {
        for idents in self.0 {
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
}

impl<'a> Emit for KindNotAllowedError<'a> {
    fn emit(self) {
        let name = |kind| match kind {
            Kind::Dimension => "Dimension",
            Kind::Unit => "Unit",
            Kind::BaseUnit => "Unit",
            Kind::Constant => "Constant",
        };
        let plural = |kind| match kind {
            Kind::Dimension => "Dimensions",
            Kind::Unit => "Units",
            Kind::BaseUnit => "Units",
            Kind::Constant => "Constants",
        };
        let allowed_rhs_kinds = |kind| match kind {
            Kind::Dimension => "other dimensions",
            Kind::Unit => "other units and constants",
            Kind::BaseUnit => "other units and constants",
            Kind::Constant => "other constants and units",
        };
        Diagnostic::spanned(
            vec![
                self.lhs_ident.span().unwrap(),
                self.rhs_ident.span().unwrap(),
            ],
            Level::Error,
            format!(
                "{} {} is defined in terms of the {} {}.",
                name(self.lhs_kind),
                self.lhs_ident,
                name(self.rhs_kind).to_lowercase(),
                self.rhs_ident
            ),
        )
        .note(format!(
            "{} can only be defined in terms of {}.",
            plural(self.lhs_kind),
            allowed_rhs_kinds(self.lhs_kind)
        ))
        .emit();
    }
}

impl<'a> Emit for WrongTypeInAnnotationError<'a> {
    fn emit(self) {
        let name = match self.annotation_kind {
            Kind::Dimension => unreachable!(),
            Kind::BaseUnit => "unit",
            Kind::Unit => "unit",
            Kind::Constant => "constant",
        };
        self.annotation_ident
            .span()
            .unwrap()
            .error(format!(
                "Type error in annotation: Expected dimension, found {} '{}'.",
                name, self.annotation_ident
            ))
            .note("Annotations can only be done using dimensions.")
            .emit();
    }
}

impl<'a> Emit for MultipleBaseUnitsForDimensionError<'a> {
    fn emit(self) {
        self.unit
            .span()
            .unwrap()
            .error(format!(
                "'{}' is defined to be a base unit for dimension '{}', but there already is a base unit for this dimension.",
                self.unit,
                self.dimension,
            ))
            .note("There can only be one base unit per base dimension.")
            .emit();
    }
}

impl<'a> Emit for BaseUnitForNonBaseDimensionError<'a> {
    fn emit(self) {
        self.unit
            .span()
            .unwrap()
            .error(format!(
                "'{}' is defined to be a base unit for dimension '{}', but '{}' is not a base dimension.",
                self.unit,
                self.dimension,
                self.dimension,
            ))
            .note("Base units can only be defined for base dimensions.")
            .emit();
    }
}

impl<'a> Emit for SymbolDefinedMultipleTimes<'a> {
    fn emit(self) {
        Diagnostic::spanned(
            self.units
                .into_iter()
                .map(|ident| ident.span().unwrap())
                .collect::<Vec<_>>(),
            Level::Error,
            format!("Symbol '{}' is used for multiple units.", self.symbol),
        )
        .emit()
    }
}
