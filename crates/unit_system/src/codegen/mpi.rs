use proc_macro2::TokenStream;

use crate::types::Defs;

impl Defs {
    pub fn mpi_impl(&self) -> TokenStream {
        TokenStream::new()
    }
}
