#[cfg(feature = "default-f32")]
pub(crate) fn assert_is_close<const U: crate::si::Dimension>(
    x: crate::si::Quantity<f32, U>,
    y: crate::si::Quantity<f32, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f32::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}

#[cfg(feature = "default-f64")]
pub(crate) fn assert_is_close<const U: crate::si::Dimension>(
    x: crate::si::Quantity<f64, U>,
    y: crate::si::Quantity<f64, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f64::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}
