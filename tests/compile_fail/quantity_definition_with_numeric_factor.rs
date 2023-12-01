#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use ::diman::unit_system;

unit_system!(
    Quantity,
    Dimension {},
    [
        def Dimensionless = {},
        def Length = 2.0 / Dimensionless,
    ]
);

fn main() {
}
