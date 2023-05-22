use proc_macro2::TokenStream;
use quote::quote;

use crate::{types::Defs, utils::join};

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
        join([ self.add_sub_impl(
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
         self.neg_impl()])
    }

    fn add_sub_impl(
        &self,
        trait_type: TokenStream,
        fn_name: TokenStream,
        inner_code: TokenStream,
    ) -> TokenStream {
        let Self { quantity_type, .. } = &self;
        let output_type_def = 
            quote! {
                type Output = #quantity_type<S, D>;
            };
        self.generic_impl(
            trait_type,
            quote! {Output = S},
            quote! {Self},
            fn_name,
            quote! {self, rhs: Self},
            inner_code,
            output_type_def
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
            quote! {}
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
}
