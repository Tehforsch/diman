use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::types::Defs;

#[derive(Default)]
struct NumericTrait {
    name: TokenStream,
    fn_name: TokenStream,
    fn_return_type: TokenStream,
    fn_args: TokenStream,
    fn_return_expr: TokenStream,
    trait_bound_impl: TokenStream,
    output_type_def: TokenStream,
    impl_generics: TokenStream,
    rhs: TokenStream,
    lhs: TokenStream,
}

impl NumericTrait {
    fn additive_quantity_quantity_defaults(defs: &Defs) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        Self {
            impl_generics: quote! { < const D: #dimension_type, S > },
            rhs: quote! { #quantity_type<S, D> },
            lhs: quote! { #quantity_type<S, D> },
            ..Default::default()
        }
    }

    /// For an impl of Add or Sub between two quantities
    fn add_or_sub_quantity_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            fn_return_type: quote! { Self },
            fn_args: quote! {self, rhs: Self},
            trait_bound_impl: quote! {S: #name<Output = S>},
            output_type_def: quote! { type Output = Self; },
            ..Self::additive_quantity_quantity_defaults(defs)
        }
    }

    /// For an impl of AddAssign or SubAssign between two quantities
    fn add_or_sub_assign_quantity_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            fn_return_type: quote! {()},
            fn_args: quote! {&mut self, rhs: Self},
            output_type_def: quote! {},
            trait_bound_impl: quote! {S: #name<S>},
            ..Self::additive_quantity_quantity_defaults(defs)
        }
    }

