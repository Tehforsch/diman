#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_args, generic_const_items, min_generic_const_args)]
pub mod example_system;
use example_system::units::*;

fn main() {
    let x: () = meters.new(1.0) * meters.new(1.0);
}
