mod operator_trait;

use proc_macro2::TokenStream;
use quote::quote;

use crate::types::Defs;

impl Defs {
    pub fn gen_numeric_trait_impls(&self) -> TokenStream {
        let operators = self.gen_operator_trait_impls();
        let sum = self.gen_sum_impl();
        let neg = self.gen_neg_impl();
        let from = self.gen_from_impl();
        quote! {
            #operators
            #sum
            #neg
            #from
        }
    }

    fn gen_sum_impl(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<const D: #dimension_type, S: Default + core::ops::AddAssign<S>> core::iter::Sum
                for #quantity_type<S, D>
            {
                fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                    let mut total = Self::default();
                    for item in iter {
                        total += item;
                    }
                    total
                }
            }

        }
    }

    fn gen_neg_impl(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<const D: #dimension_type, S: core::ops::Neg<Output=S>> core::ops::Neg for #quantity_type<S, D> {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self(-self.0)
                }
            }
        }
    }

    fn gen_from_impl(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<S> From<S>
                for #quantity_type<S, { #dimension_type::none() }>
            {
                fn from(rhs: S) -> Self {
                    Self(rhs)
                }
            }

        }
    }
}
