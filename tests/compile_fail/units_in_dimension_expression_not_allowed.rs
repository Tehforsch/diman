#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use ::diman::unit_system;
unit_system!(
    quantity_type Quantity,
    dimension_type Dimension,
    dimension Mass,
    unit kilograms: Mass,
    dimension Mass2 = kilograms,
);

fn main() {
}
