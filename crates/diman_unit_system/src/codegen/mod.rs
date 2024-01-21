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
mod quantity_type;
#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "serde")]
mod serde;
mod storage_types;
mod unit_type;
mod units_and_constants;
mod vector_methods;

use proc_macro2::TokenStream;
use quote::quote;

use crate::types::Defs;

pub enum CallerType {
    /// The macro is called from within this crate (`diman_unit_system`)
    /// and imports need to be directly from `diman_lib`.
    #[allow(dead_code)]
    Internal,
    /// The macro is called from somewhere else (`diman` or a user's crate)
    /// and imports need to be from `diman`.
    External,
}

impl CallerType {
    fn path_prefix(&self) -> TokenStream {
        match self {
            CallerType::Internal => quote! { ::diman_lib },
            CallerType::External => quote! { ::diman::internal },
        }
    }
}

pub struct Codegen {
    pub defs: Defs,
    pub caller_type: CallerType,
}

fn join<const D: usize>(streams: [TokenStream; D]) -> TokenStream {
    streams.into_iter().collect()
}

impl Codegen {
    pub fn code_gen(&self) -> TokenStream {
        join([
            self.gen_dimension(),
            self.gen_quantity(),
            self.gen_dimensions(),
            self.gen_units_and_constants(),
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
