macro_rules! gen_tests_for_float {
    ($float_name: ident, $mod_name: ident, $assert_is_close: path, $assert_is_close_float: path) => {
        mod $mod_name {
            use crate::example_system::constants::{
                SOLAR_MASS, SOLAR_MASS_AWKWARD, SOLAR_MASS_GRAMS,
            };
            use crate::example_system::dimensions::{
                Dimensionless, Energy, Force, Length, Mass, Time, Velocity,
            };
            use crate::example_system::units;
            use crate::make_annotated_unit_constructor;
            use $assert_is_close as assert_is_close;
            use $assert_is_close_float as assert_is_close_float;

            make_annotated_unit_constructor!(meters, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(kilometers, Length<$float_name>, $float_name);
            make_annotated_unit_constructor!(seconds, Time<$float_name>, $float_name);
            make_annotated_unit_constructor!(kilograms, Mass<$float_name>, $float_name);
            make_annotated_unit_constructor!(
                dimensionless,
                Dimensionless<$float_name>,
                $float_name
            );
            make_annotated_unit_constructor!(meters_per_second, Velocity<$float_name>, $float_name);
            make_annotated_unit_constructor!(newtons, Force<$float_name>, $float_name);
            make_annotated_unit_constructor!(joules, Energy<$float_name>, $float_name);

            #[test]
            fn add_same_unit() {
                let x = meters(1.0);
                let y = meters(10.0);
                assert_is_close(x + y, meters(11.0));
            }

            #[test]
            fn add_different_units() {
                let x = meters(1.0);
                let y = kilometers(10.0);
                assert_is_close(x + y, meters(10001.0));
            }

            #[test]
            fn add_quantity_ref() {
                let x = meters(1.0);
                let y = meters(10.0);
                assert_is_close(x + &y, meters(11.0));
            }

            #[test]
            fn add_ref_quantity() {
                let x = meters(1.0);
                let y = meters(10.0);
                assert_is_close(&x + y, meters(11.0));
            }

            #[test]
            fn add_ref_ref() {
                let x = meters(1.0);
                let y = meters(10.0);
                assert_is_close(&x + &y, meters(11.0));
            }

            #[test]
            fn add_quantity_type() {
                let x = dimensionless(1.0);
                let y = 10.0;
                assert_is_close(x + y, dimensionless(11.0));
            }

            #[test]
            fn add_type_quantity() {
                let x = dimensionless(1.0);
                let y = 10.0;
                assert_is_close(y + x, dimensionless(11.0));
            }

            #[test]
            fn add_ref_type() {
                let x = &dimensionless(1.0);
                let y = 10.0;
                assert_is_close(x + y, dimensionless(11.0));
            }

            #[test]
            fn add_quantity_reftype() {
                let x = dimensionless(1.0);
                let y = &10.0;
                assert_is_close(x + y, dimensionless(11.0));
            }

            #[test]
            fn add_ref_reftype() {
                let x = &dimensionless(1.0);
                let y = &10.0;
                assert_is_close(x + y, dimensionless(11.0));
            }

            #[test]
            fn add_assign_quantity_quantity() {
                let mut x = meters(1.0);
                let y = kilometers(10.0);
                x += y;
                assert_is_close(x, meters(10001.0));
            }

            #[test]
            fn add_assign_quantity_ref() {
                let mut x = meters(1.0);
                let y = kilometers(10.0);
                x += &y;
                assert_is_close(x, meters(10001.0));
            }

            #[test]
            fn add_assign_ref_ref() {
                let mut x = &mut meters(1.0);
                let y = kilometers(10.0);
                x += &y;
                assert_is_close(*x, meters(10001.0));
            }

            #[test]
            fn add_assign_quantity_type() {
                let mut x = dimensionless(1.0);
                let y = 10.0;
                x += y;
                assert_is_close(x, dimensionless(11.0));
            }

            #[test]
            fn add_assign_type_quantity() {
                let x = dimensionless(1.0);
                let mut y = 10.0;
                y += x;
                assert_is_close_float(y, 11.0);
            }

            #[test]
            fn add_assign_ref_type() {
                let mut x = &mut dimensionless(1.0);
                let y = 10.0;
                x += y;
                assert_is_close(*x, dimensionless(11.0));
            }

            #[test]
            fn sum_quantity_type() {
                let items = [meters(3.0), kilometers(3.0), meters(9.0), kilometers(1.0)];
                assert_is_close(items.into_iter().sum(), meters(4012.0));
            }

            #[test]
            fn sub_different_units() {
                let x = meters(1.0);
                let y = kilometers(10.0);
                assert_is_close(x - y, meters(-9999.0));
            }

            #[test]
            fn sub_quantity_ref() {
                let x = meters(1.0);
                let y = meters(10.0);
                assert_is_close(x - &y, meters(-9.0));
            }

            #[test]
            fn sub_ref_ref() {
                let x = meters(1.0);
                let y = meters(10.0);
                assert_is_close(&x - &y, meters(-9.0));
            }

            #[test]
            fn sub_quantity_type() {
                let x = dimensionless(1.0);
                let y = 10.0;
                assert_is_close(x - y, dimensionless(-9.0));
            }

            #[test]
            fn sub_type_quantity() {
                let x = dimensionless(1.0);
                let y = 10.0;
                assert_is_close(y - x, dimensionless(9.0));
            }

            #[test]
            fn sub_ref_type() {
                let x = &dimensionless(1.0);
                let y = 10.0;
                assert_is_close(x - y, dimensionless(-9.0));
            }

            #[test]
            fn sub_quantity_reftype() {
                let x = dimensionless(1.0);
                let y = &10.0;
                assert_is_close(x - y, dimensionless(-9.0));
            }

            #[test]
            fn sub_ref_reftype() {
                let x = &dimensionless(1.0);
                let y = &10.0;
                assert_is_close(x - y, dimensionless(-9.0));
            }

            #[test]
            fn sub_assign_quantity_quantity() {
                let mut x = meters(1.0);
                let y = kilometers(10.0);
                x -= y;
                assert_is_close(x, meters(-9999.0));
            }

            #[test]
            fn sub_assign_quantity_ref() {
                let mut x = meters(1.0);
                let y = kilometers(10.0);
                x -= &y;
                assert_is_close(x, meters(-9999.0));
            }

            #[test]
            fn sub_assign_ref_quantity() {
                let mut x = &mut meters(1.0);
                let y = kilometers(10.0);
                x -= y;
                assert_is_close(*x, meters(-9999.0));
            }

            #[test]
            fn sub_assign_ref_ref() {
                let mut x = &mut meters(1.0);
                let y = kilometers(10.0);
                x -= &y;
                assert_is_close(*x, meters(-9999.0));
            }

            #[test]
            fn sub_assign_quantity_type() {
                let mut x = dimensionless(1.0);
                let y = 10.0;
                x -= y;
                assert_is_close(x, dimensionless(-9.0));
            }

            #[test]
            fn sub_assign_type_quantity() {
                let x = dimensionless(1.0);
                let mut y = 10.0;
                y -= x;
                assert_is_close_float(y, 9.0);
            }

            #[test]
            fn sub_assign_ref_type() {
                let mut x = &mut dimensionless(1.0);
                let y = 10.0;
                x -= y;
                assert_is_close(*x, dimensionless(-9.0));
            }

            #[test]
            fn neg_quantity() {
                let x = meters(5.0);
                let y = meters(2.0);
                assert_is_close(x + (-y), meters(3.0));
                assert_is_close(x - y, meters(3.0));
            }

            #[test]
            fn mul_quantity_quantity() {
                let x = newtons(2.0);
                let y = meters(3.0);
                assert_is_close(x * y, joules(6.0));
            }

            #[test]
            fn mul_quantity_ref() {
                let x = newtons(2.0);
                let y = meters(3.0);
                assert_is_close(x * &y, joules(6.0));
            }

            #[test]
            fn mul_ref_quantity() {
                let x = newtons(2.0);
                let y = meters(3.0);
                assert_is_close(&x * y, joules(6.0));
            }

            #[test]
            fn mul_ref_ref() {
                let x = newtons(2.0);
                let y = meters(3.0);
                assert_is_close(&x * &y, joules(6.0));
            }

            #[test]
            fn mul_quantity_type() {
                let x = newtons(2.0);
                let y = 3.0;
                assert_is_close(x * y, newtons(6.0));
            }

            #[test]
            fn mul_type_quantity() {
                let x = 3.0;
                let y = newtons(2.0);
                assert_is_close(x * y, newtons(6.0));
            }

            #[test]
            fn mul_quantity_typeref() {
                let x = newtons(2.0);
                let y = &3.0;
                assert_is_close(x * y, newtons(6.0));
            }

            #[test]
            fn mul_ref_type() {
                let x = &newtons(2.0);
                let y = 3.0;
                assert_is_close(x * y, newtons(6.0));
            }

            #[test]
            fn mul_ref_typeref() {
                let x = &newtons(2.0);
                let y = &3.0;
                assert_is_close(x * y, newtons(6.0));
            }

            #[test]
            fn mul_assign_quantity_quantity() {
                let mut x = newtons(2.0);
                let y = dimensionless(3.0);
                x *= y;
                assert_is_close(x, newtons(6.0));
            }

            #[test]
            fn mul_assign_quantity_ref() {
                let mut x = newtons(2.0);
                let y = dimensionless(3.0);
                x *= &y;
                assert_is_close(x, newtons(6.0));
            }

            #[test]
            fn mul_assign_ref_quantity() {
                let mut x = &mut newtons(2.0);
                let y = dimensionless(3.0);
                x *= y;
                assert_is_close(*x, newtons(6.0));
            }

            #[test]
            fn mul_assign_ref_ref() {
                let mut x = &mut newtons(2.0);
                let y = dimensionless(3.0);
                x *= &y;
                assert_is_close(*x, newtons(6.0));
            }

            #[test]
            fn mul_assign_quantity_type() {
                let mut x = newtons(2.0);
                let y = 3.0;
                x *= y;
                assert_is_close(x, newtons(6.0));
            }

            #[test]
            fn mul_assign_type_quantity() {
                let mut x = 3.0;
                let y = dimensionless(2.0);
                x *= y;
                assert_is_close_float(x, 6.0);
            }

            #[test]
            fn mul_assign_type_ref() {
                let mut x = 3.0;
                let y = &dimensionless(2.0);
                x *= y;
                assert_is_close_float(x, 6.0);
            }

            #[test]
            fn div_quantity_quantity() {
                let x = meters(6.0);
                let y = seconds(2.0);
                assert_is_close(x / y, meters_per_second(3.0));
            }

            #[test]
            fn div_quantity_ref() {
                let x = meters(6.0);
                let y = seconds(2.0);
                assert_is_close(x / &y, meters_per_second(3.0));
            }

            #[test]
            fn div_ref_quantity() {
                let x = meters(6.0);
                let y = seconds(2.0);
                assert_is_close(&x / y, meters_per_second(3.0));
            }

            #[test]
            fn div_ref_ref() {
                let x = meters(6.0);
                let y = seconds(2.0);
                assert_is_close(&x / &y, meters_per_second(3.0));
            }

            #[test]
            fn div_assign_quantity_quantity() {
                let mut x = newtons(2.0);
                let y = dimensionless(4.0);
                x /= y;
                assert_is_close(x, newtons(0.5));
            }

            #[test]
            fn div_assign_quantity_ref() {
                let mut x = newtons(2.0);
                let y = dimensionless(4.0);
                x /= &y;
                assert_is_close(x, newtons(0.5));
            }

            #[test]
            fn div_assign_ref_quantity() {
                let mut x = &mut newtons(2.0);
                let y = dimensionless(4.0);
                x /= y;
                assert_is_close(*x, newtons(0.5));
            }

            #[test]
            fn div_assign_ref_ref() {
                let mut x = &mut newtons(2.0);
                let y = dimensionless(4.0);
                x /= &y;
                assert_is_close(*x, newtons(0.5));
            }

            #[test]
            fn div_quantity_type() {
                let x = meters(6.0);
                let y = 2.0;
                assert_is_close(x / y, meters(3.0));
            }

            #[test]
            fn div_quantity_reftype() {
                let x = meters(6.0);
                let y = &2.0;
                assert_is_close(x / y, meters(3.0));
            }

            #[test]
            fn div_ref_type() {
                let x = &meters(6.0);
                let y = 2.0;
                assert_is_close(x / y, meters(3.0));
            }

            #[test]
            fn div_ref_reftype() {
                let x = &meters(6.0);
                let y = &2.0;
                assert_is_close(x / y, meters(3.0));
            }

            #[test]
            fn div_assign_quantity_type() {
                let mut x = meters(6.0);
                let y = 2.0;
                x /= y;
                assert_is_close(x, meters(3.0));
            }

            #[test]
            fn div_type_quantity() {
                let x = 2.0;
                let y = meters_per_second(6.0);
                assert_is_close(x / y, seconds(2.0) / meters(6.0));
            }

            #[test]
            fn div_assign_type_quantity() {
                let mut x = 6.0;
                let y = dimensionless(2.0);
                x /= y;
                assert_is_close_float(x, 3.0);
            }

            #[cfg(any(feature = "std", feature = "num-traits-libm"))]
            #[test]
            fn sqrt_float_quantity() {
                let x = meters(6.0).powi::<2>();
                let y = seconds(2.0).powi::<2>();
                assert_is_close((x / y).sqrt(), meters_per_second(3.0));
            }

            #[cfg(any(feature = "std", feature = "num-traits-libm"))]
            #[test]
            fn cbrt_float_quantity() {
                let x = meters(4.0).powi::<3>();
                let y = seconds(1.0).powi::<3>();
                assert_is_close((x / y).cbrt(), meters_per_second(4.0));
            }

            #[test]
            fn constant() {
                assert_is_close((1.0 as $float_name) * SOLAR_MASS, kilograms(1.988477e30));
                assert_is_close(
                    (1.0 as $float_name) * SOLAR_MASS_GRAMS,
                    kilograms(1.988477e30),
                );
                assert_is_close(
                    (1.0 as $float_name) * SOLAR_MASS_AWKWARD,
                    kilograms(1.988477e30),
                );
            }

            #[cfg(any(feature = "std", feature = "num-traits-libm"))]
            #[test]
            fn log2() {
                let x = dimensionless(128.0);
                assert_is_close(x.log2(), dimensionless(7.0));
            }

            #[test]
            fn deref_dimensionless() {
                let x = dimensionless(128.3);
                assert_eq!(x.round(), 128.0);
            }

            #[test]
            fn partial_eq_quantity_quantity() {
                let x = meters(50.0);
                let y = meters(50.0);
                assert!(x == y);
            }

            #[test]
            fn partial_eq_quantity_ref() {
                let x = meters(50.0);
                let y = meters(50.0);
                assert!(x == &y);
            }

            #[test]
            fn partial_eq_ref_quantity() {
                let x = meters(50.0);
                let y = meters(50.0);
                assert!(&x == y);
            }

            #[test]
            fn partial_eq_ref_ref() {
                let x = meters(50.0);
                let y = meters(50.0);
                assert!(&x == &y);
            }

            #[test]
            fn partial_eq_quantity_type() {
                let x = dimensionless(50.0);
                let y = 50.0;
                assert!(x == y);
            }

            #[test]
            fn partial_eq_type_quantity() {
                let x = 50.0;
                let y = dimensionless(50.0);
                assert!(x == y);
            }

            #[test]
            fn partial_ord_quantity_quantity() {
                let x = meters(50.0);
                let y = meters(49.0);
                assert!(x > y);
            }

            #[test]
            fn partial_ord_quantity_type() {
                let x = dimensionless(50.0);
                let y = 49.0;
                assert!(x >= y);
            }

            #[test]
            fn partial_ord_type_quantity() {
                let x = 50.0;
                let y = dimensionless(49.0);
                assert!(x > y);
            }

            #[test]
            fn clamp_quantity_quantity() {
                let x = meters(10.0);
                let y = kilometers(20.0);
                assert!(meters(1.0).clamp(x, y) == x);
                assert!(meters(50.0).clamp(x, y) == meters(50.0));
                assert!(meters(50000.0).clamp(x, y) == y);
            }

            #[test]
            fn clamp_quantity_type() {
                let x = 10.0;
                let y = 20.0;
                assert!(dimensionless(5.0).clamp(x, y) == x);
                assert!(dimensionless(15.0).clamp(x, y) == dimensionless(15.0));
                assert!(dimensionless(50.0).clamp(x, y) == y);
            }

            #[test]
            fn max_quantity_quantity() {
                let x = meters(10.0);
                assert!(meters(5.0).max(x) == x);
                assert!(meters(15.0).max(x) == meters(15.0));
            }

            #[test]
            fn max_quantity_type() {
                let x = 10.0;
                assert!(dimensionless(5.0).max(x) == x);
                assert!(dimensionless(15.0).max(x) == dimensionless(15.0));
            }

            #[test]
            fn min_quantity_quantity() {
                let x = meters(10.0);
                assert!(meters(5.0).min(x) == meters(5.0));
                assert!(meters(15.0).min(x) == x);
            }

            #[test]
            fn min_quantity_type() {
                let x = 10.0;
                assert!(dimensionless(5.0).min(x) == dimensionless(5.0));
                assert!(dimensionless(15.0).min(x) == x);
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(
    f32,
    f32,
    crate::utils::assert_is_close_f32,
    crate::utils::assert_is_close_float_f32
);

#[cfg(feature = "f64")]
gen_tests_for_float!(
    f64,
    f64,
    crate::utils::assert_is_close_f64,
    crate::utils::assert_is_close_float_f64
);
