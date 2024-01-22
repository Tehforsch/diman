#[cfg(any(feature = "glam-vec2", feature = "glam-dvec2"))]
macro_rules! gen_tests_for_vector_2 {
    ($float_name: ident, $mod_name: ident, $vec_name: ty, $assert_is_close: path) => {
        mod $mod_name {
            use crate::example_system::dimensions::{Length, Time};
            use crate::example_system::units::{self, meters_per_second};
            use crate::make_annotated_unit_constructor;
            use $assert_is_close as assert_is_close;

            make_annotated_unit_constructor!(meters, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(seconds, Time<$float_name>, $float_name);

            use $vec_name as Vec;
            #[test]
            fn debug_vector_2() {
                assert_eq!(
                    format!("{:?}", Vec::new(1.0, 5.0) * meters(1.0)),
                    "[1, 5] m"
                );
            }

            #[test]
            fn mul_vec2() {
                let multiplied = Vec::new(1.0, 2.0) * meters(5.0);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
                let multiplied: Length<Vec> = meters(5.0) * Vec::new(1.0, 2.0);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
            }

            #[test]
            fn mul_quantity_vec2() {
                let multiplied = (Vec::new(1.0, 2.0) * meters_per_second) * seconds(5.0);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
                let multiplied = seconds(5.0) * (Vec::new(1.0, 2.0) * meters_per_second);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
            }

            #[test]
            fn div_vec2() {
                let divided = Vec::new(1.0, 2.0) / meters(0.2);
                let base = 1.0 / meters(1.0);
                assert_is_close(divided.x(), 5.0 * base);
                assert_is_close(divided.y(), 10.0 * base);
            }
        }
    };
}

#[cfg(any(feature = "glam-vec3", feature = "glam-dvec3"))]
macro_rules! gen_tests_for_vector_3 {
    ($float_name: ident, $mod_name: ident, $vec_name: ty, $assert_is_close: path) => {
        mod $mod_name {
            use crate::example_system::dimensions::{Length, Time};
            use crate::example_system::units::{self, meters_per_second};
            use crate::make_annotated_unit_constructor;
            use $assert_is_close as assert_is_close;

            make_annotated_unit_constructor!(meters, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(seconds, Time<$float_name>, $float_name);

            use $vec_name as Vec;
            #[test]
            fn debug_vector_3() {
                assert_eq!(
                    format!("{:?}", Vec::new(1.0, 5.0, 10.0) * meters(1.0)),
                    "[1, 5, 10] m"
                );
            }

            #[test]
            fn mul_vec3() {
                let multiplied = Vec::new(1.0, 2.0, 3.0) * meters(5.0);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
                assert_is_close(multiplied.z(), meters(15.0));
                let multiplied: Length<Vec> = meters(5.0) * Vec::new(1.0, 2.0, 3.0);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
                assert_is_close(multiplied.z(), meters(15.0));
            }

            #[test]
            fn mul_quantity_vec3() {
                let multiplied = (Vec::new(1.0, 2.0, 3.0) * meters_per_second) * seconds(5.0);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
                assert_is_close(multiplied.z(), meters(15.0));
                let multiplied = seconds(5.0) * (Vec::new(1.0, 2.0, 3.0) * meters_per_second);
                assert_is_close(multiplied.x(), meters(5.0));
                assert_is_close(multiplied.y(), meters(10.0));
                assert_is_close(multiplied.z(), meters(15.0));
            }

            #[test]
            fn div_vec3() {
                let divided = Vec::new(1.0, 2.0, 3.0) / meters(0.2);
                let base = 1.0 / meters(1.0);
                assert_is_close(divided.x(), 5.0 * base);
                assert_is_close(divided.y(), 10.0 * base);
                assert_is_close(divided.z(), 15.0 * base);
            }
        }
    };
}

#[cfg(all(feature = "f32", feature = "glam-vec2"))]
gen_tests_for_vector_2!(f32, vec2, glam::Vec2, crate::utils::assert_is_close_f32);

#[cfg(all(feature = "f64", feature = "glam-dvec2"))]
gen_tests_for_vector_2!(f64, dvec2, glam::DVec2, crate::utils::assert_is_close_f64);

#[cfg(all(feature = "f32", feature = "glam-vec3"))]
gen_tests_for_vector_3!(f32, vec3, glam::Vec3, crate::utils::assert_is_close_f32);

#[cfg(all(feature = "f64", feature = "glam-dvec3"))]
gen_tests_for_vector_3!(f64, dvec3, glam::DVec3, crate::utils::assert_is_close_f64);
