#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod example_system;
pub mod utils;

mod basic;

mod type_aliases;

#[cfg(test)]
#[cfg(all(
    feature = "glam",
    feature = "glam-dvec2",
    feature = "glam-dvec3",
    feature = "f64"
))]
mod glam;

#[cfg(feature = "mpi")]
mod mpi;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rand")]
mod rand;
