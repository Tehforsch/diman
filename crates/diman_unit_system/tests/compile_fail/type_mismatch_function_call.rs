#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
pub mod example_system;
use example_system::units::meters;

fn main() {
    let x: () = meters.new(1.0f64).abs();
}
