use proc_macro2::TokenStream;
use quote::quote;

use crate::{storage_types::FloatType, types::Defs, utils::join};

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

    pub(crate) fn numeric_traits(&self) -> TokenStream {
        join([
            self.add_sub_impl(
                quote! {std::ops::Add},
                quote! {add},
                quote! {Self(self.0 + rhs.0)},
            ),
            self.add_sub_impl(
                quote! {std::ops::Sub},
                quote! {sub},
                quote! {Self(self.0 - rhs.0)},
            ),
            self.add_sub_assign_impl(
                quote! {std::ops::AddAssign},
                quote! {add_assign},
                quote! {self.0 += rhs.0;},
            ),
            self.add_sub_assign_impl(
                quote! {std::ops::SubAssign},
                quote! {sub_assign},
                quote! {self.0 -= rhs.0;},
            ),
            self.neg_impl(),
            self.mul_impls(),
            self.div_impls(),
        ])
    }

    fn add_sub_impl(
        &self,
        trait_type: TokenStream,
        fn_name: TokenStream,
        inner_code: TokenStream,
    ) -> TokenStream {
        let Self { quantity_type, .. } = &self;
        let output_type_def = quote! {
            type Output = #quantity_type<S, D>;
        };
        self.generic_impl(
            trait_type,
            quote! {Output = S},
            quote! {Self},
            fn_name,
            quote! {self, rhs: Self},
            inner_code,
            output_type_def,
        )
    }

    fn add_sub_assign_impl(
        &self,
        trait_type: TokenStream,
        fn_name: TokenStream,
        inner_code: TokenStream,
    ) -> TokenStream {
        self.generic_impl(
            trait_type,
            quote! {S},
            quote! {()},
            fn_name,
            quote! {&mut self, rhs: Self},
            inner_code,
            quote! {},
        )
    }

    fn neg_impl(&self) -> TokenStream {
        let Self { quantity_type, .. } = &self;
        self.generic_impl(
            quote! {std::ops::Neg},
            quote! {Output = S},
            quote! {Self},
            quote! {neg},
            quote! {self},
            quote! {Self(-self.0)},
            quote! {type Output = #quantity_type<S, D>;},
        )
    }

    fn generic_impl(
        &self,
        trait_type: TokenStream,
        inner_trait_spec: TokenStream,
        return_type: TokenStream,
        fn_name: TokenStream,
        fn_args: TokenStream,
        inner_code: TokenStream,
        output_type_def: TokenStream,
    ) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            impl<S, const D: #dimension_type> #trait_type for #quantity_type<S, D>
            where
                S: #trait_type<#inner_trait_spec>,
            {
                #output_type_def
                fn #fn_name(#fn_args) -> #return_type {
                    #inner_code
                }
            }
        }
    }

    fn mul_impls(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| self.mul_quantity_quantity_impl(float_type))
            .collect()
    }

    fn div_impls(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| self.div_quantity_quantity_impl(float_type))
            .collect()
    }

    fn mul_quantity_quantity_impl(&self, float_type: &FloatType) -> TokenStream {
        self.mul_div_quantity_quantity_impl(
            float_type,
            quote! { std::ops::Mul },
            quote! { mul },
            quote! { dimension_mul },
            quote! { self.0 * rhs.0},
        )
    }

    fn div_quantity_quantity_impl(&self, float_type: &FloatType) -> TokenStream {
        self.mul_div_quantity_quantity_impl(
            float_type,
            quote! { std::ops::Div },
            quote! { div },
            quote! { dimension_div },
            quote! { self.0 / rhs.0},
        )
    }

    fn mul_div_quantity_quantity_impl(
        &self,
        float_type: &FloatType,
        trait_name: TokenStream,
        fn_name: TokenStream,
        dimension_fn: TokenStream,
        expression: TokenStream,
    ) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        let type_lhs = &float_type.name;
        let type_rhs = &float_type.name;
        quote! {
            impl<const DL: #dimension_type, const DR: #dimension_type> #trait_name<#quantity_type<#type_rhs, DR>>
                for #quantity_type<#type_lhs, DL>
            where
                #quantity_type<#type_lhs, { DL.#dimension_fn(DR) }>:,
            {
                type Output = #quantity_type<
                    <#type_lhs as #trait_name<#type_rhs>>::Output,
                    { DL.#dimension_fn(DR) },
                >;

                fn #fn_name(self, rhs: #quantity_type<#type_rhs, DR>) -> Self::Output {
                    #quantity_type(#expression)
                }
            }
        }
    }
}
