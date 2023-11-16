#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman::dimension;
use diman::unit_system;

#[dimension]
pub struct Dimension {
    pub length: i32,
}

unit_system!(
    Quantity,
    Dimension,
    [
        def Length = { length: 1 },
        unit (kilometers, "km") = 1000.0 * meters,
    ]
);

fn main() {}
