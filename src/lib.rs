#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![doc = include_str!("../README.md")]

mod debug_storage_type;
mod type_aliases;

#[cfg(feature = "si")]
/// Defines the proper dimensions for the SI system. The unit definitions
/// are primarily used for the doctests now and not complete in any sense.
pub mod si;

pub use debug_storage_type::DebugStorageType;
pub use diman_unit_system::dimension;
pub use diman_unit_system::unit_system;
pub use type_aliases::Product;
pub use type_aliases::QProduct;
pub use type_aliases::Quotient;
