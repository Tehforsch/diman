use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::{CallerType, Codegen};
use crate::types::Exponent;

// The following impls are all really ugly, and I'd prefer
// if they were just part of a trait. However they need to be
// const, so we'd need a const trait - this certainly isn't worth
// adding an unstable feature for.
// There is also an option of using a custom type instead of i64
// for the integer exponents, but this still doesn't solve all problems
// and also requires all the methods to "magically" exist.

#[cfg(feature = "rational-dimensions")]
impl Codegen {
    pub fn get_base_dimension_entry(&self, field: &Ident, value: &Exponent) -> TokenStream {
        let num = value.num();
        let denom = value.denom();
        quote! { #field: Exponent::new(#num, #denom), }
    }

    fn base_dimension_type(&self) -> TokenStream {
        quote! { Exponent }
    }

    fn add_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.add(other.#ident),
        }
    }

    fn sub_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.sub(other.#ident),
        }
    }

    fn neg_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.neg(),
        }
    }

    fn mul_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.mul(Exponent::int(other as i64)),
        }
    }

    fn sqrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.div(Exponent::int(2)),
        }
    }

    fn cbrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.div(Exponent::int(3)),
        }
    }

    fn sqrt_safety(&self, _ident: &Ident) -> TokenStream {
        quote! {}
    }

    fn cbrt_safety(&self, _ident: &Ident) -> TokenStream {
        quote! {}
    }
}

#[cfg(not(feature = "rational-dimensions"))]
impl Codegen {
    pub fn get_base_dimension_entry(&self, field: &Ident, value: &Exponent) -> TokenStream {
        quote! { #field: #value, }
    }

    fn base_dimension_type(&self) -> TokenStream {
        quote! { i64 }
    }

    fn add_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident + other.#ident,
        }
    }

    fn sub_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident - other.#ident,
        }
    }

    fn neg_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: -self.#ident,
        }
    }

    fn mul_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident * other as i64,
        }
    }

    fn sqrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident / 2,
        }
    }

    fn cbrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident / 3,
        }
    }

    fn sqrt_safety(&self, ident: &Ident) -> TokenStream {
        quote! {
            if self.#ident % 2 != 0 {
                panic!("Cannot take square root of quantity with a dimension that is not divisible by 2 in all components.");
            }
        }
    }

    fn cbrt_safety(&self, ident: &Ident) -> TokenStream {
        quote! {
            if self.#ident % 3 != 0 {
                panic!("Cannot take cubic root of quantity with a dimension that is not divisible by 3 in all components.");
            }
        }
    }
}
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
        let use_exponent = self.use_exponent_and_dimension_exponent_trait();
        quote! {
            #use_exponent

            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug, ::core::marker::ConstParamTy)]
            pub struct #name {
                #dimensions
            }

            #methods_impl
        }
    }

    fn use_exponent_and_dimension_exponent_trait(&self) -> TokenStream {
        let path_prefix = self.caller_type.path_prefix();
        let use_exponent = match self.caller_type {
            CallerType::External => {
                #[cfg(feature = "rational-dimensions")]
                quote! { use #path_prefix::ratio::Ratio as Exponent; }
                #[cfg(not(feature = "rational-dimensions"))]
                quote! { use i64 as Exponent; }
            }
            CallerType::Internal => {
                #[cfg(feature = "rational-dimensions")]
                quote! { use #path_prefix::::ratio::Ratio as Exponent; }
                #[cfg(not(feature = "rational-dimensions"))]
                quote! { use i64 as Exponent; }
            }
        };
        let use_dimension_exponent_trait = match self.caller_type {
            CallerType::External => {
                quote! { use #path_prefix::dimension_exponent::DimensionExponent; }
            }
            CallerType::Internal => {
                quote! { use #path_prefix::dimension_exponent::DimensionExponent; }
            }
        };
        quote! {
            #use_exponent
            #use_dimension_exponent_trait
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
