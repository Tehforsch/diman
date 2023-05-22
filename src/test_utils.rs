#[cfg(feature = "f32")]
pub(crate) fn assert_is_close<const U: crate::test_system::Dimension>(
    x: crate::test_system::Quantity<f32, U>,
    y: crate::test_system::Quantity<f32, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f32::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}

#[cfg(feature = "f64")]
pub(crate) fn assert_is_close<const U: crate::test_system::Dimension>(
    x: crate::test_system::Quantity<f64, U>,
    y: crate::test_system::Quantity<f64, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f64::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}
