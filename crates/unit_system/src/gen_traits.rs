use proc_macro2::TokenStream;
use quote::quote;

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
            impl_generics: quote! { const D: #dimension_type, S },
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
            impl_generics: quote! { const DL: #dimension_type, const DR: #dimension_type, LHS, RHS },
            rhs,
            lhs,
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

    fn iter_numeric_traits(&self) -> impl Iterator<Item = NumericTrait> {
        let Self { quantity_type, .. } = self;
        vec![
            NumericTrait::add_or_sub_quantity_quantity(
                &self,
                quote! { std::ops::Add },
                quote! { add },
                quote! { Self(self.0 + rhs.0) },
            ),
            NumericTrait::add_or_sub_quantity_quantity(
                &self,
                quote! { std::ops::Sub },
                quote! { sub },
                quote! { Self(self.0 - rhs.0) },
            ),
            NumericTrait::add_or_sub_assign_quantity_quantity(
                &self,
                quote! { std::ops::AddAssign },
                quote! { add_assign },
                quote! { self.0 += rhs.0; },
            ),
            NumericTrait::add_or_sub_assign_quantity_quantity(
                &self,
                quote! { std::ops::SubAssign },
                quote! { sub_assign },
                quote! { self.0 -= rhs.0; },
            ),
            NumericTrait::mul_or_div_quantity_quantity(
                &self,
                quote! { std::ops::Mul },
                quote! { mul },
                quote! { #quantity_type(self.0 * rhs.0) },
                quote! { dimension_mul },
            ),
            NumericTrait::mul_or_div_quantity_quantity(
                &self,
                quote! { std::ops::Div },
                quote! { div },
                quote! { #quantity_type(self.0 / rhs.0) },
                quote! { dimension_div },
            ),
        ]
        .into_iter()
    }

    pub fn numeric_traits(&self) -> TokenStream {
        self.iter_numeric_traits()
            .into_iter()
            .map(|num_trait| self.generic_numeric_trait_impl(num_trait))
            .collect()
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
            impl<#impl_generics> #name::<#rhs> for #lhs
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
}
