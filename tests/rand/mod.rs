macro_rules! gen_tests_for_float {
    ($float_name: ident) => {
        mod $float_name {
            use rand::Rng;

            use crate::example_system::dimensions::Length;
            use crate::example_system::units;
            use crate::make_annotated_unit_constructor;

            make_annotated_unit_constructor!(meters, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(kilometers, Length<$float_name>, $float_name);

            #[test]
            fn test_random_quantity_generation() {
                let mut rng = rand::thread_rng();
                for _ in 0..100 {
                    let x = rng.gen_range(meters(0.0)..kilometers(1.0));
                    assert!(meters(0.0) <= x);
                    assert!(x < meters(1000.0));
                }
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64);
