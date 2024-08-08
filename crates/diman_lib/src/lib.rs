#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

pub mod dimension_exponent;
pub mod magnitude;
#[cfg(any(feature = "std", feature = "num-traits-libm"))]
pub mod ratio;
pub mod runtime_unit_storage;

pub mod num_traits_reexport {
    #[cfg(feature = "num-traits-libm")]
    pub use num_traits::float::Float;
    #[cfg(not(feature = "num-traits-libm"))]
    pub use num_traits::float::FloatCore;
}
