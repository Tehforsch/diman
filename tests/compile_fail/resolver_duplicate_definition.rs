#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman::unit_system;

unit_system!(
    Quantity,
    Dimension { Length },
    [
        def Length = { length: 1 },
        unit (meters, "m") = Length,
        unit (meters, "m") = Length,
        unit (kilometers, "km") = 1000.0 * meters,
    ]
);

fn main() {}
