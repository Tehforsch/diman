use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::types::Defs;

impl Defs {
    pub(crate) fn gen_dimension(&self) -> TokenStream {
        let name = &self.dimension_type;

        let dim_type = self.base_dimension_type();
        let dimensions: proc_macro2::TokenStream = self
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
        quote! {
            use ::diman::Ratio;

            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug, ::core::marker::ConstParamTy)]
            pub struct #name {
                #dimensions
            }

            #methods_impl
        }
    }

    fn dimension_methods_impl(&self) -> TokenStream {
        let type_name = &self.dimension_type;
        let gen = |f: &dyn Fn(&Defs, &Ident) -> TokenStream| {
            self.base_dimensions()
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
