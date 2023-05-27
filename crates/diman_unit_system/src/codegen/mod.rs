mod debug;
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
mod utils;
mod vector_methods;

use proc_macro2::TokenStream;

use crate::types::Defs;

use self::utils::join;

impl Defs {
    pub fn code_gen(&self) -> TokenStream {
        join([
            self.type_definition(),
            self.type_functions(),
            self.float_definitions(),
            self.vector_definitions(),
            self.unit_constructors(),
            self.qproduct_trait(),
            self.numeric_traits(),
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
