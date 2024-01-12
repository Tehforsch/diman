#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman_unit_system::unit_system;
unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Mass;
    constant SOLAR_MASS: Mass = 1.988477e30 * Mass;
);

fn main() {
}
