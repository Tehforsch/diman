use crate::example_system::f64::InverseTemperature;
use crate::example_system::f64::Temperature;

#[test]
fn one_over_quantity_def() {
    let _: InverseTemperature = 1.0 / Temperature::kelvins(5.0);
}
