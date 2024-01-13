use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use super::Codegen;

impl Codegen {
    pub fn gen_generic_methods(&self) -> TokenStream {
        self.storage_type_names()
            .map(|name| self.impl_method_for_generic_storage_type(&name, &quote! { abs }))
            .collect()
    }

    fn impl_method_for_generic_storage_type(
        &self,
        storage_type: &Type,
        name: &TokenStream,
    ) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        quote! {
            impl<const D: #dimension_type> #quantity_type<#storage_type, D> {
                pub fn #name(&self) -> #quantity_type<#storage_type, D> {
                    Self(self.0.#name())
                }
            }
        }
    }
}
