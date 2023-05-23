mod utils;
mod float_methods;
mod generic_methods;
mod vector_methods;
mod debug;
#[cfg(feature="mpi")]
mod mpi;
#[cfg(feature="serde")]
mod serde;
#[cfg(feature="rand")]
mod rand;
#[cfg(feature="hdf5")]
mod hdf5;
mod unit_constructors;
mod traits;
pub mod type_defs;

use proc_macro2::TokenStream;

use crate::types::Defs;

use self::utils::join;

impl Defs {
    pub fn code_gen(&self) -> TokenStream {
        join([
            self.type_definition(),
            self.type_functions(),
            self.float_quantity_definitions(),
            self.vector_quantity_definitions(),
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
