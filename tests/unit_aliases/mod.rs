use ::diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[alias(metres)]
    #[metric_prefixes]
    unit meters: Length;

    #[prefix(kilo)]
    unit foo: Length = 0.25 * meters;
);

macro_rules! gen_tests_for_float {
    ($float_name: ident, $mod_name: ident, $assert_is_close: path, $assert_is_close_float: path) => {
        mod $mod_name {
            use super::$float_name::Length;

            #[test]
            fn unit_aliases() {
                assert_eq!(Length::meters(100.0), Length::metres(100.0));
                let x = Length::meters(100.0);
                assert_eq!(x.in_meters(), 100.0);
                assert_eq!(x.in_metres(), 100.0);
            }

            #[test]
            fn prefixed_aliases() {
                assert_eq!(Length::centimeters(100.0), Length::centimetres(100.0));
                let x = Length::centimeters(100.0);
                assert_eq!(x.in_meters(), 1.0);
                assert_eq!(x.in_metres(), 1.0);
                assert_eq!(x.in_centimeters(), 100.0);
                assert_eq!(x.in_centimetres(), 100.0);
            }

            #[test]
            fn explicit_prefix() {
                assert_eq!(Length::foo(100.0), Length::meters(25.0));
                assert_eq!(Length::kilofoo(100.0), Length::kilometers(25.0));
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(
    f32,
    mod_f32,
    crate::utils::assert_is_close_f32,
    crate::utils::assert_is_close_float_f32
);

#[cfg(feature = "f64")]
gen_tests_for_float!(
    f64,
    mod_f64,
    crate::utils::assert_is_close_f64,
    crate::utils::assert_is_close_float_f64
);
