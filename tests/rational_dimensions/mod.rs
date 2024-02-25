diman::unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    dimension Time;
    dimension Sorptivity = Length Time^(-1/2);

    #[base(Time)]
    #[prefix(milli)]
    #[symbol(s)]
    unit seconds;

    #[base(Length)]
    #[prefix(micro)]
    #[symbol(m)]
    unit meters;

    unit meters_per_sqrt_second: Sorptivity = meters / seconds^(1/2);
);

macro_rules! gen_tests_for_float {
    ($mod_name: ident, $float_name: ident, $assert_is_close: path) => {
        mod $mod_name {
            use super::dimensions::{Length, Sorptivity, Time};
            use super::units;
            use crate::make_annotated_unit_constructor;
            make_annotated_unit_constructor!(micrometers, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(milliseconds, Time<$float_name>, $float_name);
            make_annotated_unit_constructor!(
                meters_per_sqrt_second,
                Sorptivity<$float_name>,
                $float_name
            );
            #[test]
            fn rational_dimensions_allowed() {
                let l = micrometers(2.0);
                let t = milliseconds(5.0);
                let sorptivity: Sorptivity<$float_name> = l / t.sqrt();
                let val = l.value_unchecked() / t.value_unchecked().sqrt();
                $assert_is_close(
                    sorptivity.value_unchecked(),
                    meters_per_sqrt_second(val).value_unchecked(),
                );
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(mod_f32, f32, crate::utils::assert_is_close_float_f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(mod_f64, f64, crate::utils::assert_is_close_float_f64);
