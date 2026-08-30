#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_args, generic_const_items, min_generic_const_args)]

use diman_unit_system::unit_system_internal;
unit_system_internal!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Mass;
    #[base(Mass)]
    #[symbol(kg)]
    unit kilograms: Mass;
    unit grams = 1e-3 * kilograms;
    constant SOLAR_MASS: Mass = 1.988477e30 * kilograms * grams;
);

fn main() {
}
