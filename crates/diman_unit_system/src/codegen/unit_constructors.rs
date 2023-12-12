use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use crate::{
    storage_types::{FloatType, VectorType},
    types::{Defs, Unit},
};

impl Defs {
    pub fn unit_constructors(&self) -> TokenStream {
        self.units.iter().map(|unit| {
            let dimension = self.get_dimension_expr(&unit.dimensions);
            let quantity_type = &self.quantity_type;
            let unit_name = &unit.name;
            let factor = &unit.factor;
            let conversion_method_name = format_ident!("in_{}", unit_name);
            let vector_impls: TokenStream = self
                .vector_types()
                .iter()
                .map(|vector_type| self.vector_unit_constructor(vector_type, unit, &dimension))
                .collect();
            let float_impls: TokenStream = self
                .float_types()
                .iter()
                .map(|float_type| self.float_unit_constructor(float_type, unit, &dimension))
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
        // Without const_fn_floating_point_arithmetic (https://github.com/rust-lang/rust/issues/57241)
        // we cannot make unit constructors a const fn in general (since it requires the unstable
        // const_fn_floating_point_arithmetic feature). The following allows the constructor with 1.0
        // conversion factor to be const.
        let const_fn = *factor == 1.0;
        let fn_def = if const_fn {
            quote! { const fn }
        } else {
            quote! { fn }
        };
        let value = if const_fn {
            quote! { val }
        } else {
            quote! { val * #factor as #name }
        };
        quote_spanned! {span =>
            impl #quantity_type<#name, {#quantity_dimension}> {
                pub #fn_def #unit_name(val: #name) -> #quantity_type<#name, {#quantity_dimension}> {
                    #quantity_type::<#name, {#quantity_dimension}>(#value)
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
        let span = self.dimension_type.span();
        quote_spanned! {span =>
            impl #quantity_type<#name, {#quantity_dimension}> {
                pub fn #unit_name(#fn_args) -> #quantity_type<#name, {#quantity_dimension}> {
                    #quantity_type::<#name, {#quantity_dimension}>(#name::new(#call_args) * (#factor as #float_type))
                }
            }
        }
    }
}
