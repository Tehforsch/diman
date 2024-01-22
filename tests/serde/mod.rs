#[cfg(any(feature = "f32", feature = "f64"))]
macro_rules! gen_tests_for_float {
    ($float_name: ident, $assert_is_close: path) => {
        mod $float_name {
            use crate::example_system::dimensions::{Energy, Length, Time, Velocity};
            use crate::example_system::units;
            use crate::make_annotated_unit_constructor;
            use $assert_is_close as assert_is_close;
            make_annotated_unit_constructor!(meters, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(kilometers, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(seconds, Time<$float_name>, $float_name);
            make_annotated_unit_constructor!(joules, Energy<$float_name>, $float_name);

            #[test]
            fn deserialize_float() {
                let q: Length<$float_name> = serde_yaml::from_str("5.0 km").unwrap();
                assert_is_close(q, kilometers(5.0));
                let q: Velocity<$float_name> = serde_yaml::from_str("5.0 km s^-1").unwrap();
                assert_is_close(q, kilometers(5.0) / seconds(1.0));
            }

            #[test]
            #[should_panic(expected = "mismatch in dimensions")]
            fn deserialize_float_dimension_mismatch() {
                let q: Length<$float_name> = serde_yaml::from_str("5.0 kg").unwrap();
                assert_is_close(q, kilometers(5.0));
            }

            #[test]
            fn serialize_float() {
                assert_eq!(
                    serde_yaml::to_string(&kilometers(5.0)).unwrap().trim(),
                    "5000 m"
                );
                assert_eq!(serde_yaml::to_string(&joules(5.0)).unwrap().trim(), "5 J");
            }

            #[test]
            fn serialize_float_unnamed_dimension() {
                let unnamed_dimension = joules(5.0) * meters(1.0);
                assert_eq!(
                    serde_yaml::to_string(&unnamed_dimension).unwrap().trim(),
                    "5 m^3 s^-2 kg"
                );
            }
        }
    };
}

#[cfg(any(feature = "glam-vec2", feature = "glam-dvec2"))]
macro_rules! gen_tests_for_vector_2 {
    ($float_name: ident, $mod_name: ident, $vec_name: ty, $assert_is_close: path) => {
        mod $mod_name {
            use crate::example_system::dimensions::Length;
            use crate::example_system::units;
            use crate::example_system::units::{dimensionless, meters};
            use $assert_is_close as assert_is_close;
            use $vec_name as Vec2;

            use crate::make_annotated_unit_constructor;
            make_annotated_unit_constructor!(kilometers, Length<$float_name>, $float_name);

            #[test]
            fn deserialize_vector() {
                let q: Length<Vec2> = serde_yaml::from_str("(5.0 3.0) km").unwrap();
                assert_is_close(q.x(), kilometers(5.0));
                assert_is_close(q.y(), kilometers(3.0));
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_fewer_than_2_components() {
                let _: Length<Vec2> = serde_yaml::from_str("(5.0) km").unwrap();
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_more_than_2_components() {
                let _: Length<Vec2> = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
            }

            #[test]
            fn serialize_vector() {
                let x = meters * Vec2::new(5.3, 1.1);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1) m\n");
            }

            #[test]
            fn serialize_dimensionless_vector() {
                let x = dimensionless * Vec2::new(5.3, 1.1);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1)\n");
            }
        }
    };
}

#[cfg(any(feature = "glam-vec3", feature = "glam-dvec3"))]
macro_rules! gen_tests_for_vector_3 {
    ($float_name: ident, $mod_name: ident, $vec_name: ty, $assert_is_close: path) => {
        mod $mod_name {
            use crate::example_system::dimensions::Length;
            use crate::example_system::units;
            use crate::example_system::units::{dimensionless, meters};
            use $assert_is_close as assert_is_close;
            use $vec_name as Vec3;

            use crate::make_annotated_unit_constructor;
            make_annotated_unit_constructor!(kilometers, Length<$float_name>, $float_name);

            #[test]
            fn deserialize_vector() {
                let q: Length<Vec3> = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
                assert_is_close(q.x(), kilometers(5.0));
                assert_is_close(q.y(), kilometers(3.0));
                assert_is_close(q.z(), kilometers(7.0));
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_fewer_than_3_components() {
                let _: Length<Vec3> = serde_yaml::from_str("(5.0 4.0) km").unwrap();
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_more_than_3_components() {
                let _: Length<Vec3> = serde_yaml::from_str("(5.0 3.0 7.0 9.0) km").unwrap();
            }

            #[test]
            fn serialize_vector() {
                let x = meters * Vec3::new(5.3, 1.1, 2.2);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1 2.2) m\n");
            }

            #[test]
            fn serialize_dimensionless_vector() {
                let x = dimensionless * Vec3::new(5.3, 1.1, 2.2);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1 2.2)\n");
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32, crate::utils::assert_is_close_f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64, crate::utils::assert_is_close_f64);

#[cfg(all(feature = "f32", feature = "glam-vec2"))]
gen_tests_for_vector_2!(f32, vec2, glam::Vec2, crate::utils::assert_is_close_f32);

#[cfg(all(feature = "f64", feature = "glam-dvec2"))]
gen_tests_for_vector_2!(f64, dvec2, glam::DVec2, crate::utils::assert_is_close_f64);

#[cfg(all(feature = "f32", feature = "glam-vec3"))]
gen_tests_for_vector_3!(f32, vec3, glam::Vec3, crate::utils::assert_is_close_f32);

#[cfg(all(feature = "f64", feature = "glam-dvec3"))]
gen_tests_for_vector_3!(f64, dvec3, glam::DVec3, crate::utils::assert_is_close_f64);
