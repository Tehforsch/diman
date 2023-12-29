use ::diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[alias(metres)]
    unit meters: Length;
);

#[test]
fn unit_aliases() {
    use self::f64::Length;

    assert_eq!(Length::meters(100.0), Length::metres(100.0));
}
