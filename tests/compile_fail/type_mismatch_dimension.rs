#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
pub mod example_system;
use example_system::f64::*;

fn main() {
    let x: Time = Length::meters(1.0);
}
