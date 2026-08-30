#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_args, generic_const_items, min_generic_const_args)]

use diman_unit_system::unit_system_internal;
unit_system_internal!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Mass;
    #[base(Mass)]
    #[symbol(kg)]
    unit kilograms;
    unit foo: kilograms = kilograms;
);

fn main() {
}
