use ::diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[alias(metres)]
    #[metric_prefixes]
    #[symbol(m)]
    unit meters: Length;

    #[prefix(kilo)]
    unit foo: Length = 0.25 * meters;
);

macro_rules! gen_tests_for_float {
    ($float_name: ident, $mod_name: ident, $assert_is_close: path, $assert_is_close_float: path) => {
        mod $mod_name {
            use super::dimensions::Length;
            use super::units;
            use crate::make_annotated_unit_constructor;
            make_annotated_unit_constructor!(meters, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(metres, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(centimeters, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(centimetres, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(kilometers, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(foo, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(kilofoo, Length<$float_name>, $float_name);

            #[test]
            fn unit_aliases() {
                assert_eq!(meters(100.0), metres(100.0));
                let x = meters(100.0);
                assert_eq!(x.value_in(units::meters), 100.0);
                assert_eq!(x.value_in(units::metres), 100.0);
            }

            #[test]
            fn prefixed_aliases() {
                assert_eq!(centimeters(100.0), centimetres(100.0));
                let x = centimeters(100.0);
                assert_eq!(x.value_in(units::meters), 1.0);
                assert_eq!(x.value_in(units::metres), 1.0);
                assert_eq!(x.value_in(units::centimeters), 100.0);
                assert_eq!(x.value_in(units::centimetres), 100.0);
            }

            #[test]
            fn explicit_prefix() {
                assert_eq!(foo(100.0), meters(25.0));
                assert_eq!(kilofoo(100.0), kilometers(25.0));
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
