#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman_unit_system::unit_system_internal;

unit_system_internal!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Dimensionless = 1;
    dimension Length = 2.0 / Dimensionless;
);

fn main() {
}
