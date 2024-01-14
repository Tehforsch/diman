use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::{CallerType, Codegen};

impl Codegen {
    pub(crate) fn gen_dimension(&self) -> TokenStream {
        let name = &self.defs.dimension_type;

        let dim_type = self.base_dimension_type();
        let dimensions: proc_macro2::TokenStream = self
            .defs
            .base_dimensions
            .iter()
            .map(|dim| {
                let dim = &dim.0;
                quote! {
                    #dim: #dim_type,
                }
            })
            .collect();
        let methods_impl: proc_macro2::TokenStream = self.dimension_methods_impl();
        let use_exponent = self.use_exponent_and_base_dimension_exponent_trait();
        quote! {
            #use_exponent

            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug, ::core::marker::ConstParamTy)]
            pub struct #name {
                #dimensions
            }

            #methods_impl
        }
    }

    fn use_exponent_and_base_dimension_exponent_trait(&self) -> TokenStream {
        let use_exponent = match self.caller_type {
            CallerType::External => {
                #[cfg(feature = "rational-dimensions")]
                quote! { use ::diman::internal::Ratio as Exponent; }
                #[cfg(not(feature = "rational-dimensions"))]
                quote! { use i64 as Exponent; }
            }
            CallerType::Internal => {
                #[cfg(feature = "rational-dimensions")]
                quote! { use ::diman_lib::ratio::Ratio as Exponent; }
                #[cfg(not(feature = "rational-dimensions"))]
                quote! { use i64 as Exponent; }
            }
        };
        let use_base_dimension_exponent_trait = match self.caller_type {
            CallerType::External => {
                quote! { use ::diman::internal::BaseDimensionExponent; }
            }
            CallerType::Internal => {
                quote! { use ::diman_lib::base_dimension_exponent::BaseDimensionExponent; }
            }
        };
        quote! {
            #use_exponent
            #use_base_dimension_exponent_trait
        }
    }

    fn zero_entry(&self, ident: &Ident) -> TokenStream {
        #[cfg(feature = "rational-dimensions")]
        quote! { #ident: Exponent::int(0), }
        #[cfg(not(feature = "rational-dimensions"))]
        quote! { #ident: 0, }
    }

    fn dimension_methods_impl(&self) -> TokenStream {
        let type_name = &self.defs.dimension_type;
        let gen = |f: &dyn Fn(&Codegen, &Ident) -> TokenStream| {
            self.defs
                .base_dimensions()
                .map(|ident| f(self, ident))
                .collect::<TokenStream>()
        };
        let none_gen = gen(&Self::zero_entry);
        let mul_gen = gen(&Self::add_entry);
        let div_gen = gen(&Self::sub_entry);
        let inv_gen = gen(&Self::neg_entry);
        let powi_gen = gen(&Self::mul_entry);
        let sqrt_gen = gen(&Self::sqrt_entry);
        let cbrt_gen = gen(&Self::cbrt_entry);
        let sqrt_safety_gen = gen(&Self::sqrt_safety);
        let cbrt_safety_gen = gen(&Self::cbrt_safety);

        quote! {
            impl #type_name {
                pub const fn none() -> Self {
                    Self {
                        #none_gen
                    }
                }

                pub const fn add(self, other: Self) -> Self {
                    Self {
                        #mul_gen
                    }
                }

                pub const fn sub(self, other: Self) -> Self {
                    Self {
                        #div_gen
                    }
                }

                pub const fn neg(self) -> Self {
                    Self {
                        #inv_gen
                    }
                }

                pub const fn mul(self, other: i32) -> Self {
                    Self {
                        #powi_gen
                    }
                }

                pub const fn div_2(self) -> Self {
                    #sqrt_safety_gen
                    Self {
                        #sqrt_gen
                    }
                }

                pub const fn div_3(self) -> Self {
                    #cbrt_safety_gen
                    Self {
                        #cbrt_gen
                    }
                }
            }
        }
    }
}
