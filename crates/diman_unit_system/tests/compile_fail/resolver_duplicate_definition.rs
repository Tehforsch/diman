#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman_unit_system::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[symbol(m)]
    unit meters: Length;
    unit foo: Length = 10.0 * meters;
    unit foo: Length = 20.0 * meters;
);

fn main() {}
