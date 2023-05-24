#[cfg(feature = "f64")]
pub(crate) fn assert_is_close<const U: crate::example_system::Dimension>(
    x: crate::example_system::Quantity<f64, U>,
    y: crate::example_system::Quantity<f64, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f64::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}

pub(crate) fn assert_is_close_float(x: f64, y: f64) {
    assert!((x - y).abs() < f64::EPSILON, "{} {}", x, y)
}
