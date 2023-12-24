#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman::unit_system;

unit_system!(
    quantity_type Quantity,
    dimension_type Dimension,
    dimension Length,
    unit meters: Length,
    unit kilometers = 1000.0 * meters,
    unit millimeters = 0.001 * meters,
    unit undefined = undefined,
);

fn main() {
    use crate::f64::*;
    let l = Length::meters(1.0);
}
