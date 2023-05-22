use proc_macro2::TokenStream;
use quote::quote;

use crate::types::Defs;

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
        let add_impl = self.add_sub_impl(
            quote! {std::ops::Add},
            quote! {add},
            quote! {Self(self.0 + rhs.0)},
        );
        let sub_impl = self.add_sub_impl(
            quote! {std::ops::Sub},
            quote! {sub},
            quote! {Self(self.0 - rhs.0)},
        );
        let add_assign_impl = self.add_sub_assign_impl(
            quote! {std::ops::AddAssign},
            quote! {add_assign},
            quote! {self.0 += rhs.0;},
        );
        let sub_assign_impl = self.add_sub_assign_impl(
            quote! {std::ops::SubAssign},
            quote! {sub_assign},
            quote! {self.0 -= rhs.0;},
        );
        quote! {
            #add_impl
            #add_assign_impl
            #sub_impl
            #sub_assign_impl
        }
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
            quote! {Self},
            fn_name,
            inner_code,
            quote! {}
        )
    }

    fn generic_impl(
        &self,
        trait_type: TokenStream,
        inner_trait_spec: TokenStream,
        return_type: TokenStream,
        fn_name: TokenStream,
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
                fn #fn_name(self, rhs: Self) -> #return_type {
                    #inner_code
                }
            }
        }
    }
}
