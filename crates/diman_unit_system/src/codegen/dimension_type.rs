use proc_macro2::TokenStream;
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
        let ratio_impl = self.ratio_impl();
        quote! {
            #ratio_impl
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug, ::core::marker::ConstParamTy)]
            pub struct #name {
                #dimensions
            }

            #methods_impl
        }
    }

    fn dimension_methods_impl(&self) -> TokenStream {
        let type_name = &self.dimension_type;
        let none_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.zero_entry(ident))
            .collect();

        let mul_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.add_entry(ident))
            .collect();

        let div_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.sub_entry(ident))
            .collect();

        let inv_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.neg_entry(ident))
            .collect();

        let powi_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.mul_entry(ident))
            .collect();

        let sqrt_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.sqrt_entry(ident))
            .collect();

        let cbrt_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.cbrt_entry(ident))
            .collect();

        let sqrt_safety_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.sqrt_safety(ident))
            .collect();

        let cbrt_safety_gen: proc_macro2::TokenStream = self
            .base_dimensions()
            .map(|ident| self.cbrt_safety(ident))
            .collect();

        quote! {
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
        }
    }

    #[cfg(not(feature = "rational-dimensions"))]
    fn ratio_impl(&self) -> proc_macro2::TokenStream {
        quote! {}
    }

    /// Defines the `Ratio` type inside the calling crate.
    /// This is done to improve error messages, since the
    /// messages would otherwise show diman::Ratio everywhere.
    #[cfg(feature = "rational-dimensions")]
    fn ratio_impl(&self) -> proc_macro2::TokenStream {
        quote! {
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug, ::core::marker::ConstParamTy)]
            struct Ratio {
                num: i64,
                denom: i64,
            }

            const fn gcd(mut a: i64, mut b: i64) -> i64 {
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                a.abs()
            }

            impl Ratio  {
                const fn int(num: i64) -> Self {
                    Self { num, denom: 1 }
                }

                const fn new(num: i64, denom: i64) -> Self {
                    let gcd = gcd(num, denom);
                    Self {
                        num: num / gcd,
                        denom: denom / gcd,
                    }
                }

                pub const fn powi(self, exp: i32) -> Self {
                    let num = self.num * exp as i64;
                    let denom = self.denom * exp as i64;
                    Self::new(num, denom)
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

            impl core::fmt::Display for Ratio {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    if self.denom == 1 {
                        write!(f, "{}", self.num)
                    } else {
                        write!(f, "{}/{}", self.num, self.denom)
                    }
                }
            }

        }
    }
}