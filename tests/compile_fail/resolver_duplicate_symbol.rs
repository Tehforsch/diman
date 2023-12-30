#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[prefix(milli)]
    #[symbol(m)]
    unit meters: Length;
    #[symbol(mm)]
    unit my_unit: Length = meters * 10.0;
);

fn main() {}
