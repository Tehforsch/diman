#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

use diman::{dimension, unit_system};

#[dimension]
pub struct Dimension {
    time: i32,
    length: i32,
}

#[rustfmt::skip]
unit_system!(
    Quantity,
    Dimension,
    [
        def Dimensionless = {},
        unit dimensionless = Dimensionless,
    ]
);

use crate::f64::*;

fn main() {
    let x: () = 1.0 * Dimensionless::dimensionless(1.0);
}
