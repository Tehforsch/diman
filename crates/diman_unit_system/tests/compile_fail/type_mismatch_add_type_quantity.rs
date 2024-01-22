#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
pub mod example_system;
use example_system::units::dimensionless;

fn main() {
    let x: () = 1.0 + dimensionless.new(1.0);
}
