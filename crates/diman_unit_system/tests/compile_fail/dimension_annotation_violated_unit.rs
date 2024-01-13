#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman_unit_system::unit_system_internal;
unit_system_internal!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Mass;
    dimension Length;
    #[base(Mass)]
    #[symbol(kg)]
    unit kilograms: Mass;
    unit foo: Length = kilograms * kilograms;
);

fn main() {
}
