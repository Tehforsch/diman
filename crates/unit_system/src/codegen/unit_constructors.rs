use quote::{quote, format_ident, quote_spanned};
use proc_macro2::TokenStream;
use syn::spanned::Spanned;

use crate::{types::{Defs, Unit}, storage_types::{VectorType, FloatType}};

impl Defs {
    pub fn unit_constructors(&self) -> TokenStream {
        self.units.iter().map(|unit| {
            let dimension = self.get_dimension_expr(&unit.dimension);
            let quantity_type = &self.quantity_type;
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
        }).collect()
    }

    fn float_unit_constructor(
        &self,
        float_type: &FloatType,
        unit: &Unit,
        quantity_dimension: &TokenStream,
    ) -> TokenStream {
        let Defs { quantity_type, .. } = &self;
        let Unit {
            name: unit_name,
            factor,
            ..
        } = unit;
        let name = &float_type.name;
        let span = self.dimension_type.span();
        quote_spanned! {
            span => 
            impl #quantity_type<#name, {#quantity_dimension}> {
                pub fn #unit_name(val: #name) -> #quantity_type<#name, {#quantity_dimension}> {
                    #quantity_type::<#name, {#quantity_dimension}>(val * (#factor as #name))
                }
            }
        }
    }

    fn vector_unit_constructor(
        &self,
        vector_type: &VectorType,
        unit: &Unit,
        quantity_dimension: &TokenStream,
    ) -> TokenStream {
        let Defs { quantity_type, .. } = &self;
        let Unit {
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
        let float_type = &float_type.name;
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
}
