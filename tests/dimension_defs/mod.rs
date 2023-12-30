use crate::example_system::f64::Area;
use crate::example_system::f64::InverseTemperature;
use crate::example_system::f64::Length;
use crate::example_system::f64::Temperature;
use crate::example_system::f64::Volume;
use crate::utils::assert_is_close_f64;

#[test]
fn one_over_dimension_def() {
    let _: InverseTemperature = 1.0 / Temperature::kelvins(5.0);
}

#[test]
fn exponent_dimension_def() {
    let vol: Volume = Area::square_meters(10.0) * Length::meters(2.0);
    let vol2: Volume = Volume::cubic_meters(20.0);
    assert_is_close_f64(vol, vol2);
}
