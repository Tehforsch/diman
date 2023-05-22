use proc_macro2::TokenStream;
use quote::quote;

use crate::{storage_types::FloatType, types::Defs, utils::join};

struct NumericTrait {
    name: TokenStream,
    fn_name: TokenStream,
    fn_return_type: TokenStream,
    fn_args: TokenStream,
    fn_return_expr: TokenStream,
    trait_bound_impl: TokenStream,
    output_type_def: TokenStream,
    inner_trait_spec: TokenStream,
    impl_generics: TokenStream,
    const_generic_bound: TokenStream,
    rhs: TokenStream,
    lhs: TokenStream,
}

impl NumericTrait {
    fn add_or_sub(
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
            name,
            fn_name,
            fn_return_expr,
            fn_return_type: quote! { Self },
            fn_args: quote! {self, rhs: Self},
            trait_bound_impl: quote! {},
            inner_trait_spec: quote! {Output = S},
            output_type_def: quote! { type Output = Self; },
            impl_generics: quote! { const D: #dimension_type, S },
            const_generic_bound: quote! {},
            rhs: quote! { #quantity_type<S, D> },
            lhs: quote! { #quantity_type<S, D> },
        }
    }

    fn add_or_sub_assign(
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
            name,
            fn_name,
            fn_return_expr,
            fn_return_type: quote! {()},
            fn_args: quote! {&mut self, rhs: Self},
            trait_bound_impl: quote! {},
            inner_trait_spec: quote! {S},
            output_type_def: quote! {},
            impl_generics: quote! { const D: #dimension_type, S },
            const_generic_bound: quote! {},
            rhs: quote! { #quantity_type<S, D> },
            lhs: quote! { #quantity_type<S, D> },
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
        // join([
        //     self.add_sub_impl(
        //         quote! {std::ops::Add},
        //         quote! {add},
        //         quote! {Self(self.0 + rhs.0)},
        //     ),
        //     self.add_sub_impl(
        //         quote! {std::ops::Sub},
        //         quote! {sub},
        //         quote! {Self(self.0 - rhs.0)},
        //     ),
        //     self.add_sub_assign_impl(
        //         quote! {std::ops::AddAssign},
        //         quote! {add_assign},
        //         quote! {self.0 += rhs.0;},
        //     ),
        //     self.add_sub_assign_impl(
        //         quote! {std::ops::SubAssign},
        //         quote! {sub_assign},
        //         quote! {self.0 -= rhs.0;},
        //     ),
        //     self.neg_impl(),
        //     self.quantity_quantity_mul_impl(),
        //     self.quantity_quantity_mul_assign_impl(),
        //     self.quantity_quantity_div_impl(),
        // ])
        vec![
            NumericTrait::add_or_sub(
                &self,
                quote! { std::ops::Add },
                quote! { add },
                quote! { Self(self.0 + rhs.0) },
            ),
            NumericTrait::add_or_sub(
                &self,
                quote! { std::ops::Sub },
                quote! { sub },
                quote! { Self(self.0 - rhs.0) },
            ),
            NumericTrait::add_or_sub_assign(
                &self,
                quote! { std::ops::AddAssign },
                quote! { add_assign },
                quote! { self.0 += rhs.0; },
            ),
            NumericTrait::add_or_sub_assign(
                &self,
                quote! { std::ops::SubAssign },
                quote! { sub_assign },
                quote! { self.0 -= rhs.0; },
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
            inner_trait_spec,
            output_type_def,
            impl_generics,
            rhs,
            lhs,
            const_generic_bound,
        } = &numeric_trait;
        quote! {
            impl<#impl_generics> #name<#rhs> for #lhs
            where
                S: #name<#inner_trait_spec>,
                #const_generic_bound
                #trait_bound_impl
            {
                #output_type_def
                fn #fn_name(#fn_args) -> #fn_return_type {
                    #fn_return_expr
                }
            }
        }
    }

    // // Generate an impl like Quantity<DimL, L> * Quantity<DimR, R> = Quantity<DimL * DimR, L * R>
    // fn mul_div_assign_generic_quantity_quantity_impl(
    //     &self,
    //     trait_name: TokenStream,
    //     fn_name: TokenStream,
    //     dimension_fn: TokenStream,
    //     expression: TokenStream,
    // ) -> TokenStream {
    //     let Self {
    //         quantity_type,
    //         dimension_type,
    //         ..
    //     } = &self;
    //     quote! {
    //         impl<const DL: #dimension_type, const DR: #dimension_type, LHS, RHS> #trait_name<#quantity_type<RHS, DR>>
    //             for #quantity_type<LHS, DL>
    //         where
    //             LHS: #trait_name<RHS>,
    //             #quantity_type<LHS, { DL.#dimension_fn(DR) }>:,
    //         {
    //             type Output = #quantity_type<
    //                 <LHS as #trait_name<RHS>>::Output,
    //                 { DL.#dimension_fn(DR) },
    //             >;

    //             fn #fn_name(#fn_args) -> #fn_return_type {
    //                 #fn_return_expr
    //             }
    //         }
    //     }
    // }

    // // Generate an impl like Quantity<Dim, L> * R = Quantity<Dim, L * R>
    // fn mul_div_quantity_storage_impl(
    //     &self,
    //     trait_name: TokenStream,
    //     fn_name: TokenStream,
    //     dimension_fn: TokenStream,
    //     expression: TokenStream,
    // ) -> TokenStream {
    //     let Self {
    //         quantity_type,
    //         dimension_type,
    //         ..
    //     } = &self;
    //     quote! {
    //         impl<const DL: #dimension_type, LHS, RHS> #trait_name<RHS>
    //             for #quantity_type<LHS, DL>
    //         where
    //             LHS: #trait_name<RHS>,
    //         {
    //             type Output = #quantity_type<
    //                 <LHS as #trait_name<RHS>>::Output,
    //                 { DL.#dimension_fn(DR) },
    //             >;

    //             fn #fn_name(self, rhs: #quantity_type<RHS, DR>) -> Self::Output {
    //                 #quantity_type(#expression)
    //             }
    //         }
    //     }
    // }
}
