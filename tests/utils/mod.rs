#[cfg(feature = "f32")]
pub(crate) fn assert_is_close_f32<const U: crate::example_system::Dimension>(
    x: crate::example_system::Quantity<f32, U>,
    y: crate::example_system::Quantity<f32, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f32::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}

#[cfg(feature = "f64")]
pub(crate) fn assert_is_close_f64<const U: crate::example_system::Dimension>(
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

#[cfg(feature = "f32")]
pub(crate) fn assert_is_close_float_f32(x: f32, y: f32) {
    assert!((x - y).abs() < f32::EPSILON, "{} {}", x, y)
}

#[cfg(feature = "f64")]
pub(crate) fn assert_is_close_float_f64(x: f64, y: f64) {
    assert!((x - y).abs() < f64::EPSILON, "{} {}", x, y)
}
