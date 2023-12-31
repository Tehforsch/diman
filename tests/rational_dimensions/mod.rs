macro_rules! gen_tests_for_float {
    ($float_name: ident) => {
        mod $float_name {
            use crate::example_system::$float_name::Length;
            use crate::example_system::$float_name::Time;

            #[test]
            fn rational_dimensions_allowed() {
                let x = Length::meters(1.0);
                let y = Time::seconds(1.0);
                let z = x.cbrt() * y;
                dbg!(z.powi::<3>() / y.powi::<3>());
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64);
