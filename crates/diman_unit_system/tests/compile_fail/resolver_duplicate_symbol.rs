#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_args, generic_const_items, min_generic_const_args)]

use diman_unit_system::unit_system_internal;

unit_system_internal!(
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
