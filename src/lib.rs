#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![doc = include_str!("../README.md")]

mod debug_storage_type;
mod type_aliases;

#[cfg(feature = "si")]
pub mod si;

pub use debug_storage_type::DebugStorageType;
pub use derive_dimension::dimension;
pub use type_aliases::Product;
pub use type_aliases::QProduct;
pub use type_aliases::Quotient;
pub use unit_system::unit_system;
