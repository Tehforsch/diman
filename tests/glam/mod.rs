#[cfg(any(feature = "glam-vec2", feature = "glam-dvec2"))]
macro_rules! gen_tests_for_vector_2 {
    ($float_name: ident, $mod_name: ident, $vec_name: ty, $assert_is_close: path) => {
        mod $mod_name {
            use crate::example_system::$float_name::Length;
            use crate::example_system::$float_name::Time;
            use crate::example_system::$mod_name::Length as VecLength;
            use crate::example_system::$mod_name::Velocity as VecVelocity;
            use $assert_is_close as assert_is_close;
            use $vec_name as Vec;

            #[test]
            fn debug_vector_2() {
                assert_eq!(format!("{:?}", VecLength::meters(1.0, 5.0)), "[1, 5] m");
            }

            #[test]
            fn mul_vec2() {
                let multiplied = Vec::new(1.0, 2.0) * Length::meters(5.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
                let multiplied = Length::meters(5.0) * Vec::new(1.0, 2.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
            }

            // #[test]
            // fn mul_assign_vec2() {
            //     let mut vec = VecLength::meters(1.0, 2.0);
            //     vec *= 3.0;
            //     assert_is_close(vec.x(), Length::meters(3.0));
            //     assert_is_close(vec.y(), Length::meters(6.0));
            // }

            // #[test]
            // fn div_assign_vec3() {
            //     let mut vec = Vec2Length::meters(1.0, 2.0);
            //     vec /= 2.0;
            //     assert_is_close(vec.x(), Length::meters(0.5));
            //     assert_is_close(vec.y(), Length::meters(1.0));
            // }

            #[test]
            fn mul_quantity_vec2() {
                let multiplied = VecVelocity::meters_per_second(1.0, 2.0) * Time::seconds(5.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
                let multiplied = Time::seconds(5.0) * VecVelocity::meters_per_second(1.0, 2.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
            }

            #[test]
            fn div_vec2() {
                let divided = Vec::new(1.0, 2.0) / Length::meters(0.2);
                let base = 1.0 / Length::meters(1.0);
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
            use crate::example_system::$float_name::Length;
            use crate::example_system::$float_name::Time;
            use crate::example_system::$mod_name::Length as VecLength;
            use crate::example_system::$mod_name::Velocity as VecVelocity;
            use $assert_is_close as assert_is_close;
            use $vec_name as Vec;

            #[test]
            fn debug_vector_3() {
                assert_eq!(
                    format!("{:?}", VecLength::meters(1.0, 5.0, 6.0)),
                    "[1, 5, 6] m"
                );
            }

            #[test]
            fn mul_vec3() {
                let multiplied = Vec::new(1.0, 2.0, 3.0) * Length::meters(5.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
                assert_is_close(multiplied.z(), Length::meters(15.0));
                let multiplied = Length::meters(5.0) * Vec::new(1.0, 2.0, 3.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
                assert_is_close(multiplied.z(), Length::meters(15.0));
            }

            // #[test]
            // fn mul_assign_vec3() {
            //     let mut vec = Vec3Length::meters(1.0, 2.0, 3.0);
            //     vec *= 3.0;
            //     assert_is_close(vec.x(), Length::meters(3.0));
            //     assert_is_close(vec.y(), Length::meters(6.0));
            //     assert_is_close(vec.z(), Length::meters(9.0));
            // }

            // #[test]
            // fn div_assign_vec3() {
            //     let mut vec = Vec3Length::meters(1.0, 2.0, 3.0);
            //     vec /= 2.0;
            //     assert_is_close(vec.x(), Length::meters(0.5));
            //     assert_is_close(vec.y(), Length::meters(1.0));
            //     assert_is_close(vec.z(), Length::meters(1.5));
            // }

            #[test]
            fn mul_quantity_vec3() {
                let multiplied = VecVelocity::meters_per_second(1.0, 2.0, 3.0) * Time::seconds(5.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
                assert_is_close(multiplied.z(), Length::meters(15.0));
                let multiplied = Time::seconds(5.0) * VecVelocity::meters_per_second(1.0, 2.0, 3.0);
                assert_is_close(multiplied.x(), Length::meters(5.0));
                assert_is_close(multiplied.y(), Length::meters(10.0));
                assert_is_close(multiplied.z(), Length::meters(15.0));
            }

            #[test]
            fn div_vec3() {
                let divided = Vec::new(1.0, 2.0, 3.0) / Length::meters(0.2);
                let base = 1.0 / Length::meters(1.0);
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
