mod operator_trait;

use proc_macro2::TokenStream;
use quote::quote;

use super::Codegen;

impl Codegen {
    pub fn gen_numeric_trait_impls(&self) -> TokenStream {
        let storage_trait = self.gen_storage_trait();
        let operators = self.gen_operator_trait_impls();
        let sum = self.gen_sum_impl();
        let neg = self.gen_neg_impl();
        let from = self.gen_from_impl();
        quote! {
            #storage_trait
            #operators
            #sum
            #neg
            #from
        }
    }

    /// Marks the concrete types which may be used as storage types.
    ///
    /// Without this bound, operator trait impls for generic storage types are
    /// incoherent, since we could also choose `Quantity<_, Dimension::none()>`
    /// as storage type which would overlap with the quantity-to-quantity
    /// operator implementations.
    ///
    /// This was the case in previous implementations but the compiler did not
    /// catch it (due to a compiler bug, allowing incoherent impls).
    fn gen_storage_trait(&self) -> TokenStream {
        let storage_types = self.storage_type_names();
        quote! {
            #[doc(hidden)]
            trait __DimanStorage {}

            #(impl __DimanStorage for #storage_types {})*
        }
    }

    fn gen_sum_impl(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
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
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
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
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
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
