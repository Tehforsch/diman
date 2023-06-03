#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
pub mod example_system;

fn main() {
    use example_system::vec2::Length;
    Length::unknown_method(49.0);
}
