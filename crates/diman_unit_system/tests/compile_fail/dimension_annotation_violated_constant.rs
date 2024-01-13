#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

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