    /// For an impl of Add or Sub between a dimensionless quantity and a storage type
    fn add_or_sub_quantity_type(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        Self {
            impl_generics: quote! { < S > },
            rhs: quote! { S },
            lhs: quote! { #quantity_type<S, { #dimension_type::none() }> },
            fn_args: quote! {self, rhs: S},
            ..Self::add_or_sub_quantity_quantity(defs, name, fn_name, fn_return_expr)
        }
    }

    /// For an impl of AddAssign or SubAssign between a dimensionless quantity and a storage type
    fn add_or_sub_assign_quantity_type(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        Self {
            impl_generics: quote! { < S > },
            rhs: quote! { S },
            lhs: quote! { #quantity_type<S, { #dimension_type::none() }> },
            fn_args: quote! {&mut self, rhs: S},
            ..Self::add_or_sub_assign_quantity_quantity(defs, name, fn_name, fn_return_expr)
        }
    }

    /// For an impl of Add or Sub between a storage type and a dimensionless quantity
    fn add_or_sub_type_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_inner_return_expr: TokenStream,
        storage_type: &Type,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let quantity = quote! { #quantity_type::<#storage_type, { #dimension_type::none() }> };
        let fn_return_expr = quote! { #quantity( #fn_inner_return_expr ) };
        Self {
            impl_generics: quote! {},
            lhs: quote! { #storage_type },
            rhs: quantity.clone(),
            fn_args: quote! {self, rhs: #quantity},
            output_type_def: quote! { type Output = #quantity; },
            fn_return_type: quantity,
            name,
            fn_name,
            fn_return_expr,
            trait_bound_impl: quote! {},
        }
    }

    /// For an impl of AddAssign or SubAssign between a storage type and a dimensionless quantity
    fn add_or_sub_assign_type_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let quantity = quote! { #quantity_type::<#storage_type, { #dimension_type::none() }> };
        Self {
            impl_generics: quote! {},
            lhs: quote! { #storage_type },
            rhs: quantity.clone(),
            fn_args: quote! {&mut self, rhs: #quantity},
            output_type_def: quote! {},
            fn_return_type: quote! {()},
            name,
            fn_name,
            fn_return_expr,
            trait_bound_impl: quote! {},
        }
    }

    /// For an impl of Mul or Div between two quantities
    fn mul_or_div_quantity_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
        dimension_fn: TokenStream,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let lhs = quote! { #quantity_type<LHS, DL> };
        let rhs = quote! { #quantity_type<RHS, DR> };
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            fn_return_type: quote! { #quantity_type< <LHS as #name<RHS>>::Output, { DL.#dimension_fn(DR) }> },
            fn_args: quote! { self, rhs: #rhs },
            trait_bound_impl: quote! {
                LHS: #name<RHS>,
                #quantity_type<LHS, { DL.#dimension_fn(DR) }>:,
            },
            output_type_def: quote! {
                type Output = #quantity_type<
                    <LHS as #name<RHS>>::Output,
                    { DL.#dimension_fn(DR) },
                >;
            },
            impl_generics: quote! { < const DL: #dimension_type, const DR: #dimension_type, LHS, RHS > },
            rhs,
            lhs,
        }
    }

    /// For an impl of Mul or Div between a quantity and a concrete storage type
    fn mul_or_div_quantity_type(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let lhs = quote! { #quantity_type<LHS, D> };
        let rhs = quote! { #storage_type };
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            lhs,
            rhs: rhs.clone(),
            fn_return_type: quote! { Self::Output },
            fn_args: quote! { self, rhs: #rhs },
            trait_bound_impl: quote! {
                LHS: #name<#storage_type>,
            },
            output_type_def: quote! {
                type Output = #quantity_type<
                    <LHS as #name<#storage_type>>::Output,
                    D,
                >;
            },
            impl_generics: quote! { < const D: #dimension_type, LHS >},
        }
    }

    /// For an impl of Mul or Div between a concrete storage type and a quantity
    fn div_type_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        let Defs { quantity_type, .. } = defs;
        Self {
            trait_bound_impl: quote! {
                #storage_type: #name<RHS>,
                #quantity_type<#storage_type, { D.dimension_inv() }>:,
            },
            output_type_def: quote! {
                type Output = #quantity_type<
                    <#storage_type as #name<RHS>>::Output,
                    { D.dimension_inv() },
                >;
            },
            ..Self::mul_type_quantity(defs, name, fn_name, fn_return_expr, storage_type)
        }
    }

    /// For an impl of Mul or Div between a concrete storage type and a quantity
    fn mul_type_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let rhs = quote! { #quantity_type<RHS, D> };
        let lhs = quote! { #storage_type };
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            lhs,
            rhs: rhs.clone(),
            fn_return_type: quote! { Self::Output },
            fn_args: quote! { self, rhs: #rhs },
            trait_bound_impl: quote! {
                #storage_type: #name<RHS>,
            },
            output_type_def: quote! {
                type Output = #quantity_type<
                    <#storage_type as #name<RHS>>::Output,
                    D,
                >;
            },
            impl_generics: quote! { < const D: #dimension_type, RHS >},
        }
    }

    /// For an impl of MulAssign or DivAssign between two quantities (only for
    /// dimensionless right hand side)
    fn mul_or_div_assign_quantity_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let lhs = quote! { #quantity_type<LHS, DL> };
        let rhs = quote! { #quantity_type<RHS, { #dimension_type::none() }> };
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            fn_return_type: quote! {()},
            fn_args: quote! { &mut self, rhs: #rhs },
            trait_bound_impl: quote! {
                LHS: #name<RHS>,
            },
            output_type_def: quote! {},
            impl_generics: quote! { < const DL: #dimension_type, LHS, RHS > },
            rhs,
            lhs,
        }
    }

    /// For an impl of MulAssign or DivAssign between a quantity and a storage type
    fn mul_or_div_assign_quantity_type(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
        rhs: &Type,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let lhs = quote! { #quantity_type<LHS, DL> };
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            fn_return_type: quote! {()},
            fn_args: quote! { &mut self, rhs: #rhs },
            trait_bound_impl: quote! {
                LHS: #name<#rhs>,
            },
            output_type_def: quote! {},
            impl_generics: quote! { < const DL: #dimension_type, LHS > },
            rhs: quote! { #rhs },
            lhs,
        }
    }

    /// For an impl of MulAssign or DivAssign between a quantity and a storage type
    fn mul_or_div_assign_type_quantity(
        defs: &Defs,
        name: TokenStream,
        fn_name: TokenStream,
        fn_return_expr: TokenStream,
        lhs: &Type,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let rhs = quote! { #quantity_type<RHS, D> };
        Self {
            name: name.clone(),
            fn_name,
            fn_return_expr,
            fn_return_type: quote! {()},
            fn_args: quote! { &mut self, rhs: #rhs },
            trait_bound_impl: quote! {
                #lhs: #name<RHS>,
            },
            output_type_def: quote! {},
            impl_generics: quote! { < const D: #dimension_type, RHS > },
            rhs,
            lhs: quote! { #lhs },
        }
    }
}

impl Defs {
    pub(crate) fn qproduct_trait(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            impl<S, const D: #dimension_type> diman::QProduct for #quantity_type<S, D> {
                type Output = #quantity_type<S, D>;
            }
        }
    }

    fn iter_numeric_traits(&self) -> impl Iterator<Item = NumericTrait> + '_ {
        let Self { quantity_type, .. } = self;
        vec![
            NumericTrait::add_or_sub_quantity_quantity(
                self,
                quote! { std::ops::Add },
                quote! { add },
                quote! { Self(self.0 + rhs.0) },
            ),
            NumericTrait::add_or_sub_quantity_quantity(
                self,
                quote! { std::ops::Sub },
                quote! { sub },
                quote! { Self(self.0 - rhs.0) },
            ),
            NumericTrait::add_or_sub_assign_quantity_quantity(
                self,
                quote! { std::ops::AddAssign },
                quote! { add_assign },
                quote! { self.0 += rhs.0; },
            ),
            NumericTrait::add_or_sub_assign_quantity_quantity(
                self,
                quote! { std::ops::SubAssign },
                quote! { sub_assign },
                quote! { self.0 -= rhs.0; },
            ),
            NumericTrait::add_or_sub_quantity_type(
                self,
                quote! { std::ops::Add },
                quote! { add },
                quote! { Self(self.0 + rhs) },
            ),
            NumericTrait::add_or_sub_quantity_type(
                self,
                quote! { std::ops::Sub },
                quote! { sub },
                quote! { Self(self.0 - rhs) },
            ),
            NumericTrait::add_or_sub_assign_quantity_type(
                self,
                quote! { std::ops::AddAssign },
                quote! { add_assign },
                quote! { self.0 += rhs; },
            ),
            NumericTrait::add_or_sub_assign_quantity_type(
                self,
                quote! { std::ops::SubAssign },
                quote! { sub_assign },
                quote! { self.0 -= rhs; },
            ),
            NumericTrait::mul_or_div_quantity_quantity(
                self,
                quote! { std::ops::Mul },
                quote! { mul },
                quote! { #quantity_type(self.0 * rhs.0) },
                quote! { dimension_mul },
            ),
            NumericTrait::mul_or_div_quantity_quantity(
                self,
                quote! { std::ops::Div },
                quote! { div },
                quote! { #quantity_type(self.0 / rhs.0) },
                quote! { dimension_div },
            ),
            NumericTrait::mul_or_div_assign_quantity_quantity(
                self,
                quote! { std::ops::MulAssign },
                quote! { mul_assign },
                quote! { self.0 *= rhs.0; },
            ),
            NumericTrait::mul_or_div_assign_quantity_quantity(
                self,
                quote! { std::ops::DivAssign },
                quote! { div_assign },
                quote! { self.0 /= rhs.0; },
            ),
        ]
        .into_iter()
        .chain(
            self.storage_type_names()
                .into_iter()
                .flat_map(move |storage_type| {
                    [
                        NumericTrait::mul_or_div_quantity_type(
                            self,
                            quote! { std::ops::Mul },
                            quote! { mul },
                            quote! { #quantity_type(self.0 * rhs) },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_quantity_type(
                            self,
                            quote! { std::ops::Div },
                            quote! { div },
                            quote! { #quantity_type(self.0 / rhs) },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_quantity_type(
                            self,
                            quote! { std::ops::MulAssign },
                            quote! { mul_assign },
                            quote! { self.0 *= rhs; },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_quantity_type(
                            self,
                            quote! { std::ops::DivAssign },
                            quote! { div_assign },
                            quote! { self.0 /= rhs; },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_type_quantity(
                            self,
                            quote! { std::ops::MulAssign },
                            quote! { mul_assign },
                            quote! { *self *= rhs.0; },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_type_quantity(
                            self,
                            quote! { std::ops::DivAssign },
                            quote! { div_assign },
                            quote! { *self /= rhs.0; },
                            &storage_type,
                        ),
                        NumericTrait::mul_type_quantity(
                            self,
                            quote! { std::ops::Mul },
                            quote! { mul },
                            quote! { #quantity_type(self * rhs.0) },
                            &storage_type,
                        ),
                        NumericTrait::div_type_quantity(
                            self,
                            quote! { std::ops::Div },
                            quote! { div },
                            quote! { #quantity_type(self / rhs.0) },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_type_quantity(
                            self,
                            quote! { std::ops::Add },
                            quote! { add },
                            quote! { self + rhs.0 },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_type_quantity(
                            self,
                            quote! { std::ops::Sub },
                            quote! { sub },
                            quote! { self - rhs.0 },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_assign_type_quantity(
                            self,
                            quote! { std::ops::AddAssign },
                            quote! { add_assign },
                            quote! { *self += rhs.0; },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_assign_type_quantity(
                            self,
                            quote! { std::ops::SubAssign },
                            quote! { sub_assign },
                            quote! { *self -= rhs.0; },
                            &storage_type,
                        ),
                    ]
                    .into_iter()
                }),
        )
    }

    pub fn numeric_traits(&self) -> TokenStream {
        let ops: TokenStream = self
            .iter_numeric_traits()
            .map(|num_trait| self.generic_numeric_trait_impl(num_trait))
            .collect();
        let sum = self.impl_sum();
        let neg = self.impl_neg();
        quote! {
            #ops
            #sum
            #neg
        }
    }

    fn generic_numeric_trait_impl(&self, numeric_trait: NumericTrait) -> TokenStream {
        let NumericTrait {
            name,
            fn_name,
            fn_return_type,
            fn_args,
            trait_bound_impl,
            fn_return_expr,
            output_type_def,
            impl_generics,
            rhs,
            lhs,
        } = &numeric_trait;
        quote! {
            impl #impl_generics #name::<#rhs> for #lhs
            where
                #trait_bound_impl
            {
                #output_type_def
                fn #fn_name(#fn_args) -> #fn_return_type {
                    #fn_return_expr
                }
            }
        }
    }

    fn impl_sum(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<const D: #dimension_type, S: Default + std::ops::AddAssign<S>> std::iter::Sum
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

    fn impl_neg(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<const D: #dimension_type, S: std::ops::Neg<Output=S>> std::ops::Neg for #quantity_type<S, D> {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self(-self.0)
                }
            }
        }
    }
}
