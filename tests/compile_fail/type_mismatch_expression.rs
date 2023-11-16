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
        def Length = { length: 1 },
        unit (meters, "m") = Length,

    ]
);

use crate::f64::*;

fn main() {
    let x: () = Length::meters(1.0) / Length::meters(1.0);
}
