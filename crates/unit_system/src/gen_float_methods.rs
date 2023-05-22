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

            self.specific_float_methods_for_all_float_types()
        ])
    }

    fn specific_float_methods_for_all_float_types(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| self.specific_float_methods(float_type))
            .collect()
    }

    fn specific_float_methods(&self, float_type: &FloatType) -> TokenStream {
        let float_type = &float_type.name;
        let Self {
            dimension_type,
            quantity_type,
            ..
        } = &self;
        quote! {
            impl<const D: #dimension_type> #quantity_type<#float_type, D> {
                pub fn squared(&self) -> #quantity_type<#float_type, { D.dimension_powi(2) }>
                where
                    #quantity_type::<#float_type, { D.dimension_powi(2) }>:
                {
                    #quantity_type::<#float_type, { D.dimension_powi(2) }>(self.0.powi(2))
                }

                pub fn cubed(&self) -> #quantity_type<#float_type, { D.dimension_powi(3) }>
                where
                    #quantity_type::<#float_type, { D.dimension_powi(3) }>:
                {
                    #quantity_type::<#float_type, { D.dimension_powi(3) }>(self.0.powi(3))
                }

                pub fn powi<const I: i32>(&self) -> #quantity_type<#float_type, { D.dimension_powi(I) }>
                where
                    #quantity_type::<#float_type, { D.dimension_powi(I) }>:
                {
                    #quantity_type::<#float_type, { D.dimension_powi(I) }>(self.0.powi(I))
                }

                pub fn sqrt(&self) -> #quantity_type<#float_type, { D.dimension_sqrt() }>
                {
                    #quantity_type::<#float_type, { D.dimension_sqrt() }>(self.0.sqrt())
                }

                pub fn cbrt(&self) -> #quantity_type<#float_type, { D.dimension_cbrt() }>
                {
                    #quantity_type::<#float_type, { D.dimension_cbrt() }>(self.0.cbrt())
                }

                pub fn min(self, other: Self) -> Self {
                    Self(self.0.min(other.0))
                }

                pub fn max(self, other: Self) -> Self {
                    Self(self.0.max(other.0))
                }

                pub fn clamp(self, min: Self, max: Self) -> Self {
                    Self(self.0.clamp(min.0, max.0))
                }

                pub fn zero() -> Self {
                    Self(0.0)
                }

                pub fn is_positive(&self) -> bool {
                    self.0 > 0.0
                }

                pub fn is_positive_or_zero(&self) -> bool {
                    self.0 >= 0.0
                }

                pub fn is_negative(&self) -> bool {
                    self.0 < 0.0
                }

                pub fn is_negative_or_zero(&self) -> bool {
                    self.0 <= 0.0
                }

                pub fn is_nan(&self) -> bool {
                    self.0.is_nan()
                }
            }
        }
    }
}
