use crate::example_system::dimensions::InverseTemperature;
use crate::example_system::dimensions::Volume;
use crate::example_system::units::{cubic_meters, kelvins, meters, square_meters};
use crate::utils::assert_is_close_f64;

#[test]
fn one_over_dimension_def() {
    let _: InverseTemperature<f64> = 1.0 / (5.0 * kelvins);
}

#[test]
fn exponent_dimension_def() {
    let vol: Volume<f64> = 20.0 * square_meters * meters;
    let vol2: Volume<f64> = 20.0 * cubic_meters;
    assert_is_close_f64(vol, vol2);
}
