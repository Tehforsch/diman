#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[symbol(m)]
    unit meters: Length;
    #[base(Length)]
    #[symbol(pc)]
    unit parsec: Length;
);

fn main() {}
