// These just need to compile.
macro_rules! gen_tests_for_float {
    ($float_name: ident, $mod_name: ident) => {
        mod $mod_name {
            use crate::example_system::dimensions::Length;
            use crate::example_system::dimensions::Time;
            use diman::Product;
            use diman::Quotient;

            #[allow(unused)]
            fn product_1(
                length: Length<$float_name>,
                time: Time<$float_name>,
            ) -> Product<Length<$float_name>, Time<$float_name>> {
                length * time
            }

            #[allow(unused)]
            fn quotient_1(
                length: Length<$float_name>,
                time: Time<$float_name>,
            ) -> Quotient<Length<$float_name>, Time<$float_name>> {
                length / time
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32, f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64, f64);
