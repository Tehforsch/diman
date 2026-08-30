#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_args, generic_const_items, min_generic_const_args)]
pub mod example_system;

fn main() {
    use example_system::dimensions::Length;
    Length::unknown_method(49.0);
}
