use proc_macro2::TokenStream;

use crate::types::Defs;

impl Defs {
    pub fn hdf5_impl(&self) -> TokenStream {
        TokenStream::new()
    }
}
