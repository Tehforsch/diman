mod base_dimension_type;
mod debug;
mod dimension_def;
mod float_methods;
mod generic_methods;
#[cfg(feature = "hdf5")]
mod hdf5;
#[cfg(feature = "mpi")]
mod mpi;
#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "serde")]
mod serde;
mod traits;
pub mod type_defs;
mod unit_constructors;
mod vector_methods;

use proc_macro2::TokenStream;

use crate::types::Defs;

fn join<const D: usize>(streams: [TokenStream; D]) -> TokenStream {
    streams.into_iter().collect()
}

impl Defs {
    pub fn code_gen(&self) -> TokenStream {
        join([
            self.dimension_impl(),
            self.quantity_definition(),
            self.quantity_functions(),
            self.definitions_for_storage_types(),
            self.unit_constructors(),
            self.impl_numeric_traits(),
            self.debug_trait(),
            self.float_methods(),
            self.vector_methods(),
            self.generic_methods(),
            #[cfg(feature = "serde")]
            self.serde_impl(),
            #[cfg(feature = "hdf5")]
            self.hdf5_impl(),
            #[cfg(feature = "mpi")]
            self.mpi_impl(),
            #[cfg(feature = "rand")]
            self.rand_impl(),
        ])
    }
}
