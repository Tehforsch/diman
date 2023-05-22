use proc_macro2::TokenStream;
use quote::quote;

use crate::{storage_types::FloatType, types::Defs, utils::join};

impl Defs {
    fn dimensionless_float_method(
        &self,
        float_type: &FloatType,
        method_name: &TokenStream,
    ) -> TokenStream {
        let float_type_name = &float_type.name;
        let Self {
            dimension_type,
            quantity_type,
            ..
        } = &self;
        quote! {
            impl #quantity_type<#float_type_name, {#dimension_type::none()} > {
                pub fn #method_name(&self) -> #quantity_type<#float_type_name, {#dimension_type::none()}> {
                    Self(self.0.#method_name())
                }
            }
        }
    }

    fn dimensionless_float_method_for_all_float_types(
        &self,
        method_name: &TokenStream,
    ) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| self.dimensionless_float_method(float_type, method_name))
            .collect()
    }

    pub fn float_methods(&self) -> TokenStream {
        join([
            self.dimensionless_float_method_for_all_float_types(&quote! { log2 }),
            self.dimensionless_float_method_for_all_float_types(&quote! { ln }),
            self.dimensionless_float_method_for_all_float_types(&quote! { log10 }),
            self.dimensionless_float_method_for_all_float_types(&quote! { exp }),
            self.dimensionless_float_method_for_all_float_types(&quote! { exp2 }),
            self.dimensionless_float_method_for_all_float_types(&quote! { ceil }),
            self.dimensionless_float_method_for_all_float_types(&quote! { floor }),
            self.dimensionless_float_method_for_all_float_types(&quote! { sin }),
            self.dimensionless_float_method_for_all_float_types(&quote! { cos }),
            self.dimensionless_float_method_for_all_float_types(&quote! { tan }),
            self.dimensionless_float_method_for_all_float_types(&quote! { asin }),
            self.dimensionless_float_method_for_all_float_types(&quote! { acos }),
            self.dimensionless_float_method_for_all_float_types(&quote! { atan }),
            self.dimensionless_float_method_for_all_float_types(&quote! { sinh }),
            self.dimensionless_float_method_for_all_float_types(&quote! { cosh }),
            self.dimensionless_float_method_for_all_float_types(&quote! { tanh }),
            self.dimensionless_float_method_for_all_float_types(&quote! { asinh }),
            self.dimensionless_float_method_for_all_float_types(&quote! { acosh }),
            self.dimensionless_float_method_for_all_float_types(&quote! { atanh }),
            self.dimensionless_float_method_for_all_float_types(&quote! { exp_m1 }),
            self.dimensionless_float_method_for_all_float_types(&quote! { ln_1p }),
        ])
    }
}
