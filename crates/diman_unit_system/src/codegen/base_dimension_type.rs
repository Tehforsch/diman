use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::Codegen;
use crate::types::Exponent;

#[cfg(feature = "rational-dimensions")]
impl Codegen {
    pub fn get_base_dimension_entry(&self, field: &Ident, value: &Exponent) -> TokenStream {
        let num = value.num();
        let denom = value.denom();
        quote! { #field: Exponent::new(#num, #denom), }
    }

    pub fn base_dimension_type(&self) -> TokenStream {
        quote! { Exponent }
    }

    pub fn add_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.add(other.#ident),
        }
    }

    pub fn sub_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.sub(other.#ident),
        }
    }

    pub fn neg_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.neg(),
        }
    }

    pub fn mul_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.mul(Exponent::int(other as i64)),
        }
    }

    pub fn sqrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.div(Exponent::int(2)),
        }
    }

    pub fn cbrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident.div(Exponent::int(3)),
        }
    }

    pub fn sqrt_safety(&self, _ident: &Ident) -> TokenStream {
        quote! {}
    }

    pub fn cbrt_safety(&self, _ident: &Ident) -> TokenStream {
        quote! {}
    }
}

#[cfg(not(feature = "rational-dimensions"))]
impl Codegen {
    pub fn get_base_dimension_entry(&self, field: &Ident, value: &Exponent) -> TokenStream {
        quote! { #field: #value, }
    }

    pub fn base_dimension_type(&self) -> TokenStream {
        quote! { i64 }
    }

    pub fn add_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident + other.#ident,
        }
    }

    pub fn sub_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident - other.#ident,
        }
    }

    pub fn neg_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: -self.#ident,
        }
    }

    pub fn mul_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident * other as i64,
        }
    }

    pub fn sqrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident / 2,
        }
    }

    pub fn cbrt_entry(&self, ident: &Ident) -> TokenStream {
        quote! {
            #ident: self.#ident / 3,
        }
    }

    pub fn sqrt_safety(&self, ident: &Ident) -> TokenStream {
        quote! {
            if self.#ident % 2 != 0 {
                panic!("Cannot take square root of quantity with a dimension that is not divisible by 2 in all components.");
            }
        }
    }

    pub fn cbrt_safety(&self, ident: &Ident) -> TokenStream {
        quote! {
            if self.#ident % 3 != 0 {
                panic!("Cannot take cubic root of quantity with a dimension that is not divisible by 3 in all components.");
            }
        }
    }
}
