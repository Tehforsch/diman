#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]
#![doc = include_str!("../README.md")]

mod debug_storage_type;
mod type_aliases;

pub use debug_storage_type::DebugStorageType;
pub use derive_dimension::diman_dimension;
pub use type_aliases::Product;
pub use type_aliases::QProduct;
pub use type_aliases::Quotient;
pub use unit_system::unit_system;
