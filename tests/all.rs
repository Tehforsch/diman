#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod example_system;
pub mod utils;

mod float;

mod type_aliases;

mod dimension_defs;

pub mod unit_aliases;

#[cfg(feature = "f64")]
mod gas;

#[cfg(feature = "glam")]
mod glam;

#[cfg(feature = "mpi")]
mod mpi;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rand")]
mod rand;

#[test]
#[cfg(feature = "f32")]
fn compile_fail_float() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/float_*.rs");
}

#[test]
#[cfg(feature = "glam-vec2")]
#[cfg(feature = "f32")]
fn compile_fail_glam() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/glam_*.rs");
}

#[test]
fn compile_fail_resolver() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/resolver_*.rs");
}

#[test]
fn compile_fail_type_mismatch() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/type_mismatch_*.rs");
}

#[test]
fn compile_fail_dimension_def_numeric_factor() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/dimension_definition_with_numeric_factor.rs");
}

#[test]
fn compile_fail_dimension_annotation() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/dimension_annotation*.rs");
}
