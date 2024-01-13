use proc_macro2::TokenStream;
use quote::quote;

use crate::types::Defs;

use super::{join, storage_types::FloatType};

impl Defs {
    fn ensure_float_traits(&self) -> TokenStream {
        if cfg!(feature = "num-traits-libm") {
            quote! {
                use num_traits::float::Float;
            }
        } else {
            quote! {
                use num_traits::float::FloatCore;
            }
        }
    }

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

    #[cfg_attr(
        not(any(feature = "std", feature = "num-traits-libm")),
        allow(dead_code)
    )]
    fn all_dimensionless_float_methods(&self) -> TokenStream {
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

    pub fn gen_float_methods(&self) -> TokenStream {
        join([
            self.ensure_float_traits(),
            #[cfg(any(feature = "std", feature = "num-traits-libm"))]
            self.all_dimensionless_float_methods(),
            self.specific_float_methods_for_all_float_types(),
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
                pub fn squared(&self) -> #quantity_type<#float_type, { D.mul(2) }>
                where
                    #quantity_type::<#float_type, { D.mul(2) }>:
                {
                    #quantity_type::<#float_type, { D.mul(2) }>(self.0.powi(2))
                }

                pub fn cubed(&self) -> #quantity_type<#float_type, { D.mul(3) }>
                where
                    #quantity_type::<#float_type, { D.mul(3) }>:
                {
                    #quantity_type::<#float_type, { D.mul(3) }>(self.0.powi(3))
                }

                pub fn powi<const I: i32>(&self) -> #quantity_type<#float_type, { D.mul(I) }>
                where
                    #quantity_type::<#float_type, { D.mul(I) }>:
                {
                    #quantity_type::<#float_type, { D.mul(I) }>(self.0.powi(I))
                }

                #[cfg(any(feature = "std", feature = "num-traits-libm"))]
                pub fn sqrt(&self) -> #quantity_type<#float_type, { D.div_2() }>
                {
                    #quantity_type::<#float_type, { D.div_2() }>(self.0.sqrt())
                }

                #[cfg(any(feature = "std", feature = "num-traits-libm"))]
                pub fn cbrt(&self) -> #quantity_type<#float_type, { D.div_3() }>
                {
                    #quantity_type::<#float_type, { D.div_3() }>(self.0.cbrt())
                }

                pub fn min<Q: Into<Self>>(self, other: Q) -> Self {
                    Self(self.0.min(other.into().0))
                }

                pub fn max<Q: Into<Self>>(self, other: Q) -> Self {
                    Self(self.0.max(other.into().0))
                }

                pub fn clamp<Q: Into<Self>>(self, min: Q, max: Q) -> Self {
                    Self(self.0.clamp(min.into().0, max.into().0))
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
