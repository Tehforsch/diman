#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_args, generic_const_items, min_generic_const_args)]
pub mod example_system;
use example_system::units::meters;
use example_system::dimensions::Time;

fn main() {
    let x: Time<f64> = meters.new(1.0);
}
