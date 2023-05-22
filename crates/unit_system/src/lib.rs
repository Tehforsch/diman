mod gen_traits;
mod parse;
mod types;
mod utils;
mod gen_float_methods;
mod gen_generic_methods;
mod storage_types;
mod gen_vector_methods;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use storage_types::{FloatType, VectorType};
use syn::*;
use types::{Defs, QuantityEntry, UnitEntry};
use utils::join;

#[proc_macro]
pub fn unit_system_2(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as Defs);
    join([
        defs.type_definition(),
        defs.type_functions(),
        defs.unit_array(),
        defs.float_quantity_definitions(),
        defs.vector_quantity_definitions(),
        defs.unit_constructors(),
        defs.qproduct_trait(),
        defs.numeric_traits(),
        defs.float_methods(),
        defs.vector_methods(),
        defs.generic_methods(),
    ])
    .into()
}

impl Defs {
    fn get_dimension_definition(&self, q: &QuantityEntry) -> TokenStream {
        let dimension_type = &self.dimension_type;
        let field_updates: TokenStream = q
            .dimensions_def
            .fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let value = &field.value.val;
                quote! { #ident: #value, }
            })
            .collect();
        quote! {
            #dimension_type {
                #field_updates
                ..#dimension_type::none()
            }
        }
    }

    pub(crate) fn type_definition(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
            #[repr(transparent)]
            pub struct #quantity_type<S: 'static, const D: #dimension_type>(pub(crate) S);
        }
    }

    pub(crate) fn type_functions(&self) -> TokenStream {
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
        }
    }

    pub(crate) fn vector_quantity_definitions(&self) -> TokenStream {
        self.vector_types()
            .iter()
            .map(|vector_type| {
                self.quantity_definitions_for_type(&vector_type.name, &vector_type.module_name)
            })
            .collect()
    }

    pub(crate) fn float_quantity_definitions(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| {
                self.quantity_definitions_for_type(&float_type.name, &float_type.module_name)
            })
            .collect()
    }

    pub(crate) fn quantity_definitions_for_type(
        &self,
        type_: &TokenStream,
        module_name: &TokenStream,
    ) -> TokenStream {
        let Self {
            dimension_type,
            quantity_type,
            ..
        } = &self;
        let quantities: TokenStream = self
            .quantities
            .iter()
            .map(|quantity| {
                let dimension = self.get_dimension_definition(&quantity);
                let quantity_type = &self.quantity_type;
                let quantity_name = &quantity.name;
                quote! {
                    pub type #quantity_name = #quantity_type::<#type_, { #dimension }>;
                }
            })
            .collect();
        // TODO: The use statements here are quite hacky and will probably
        // not work if dimension is declared in a different place from
        // the macro invocation.
        quote! {
            pub mod #module_name {
                use super::#dimension_type;
                use super::#quantity_type;
                #quantities
            }
        }
    }

    pub(crate) fn unit_constructors(&self) -> TokenStream {
        self
        .quantities
        .iter()
        .flat_map(|quantity| {
            let dimension = self.get_dimension_definition(&quantity);
            let quantity_type = &self.quantity_type;

            quantity
                .units_def
                .units
                .iter()
                .map(move |unit| {
                    let unit_name = &unit.name;
                    let factor = &unit.factor;
                    let conversion_method_name = format_ident!("in_{}", unit_name);
                    let vector_impls: TokenStream = self
                        .vector_types()
                        .iter()
                        .map(|vector_type| self.vector_unit_constructor(vector_type, &unit, &dimension))
                        .collect();
                    let float_impls: TokenStream = self
                        .float_types()
                        .iter()
                        .map(|float_type| self.float_unit_constructor(float_type, &unit, &dimension))
                        .collect();
                    quote! {
                        impl<S> #quantity_type<S, {#dimension}> where S: std::ops::Div<f64, Output = S> {
                            pub fn #conversion_method_name(self) -> S {
                                self.0 / #factor
                            }
                        }
                        #float_impls
                        #vector_impls
                    }
                })
        }).collect()
    }

    fn float_unit_constructor(
        &self,
        float_type: &FloatType,
        unit: &UnitEntry,
        quantity_dimension: &TokenStream,
    ) -> TokenStream {
        let Defs { quantity_type, .. } = &self;
        let UnitEntry {
            name: unit_name,
            factor,
            ..
        } = unit;
        let name = &float_type.name;
        quote! {
            impl #quantity_type<#name, {#quantity_dimension}> {
                pub fn #unit_name(val: #name) -> #quantity_type<#name, {#quantity_dimension}> {
                    #quantity_type::<#name, {#quantity_dimension}>(val * #factor)
                }
            }
        }
    }

    fn vector_unit_constructor(
        &self,
        vector_type: &VectorType,
        unit: &UnitEntry,
        quantity_dimension: &TokenStream,
    ) -> TokenStream {
        let Defs { quantity_type, .. } = &self;
        let UnitEntry {
            name: unit_name,
            factor,
            ..
        } = unit;
        let VectorType {
            name,
            float_type,
            num_dims,
            ..
        } = &vector_type;
        let fn_args = match num_dims {
            2 => quote! { x: #float_type, y: #float_type },
            3 => quote! { x: #float_type, y: #float_type, z: #float_type },
            _ => unreachable!(),
        };
        let call_args = match num_dims {
            2 => quote! { x, y },
            3 => quote! { x, y, z },
            _ => unreachable!(),
        };
        quote! {
            impl #quantity_type<#name, {#quantity_dimension}> {
                pub fn #unit_name(#fn_args) -> #quantity_type<#name, {#quantity_dimension}> {
                    #quantity_type::<#name, {#quantity_dimension}>(#name::new(#call_args) * (#factor as #float_type))
                }
            }
        }
    }

    // Only temporary to make the transition less menacing
    pub(crate) fn unit_array(&self) -> TokenStream {
        let dimension_type = &self.dimension_type;
        let unit_names_type = &self.unit_names_type;
        let unit_names_array_gen: TokenStream = self
            .quantities
            .iter()
            .flat_map(|quantity| {
                let dimension = self.get_dimension_definition(&quantity);
                quantity.units_def.units.iter().map(move |unit| {
                    let unit_symbol = unit.symbol.as_ref().unwrap();
                    let unit_factor = unit.factor;
                    quote! {
                        ({ #dimension }, #unit_symbol, #unit_factor),
                    }
                })
            })
            .collect();

        quote! {
            pub const #unit_names_type: &[(#dimension_type, &str, f64)] = &[
                #unit_names_array_gen
            ];
        }
    }
}
