use crate::dimension_math::BaseDimensions;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::{storage_types::StorageType, Codegen};

impl Codegen {
    pub fn get_dimension_expr(&self, dim: &BaseDimensions) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let field_updates: TokenStream = dim
            .fields()
            .map(|(field, value)| self.get_base_dimension_entry(field, value))
            .collect();
        let span = self.defs.quantity_type.span();
        let none_update = if dim.num_fields() < self.defs.base_dimensions.len() {
            quote! { ..#dimension_type::none() }
        } else {
            quote! {}
        };
        quote_spanned! {span =>
            #dimension_type {
                #field_updates
                #none_update
            }
        }
    }

    pub(crate) fn gen_dimensions(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let defs = self.gen_dimension_definitions();
        quote! {
            pub mod dimensions {
                use super::#dimension_type;
                use super::#quantity_type;
                use super::Exponent;
                #defs
            }
        }
    }

    fn gen_dimension_definitions(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let dimensions: TokenStream = self
            .defs
            .dimensions
            .iter()
            .map(|quantity| {
                let dimension = self.get_dimension_expr(&quantity.dimensions);
                let quantity_type = &self.defs.quantity_type;
                let quantity_name = &quantity.name;
                let span = self.defs.dimension_type.span();
                quote_spanned! {span =>
                    pub type #quantity_name<S> = #quantity_type::<S, { #dimension }>;
                }
            })
            .collect();
        quote! {
            #dimensions
        }
    }

    fn constant_definitions_for_storage_type(&self, type_: &dyn StorageType) -> TokenStream {
        self
            .defs.constants
            .iter()
            .map(|constant| {
                let dimension = self.get_dimension_expr(&constant.dimensions);
                let quantity_type = &self.defs.quantity_type;
                let constant_name = &constant.name;
                let magnitude = constant.magnitude.into_f64();
                let float_type = &type_.base_storage().name;
                let type_ = type_.name();
                // TODO(minor): The allow(clippy::approx_constant)
                // exists to allow definitions of, for example, PI in
                // unit_system calls.  A better solution would
                // probably be to define PI (and possibly some other
                // mathematical constants) for use in the unit_system
                // macro, but this is an easy fix for now.
                quote! {
                    #[allow(clippy::approx_constant)]
                    pub const #constant_name: #quantity_type::<#type_, { #dimension }> = #quantity_type::<#type_, { #dimension }>(#magnitude as #float_type);
                }
            })
            .collect()
    }
}
