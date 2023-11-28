#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
pub mod example_system;
use example_system::f64::*;

fn main() {
    let x: () = 1.0 / Dimensionless::dimensionless(1.0);
}
