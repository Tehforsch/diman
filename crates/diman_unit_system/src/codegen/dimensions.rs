use crate::{dimension_math::BaseDimensions, storage_types::StorageType, types::Defs};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

impl Defs {
    pub(crate) fn gen_quantity(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        let span = quantity_type.span();
        let functions = self.quantity_functions();
        quote_spanned! {span =>
            #[derive(Clone, Copy, Eq, Default)]
            #[repr(transparent)]
            pub struct #quantity_type<S, const D: #dimension_type>(pub(crate) S);
            #functions
        }
    }

    fn quantity_functions(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            impl<S> #quantity_type<S, { #dimension_type::none() }> {
                /// Get the value of a dimensionless quantity
                pub fn value(self) -> S {
                    self.0
                }

                /// Get a reference to the value of a dimensionless quantity
                pub fn value_ref(&self) -> &S {
                    &self.0
                }
            }

            impl<S, const D: #dimension_type> #quantity_type<S, D> {
                /// Return the value of a quantity, regardless of whether
                /// it is dimensionless or not. Use this carefully, since the
                /// result depends on the underlying base units
                pub fn value_unchecked(self) -> S {
                    self.0
                }

                /// Create a new quantity for the dimension with a given value.
                /// Use carefully, since the constructed quantity depends on the
                /// used base units.
                pub const fn new_unchecked(s: S) -> Self {
                    Self(s)
                }
            }

            impl<S> core::ops::Deref for #quantity_type<S, { #dimension_type::none() }> {
                type Target = S;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }

    pub fn get_dimension_expr(&self, dim: &BaseDimensions) -> TokenStream {
        let dimension_type = &self.dimension_type;
        let field_updates: TokenStream = dim
            .fields()
            .map(|(field, value)| self.get_base_dimension_entry(field, value))
            .collect();
        let span = self.quantity_type.span();
        let none_update = if dim.num_fields() < self.base_dimensions.len() {
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

    pub(crate) fn gen_definitions_for_storage_types(&self) -> TokenStream {
        self.storage_types()
            .map(|type_| {
                self.definitions_for_storage_type(
                    &*type_,
                    type_.module_name(),
                    type_.generate_constants(),
                )
            })
            .collect()
    }

    #[cfg(feature = "rational-dimensions")]
    fn use_ratio(&self) -> TokenStream {
        quote! { use super::Ratio; }
    }

    #[cfg(not(feature = "rational-dimensions"))]
    fn use_ratio(&self) -> TokenStream {
        quote! {}
    }

    fn definitions_for_storage_type(
        &self,
        type_: &dyn StorageType,
        module_name: &TokenStream,
        gen_constants: bool,
    ) -> TokenStream {
        let Self {
            dimension_type,
            quantity_type,
            ..
        } = &self;
        let quantities = self.quantity_definitions_for_storage_type(type_);
        let constants = if gen_constants {
            self.constant_definitions_for_storage_type(type_)
        } else {
            quote! {}
        };
        let use_ratio = self.use_ratio();
        quote! {
            pub mod #module_name {
                use super::#dimension_type;
                use super::#quantity_type;
                #use_ratio
                #quantities
                #constants
            }
        }
    }

    fn quantity_definitions_for_storage_type(&self, type_: &dyn StorageType) -> TokenStream {
        self.dimensions
            .iter()
            .map(|quantity| {
                let dimension = self.get_dimension_expr(&quantity.dimensions);
                let quantity_type = &self.quantity_type;
                let quantity_name = &quantity.name;
                let type_ = type_.name();
                let span = self.dimension_type.span();
                quote_spanned! {span =>
                    pub type #quantity_name = #quantity_type::<#type_, { #dimension }>;
                }
            })
            .collect()
    }

    fn constant_definitions_for_storage_type(&self, type_: &dyn StorageType) -> TokenStream {
        self
            .constants
            .iter()
            .map(|constant| {
                let dimension = self.get_dimension_expr(&constant.dimensions);
                let quantity_type = &self.quantity_type;
                let constant_name = &constant.name;
                let value = constant.magnitude;
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
                    pub const #constant_name: #quantity_type::<#type_, { #dimension }> = #quantity_type::<#type_, { #dimension }>(#value as #float_type);
                }
            })
            .collect()
    }
}
