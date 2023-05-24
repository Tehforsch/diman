macro_rules! gen_tests_for_float {
    ($float_name: ident) => {
        mod $float_name {
            use rand::Rng;

            use crate::example_system::$float_name::Length;

            #[test]
            fn test_random_quantity_generation() {
                let mut rng = rand::thread_rng();
                for _ in 0..100 {
                    let x = rng.gen_range(Length::meters(0.0)..Length::kilometers(1.0));
                    assert!(Length::meters(0.0) <= x);
                    assert!(x < Length::meters(1000.0));
                }
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64);
