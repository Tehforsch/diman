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

pub fn to_snakecase(dim: &proc_macro2::Ident) -> Ident {
    let snake_case = str_to_snakecase(&dim.to_string());
    Ident::new(&snake_case, dim.span())
}

impl Defs {
    pub(crate) fn dimension_impl(&self) -> proc_macro::TokenStream {
        let name = &self.dimension_type;

        let dim_type = self.base_dimension_type();
        let dimensions: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|dim| {
                quote! {
                    #dim: #dim_type,
                }
            })
            .collect();
        let methods_impl: proc_macro2::TokenStream = self.dimension_methods_impl().into();
        let ratio_impl = self.ratio_impl();
        let output = quote! {
            #ratio_impl
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
        let none_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.zero(ident))
            .collect();

        let mul_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.add(ident))
            .collect();

        let div_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.sub(ident))
            .collect();

        let inv_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.neg(ident))
            .collect();

        let powi_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.mul(ident))
            .collect();

        let sqrt_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.sqrt(ident))
            .collect();

        let cbrt_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.cbrt(ident))
            .collect();

        let sqrt_safety_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.sqrt_safety(ident))
            .collect();

        let cbrt_safety_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.cbrt_safety(ident))
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

    #[cfg(not(feature = "rational-dimensions"))]
    fn ratio_impl(&self) -> proc_macro2::TokenStream {
        quote! {}
    }

    #[cfg(feature = "rational-dimensions")]
    fn ratio_impl(&self) -> proc_macro2::TokenStream {
        quote! {
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug, ::std::marker::ConstParamTy)]
            struct Ratio {
                num: i32,
                denom: i32,
            }

            const fn gcd(mut a: i32, mut b: i32) -> i32 {
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                a.abs()
            }

            impl Ratio  {
                const fn int(num: i32) -> Self {
                    Self { num, denom: 1 }
                }

                const fn new(num: i32, denom: i32) -> Self {
                    let gcd = gcd(num, denom);
                    Ratio {
                        num: num / gcd,
                        denom: denom / gcd,
                    }
                }

                const fn add(self, rhs: Self) -> Self {
                    let num = self.num * rhs.denom + rhs.num * self.denom;
                    let denom = self.denom * rhs.denom;
                    Self::new(num, denom)
                }

                const fn sub(self, rhs: Self) -> Self {
                    self.add(rhs.neg())
                }

                const fn neg(self) -> Self {
                    Self {
                        num: -self.num,
                        denom: self.denom,
                    }
                }

                const fn mul(self, rhs: Self) -> Self {
                    let num = self.num * rhs.num;
                    let denom = self.denom * rhs.denom;
                    Self::new(num, denom)
                }

                const fn div(self, rhs: Self) -> Self {
                    self.mul(rhs.inv())
                }

                const fn inv(self) -> Self {
                    Self {
                        num: self.denom,
                        denom: self.num,
                    }
                }

            }
        }
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
