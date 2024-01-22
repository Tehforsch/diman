#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
pub mod example_system;
use example_system::units::meters;
use example_system::dimensions::Time;

fn main() {
    let x: Time<f64> = meters.new(1.0);
}
