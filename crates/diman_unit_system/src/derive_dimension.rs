use quote::quote;
use syn::Ident;

use crate::types::Defs;

fn str_to_snakecase(s: &str) -> String {
    let s = s.chars().rev().collect::<String>();
    let words = s.split_inclusive(|c: char| c.is_uppercase());
    words
        .map(|word| word.chars().rev().collect::<String>().to_lowercase())
        .rev()
        .collect::<Vec<_>>()
        .join("_")
}

fn to_snakecase(dim: &proc_macro2::Ident) -> Ident {
    let snake_case = str_to_snakecase(&dim.to_string());
    Ident::new(&snake_case, dim.span())
}

impl Defs {
    pub(crate) fn dimension_impl(&self) -> proc_macro::TokenStream {
        let name = &self.dimension_type;
        let dimensions = &self.dimensions;

        let dimensions: proc_macro2::TokenStream = dimensions
            .iter()
            .map(|dim| {
                let name = to_snakecase(dim);
                quote! {
                    #name: i32,
                }
            })
            .collect();
        let methods_impl: proc_macro2::TokenStream = self.dimension_methods_impl().into();
        let output = quote! {
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug, ::std::marker::ConstParamTy)]
            pub struct #name {
                #dimensions
            }

            #methods_impl
        };
        output.into()
    }

    pub(crate) fn dimension_methods_impl(&self) -> proc_macro::TokenStream {
        let type_name = &self.dimension_type;
        let iter_dimensions = || self.dimensions.iter().map(|x| to_snakecase(&x));
        let none_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
                quote! {
                    #ident: 0,
                }
            })
            .collect();

        let mul_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
                quote! {
                    #ident: self.#ident + other.#ident,
                }
            })
            .collect();

        let div_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
                quote! {
                    #ident: self.#ident - other.#ident,
                }
            })
            .collect();

        let inv_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
                quote! {
                    #ident: -self.#ident,
                }
            })
            .collect();

        let powi_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
                quote! {
                    #ident: self.#ident * other,
                }
            })
            .collect();

        let sqrt_safety_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
            quote! {
                if self.#ident % 2 != 0 {
                    panic!("Cannot take square root of quantity with a dimension that is not divisible by 2 in all components.");
                }
            }
        }).collect();

        let sqrt_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
                quote! {
                    #ident: self.#ident / 2,
                }
            })
            .collect();

        let cbrt_safety_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
            quote! {
                if self.#ident % 3 != 0 {
                    panic!("Cannot take cubic root of quantity with a dimension that is not divisible by 3 in all components.");
                }
            }
        }).collect();

        let cbrt_gen: proc_macro2::TokenStream = iter_dimensions()
            .map(|ident| {
                quote! {
                    #ident: self.#ident / 3,
                }
            })
            .collect();

        let gen = quote! {
            impl #type_name {
                pub const fn none() -> Self {
                    Self {
                        #none_gen
                    }
                }

                pub const fn dimension_mul(self, other: Self) -> Self {
                    Self {
                        #mul_gen
                    }
                }

                pub const fn dimension_div(self, other: Self) -> Self {
                    Self {
                        #div_gen
                    }
                }

                pub const fn dimension_inv(self) -> Self {
                    Self {
                        #inv_gen
                    }
                }

                pub const fn dimension_powi(self, other: i32) -> Self {
                    Self {
                        #powi_gen
                    }
                }

                pub const fn dimension_sqrt(self) -> Self {
                    #sqrt_safety_gen
                    Self {
                        #sqrt_gen
                    }
                }

                pub const fn dimension_cbrt(self) -> Self {
                    #cbrt_safety_gen
                    Self {
                        #cbrt_gen
                    }
                }
            }
        };
        gen.into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn str_to_snakecase() {
        assert_eq!(super::str_to_snakecase("MyType"), "my_type".to_owned());
        assert_eq!(super::str_to_snakecase("My"), "my".to_owned());
        assert_eq!(
            super::str_to_snakecase("MyVeryLongType"),
            "my_very_long_type".to_owned()
        );
    }
}
