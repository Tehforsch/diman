use ::diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[alias(metres)]
    unit meters: Length;
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
