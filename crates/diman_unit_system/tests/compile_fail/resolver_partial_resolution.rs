#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_args, generic_const_items, min_generic_const_args)]

use diman_unit_system::unit_system_internal;

unit_system_internal!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[symbol(m)]
    unit meters;
    unit kilometers = 1000.0 * meters;
    unit millimeters = 0.001 * meters;
    unit undefined = undefined;
);

fn main() {
    use crate::units::meters;
    let _ = 1.0 * meters;
}
