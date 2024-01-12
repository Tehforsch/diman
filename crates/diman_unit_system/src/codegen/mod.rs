mod base_dimension_type;
mod debug_trait;
mod dimension_type;
mod dimensions;
mod float_methods;
mod generic_methods;
#[cfg(feature = "hdf5")]
mod hdf5;
#[cfg(feature = "mpi")]
mod mpi;
mod num_traits;
#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "serde")]
mod serde;
mod storage_types;
mod units;
mod vector_methods;

use proc_macro2::TokenStream;

use crate::types::Defs;

fn join<const D: usize>(streams: [TokenStream; D]) -> TokenStream {
    streams.into_iter().collect()
}

impl Defs {
    pub fn code_gen(&self) -> TokenStream {
        join([
            self.gen_dimension(),
            self.gen_quantity(),
            self.gen_definitions_for_storage_types(),
            self.gen_unit_constructors(),
            self.gen_numeric_trait_impls(),
            self.gen_debug_trait_impl(),
            self.gen_float_methods(),
            self.gen_vector_methods(),
            self.gen_generic_methods(),
            #[cfg(feature = "serde")]
            self.gen_serde_impl(),
            #[cfg(feature = "hdf5")]
            self.gen_hdf5_impl(),
            #[cfg(feature = "mpi")]
            self.gen_mpi_impl(),
            #[cfg(feature = "rand")]
            self.gen_rand_impl(),
        ])
    }
}
