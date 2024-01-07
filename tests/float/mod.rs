macro_rules! gen_tests_for_float {
    ($float_name: ident, $mod_name: ident, $assert_is_close: path, $assert_is_close_float: path) => {
        mod $mod_name {
            use crate::example_system::$float_name::Dimensionless;
            use crate::example_system::$float_name::Energy;
            use crate::example_system::$float_name::Force;
            use crate::example_system::$float_name::Length;
            use crate::example_system::$float_name::Mass;
            use crate::example_system::$float_name::Time;
            use crate::example_system::$float_name::Velocity;
            use crate::example_system::$float_name::SOLAR_MASS;
            use crate::example_system::$float_name::SOLAR_MASS_AWKWARD;
            use crate::example_system::$float_name::SOLAR_MASS_GRAMS;
            use $assert_is_close as assert_is_close;
            use $assert_is_close_float as assert_is_close_float;

            #[test]
            fn add_same_unit() {
                let x = Length::meters(1.0);
                let y = Length::meters(10.0);
                assert_is_close(x + y, Length::meters(11.0));
            }

            #[test]
            fn add_different_units() {
                let x = Length::meters(1.0);
                let y = Length::kilometers(10.0);
                assert_is_close(x + y, Length::meters(10001.0));
            }

            #[test]
            fn add_quantity_ref() {
                let x = Length::meters(1.0);
                let y = Length::meters(10.0);
                assert_is_close(x + &y, Length::meters(11.0));
            }

            #[test]
            fn add_ref_quantity() {
                let x = Length::meters(1.0);
                let y = Length::meters(10.0);
                assert_is_close(&x + y, Length::meters(11.0));
            }

            #[test]
            fn add_ref_ref() {
                let x = Length::meters(1.0);
                let y = Length::meters(10.0);
                assert_is_close(&x + &y, Length::meters(11.0));
            }

            #[test]
            fn add_assign_quantity_quantity() {
                let mut x = Length::meters(1.0);
                let y = Length::kilometers(10.0);
                x += y;
                assert_is_close(x, Length::meters(10001.0));
            }

            #[test]
            fn add_assign_quantity_ref() {
                let mut x = Length::meters(1.0);
                let y = Length::kilometers(10.0);
                x += &y;
                assert_is_close(x, Length::meters(10001.0));
            }

            #[test]
            fn add_assign_ref_ref() {
                let mut x = &mut Length::meters(1.0);
                let y = Length::kilometers(10.0);
                x += &y;
                assert_is_close(*x, Length::meters(10001.0));
            }

            #[test]
            fn add_assign_quantity_type() {
                let mut x = Dimensionless::dimensionless(1.0);
                let y = 10.0;
                x += y;
                assert_is_close(x, Dimensionless::dimensionless(11.0));
            }

            #[test]
            fn add_assign_type_quantity() {
                let x = Dimensionless::dimensionless(1.0);
                let mut y = 10.0;
                y += x;
                assert_is_close_float(y, 11.0);
            }

            #[test]
            fn add_quantity_type() {
                let x = Dimensionless::dimensionless(1.0);
                let y = 10.0;
                assert_is_close(x + y, Dimensionless::dimensionless(11.0));
            }

            #[test]
            fn add_type_quantity() {
                let x = Dimensionless::dimensionless(1.0);
                let y = 10.0;
                assert_is_close(y + x, Dimensionless::dimensionless(11.0));
            }

            #[test]
            fn sum_quantity_type() {
                let items = [
                    Length::meters(3.0),
                    Length::kilometers(3.0),
                    Length::meters(9.0),
                    Length::kilometers(1.0),
                ];
                assert_is_close(items.into_iter().sum(), Length::meters(4012.0));
            }

            #[test]
            fn sub_different_units() {
                let x = Length::meters(1.0);
                let y = Length::kilometers(10.0);
                assert_is_close(x - y, Length::meters(-9999.0));
            }

            #[test]
            fn sub_quantity_ref() {
                let x = Length::meters(1.0);
                let y = Length::meters(10.0);
                assert_is_close(x - &y, Length::meters(-9.0));
            }

            #[test]
            fn sub_ref_ref() {
                let x = Length::meters(1.0);
                let y = Length::meters(10.0);
                assert_is_close(&x - &y, Length::meters(-9.0));
            }

            #[test]
            fn sub_assign_quantity_quantity() {
                let mut x = Length::meters(1.0);
                let y = Length::kilometers(10.0);
                x -= y;
                assert_is_close(x, Length::meters(-9999.0));
            }

            #[test]
            fn sub_assign_quantity_ref() {
                let mut x = Length::meters(1.0);
                let y = Length::kilometers(10.0);
                x -= &y;
                assert_is_close(x, Length::meters(-9999.0));
            }

            #[test]
            fn sub_assign_ref_quantity() {
                let mut x = &mut Length::meters(1.0);
                let y = Length::kilometers(10.0);
                x -= y;
                assert_is_close(*x, Length::meters(-9999.0));
            }

            #[test]
            fn sub_assign_ref_ref() {
                let mut x = &mut Length::meters(1.0);
                let y = Length::kilometers(10.0);
                x -= &y;
                assert_is_close(*x, Length::meters(-9999.0));
            }

            #[test]
            fn sub_assign_quantity_type() {
                let mut x = Dimensionless::dimensionless(1.0);
                let y = 10.0;
                x -= y;
                assert_is_close(x, Dimensionless::dimensionless(-9.0));
            }

            #[test]
            fn sub_assign_type_quantity() {
                let x = Dimensionless::dimensionless(1.0);
                let mut y = 10.0;
                y -= x;
                assert_is_close_float(y, 9.0);
            }

            #[test]
            fn sub_quantity_type() {
                let x = Dimensionless::dimensionless(1.0);
                let y = 10.0;
                assert_is_close(x - y, Dimensionless::dimensionless(-9.0));
            }

            #[test]
            fn sub_type_quantity() {
                let x = Dimensionless::dimensionless(1.0);
                let y = 10.0;
                assert_is_close(y - x, Dimensionless::dimensionless(9.0));
            }

            #[test]
            fn neg_quantity() {
                let x = Length::meters(5.0);
                let y = Length::meters(2.0);
                assert_is_close(x + (-y), Length::meters(3.0));
                assert_is_close(x - y, Length::meters(3.0));
            }

            #[test]
            fn mul_quantity_quantity() {
                let x = Force::newtons(2.0);
                let y = Length::meters(3.0);
                assert_is_close(x * y, Energy::joules(6.0));
            }

            #[test]
            fn mul_quantity_ref() {
                let x = Force::newtons(2.0);
                let y = Length::meters(3.0);
                assert_is_close(x * &y, Energy::joules(6.0));
            }

            #[test]
            fn mul_ref_quantity() {
                let x = Force::newtons(2.0);
                let y = Length::meters(3.0);
                assert_is_close(&x * y, Energy::joules(6.0));
            }

            #[test]
            fn mul_ref_ref() {
                let x = Force::newtons(2.0);
                let y = Length::meters(3.0);
                assert_is_close(&x * &y, Energy::joules(6.0));
            }

            #[test]
            fn mul_assign_quantity_quantity() {
                let mut x = Force::newtons(2.0);
                let y = Dimensionless::dimensionless(3.0);
                x *= y;
                assert_is_close(x, Force::newtons(6.0));
            }

            #[test]
            fn mul_quantity_float() {
                let x = Force::newtons(2.0);
                let y = 3.0;
                assert_is_close(x * y, Force::newtons(6.0));
            }

            #[test]
            fn mul_assign_quantity_float() {
                let mut x = Force::newtons(2.0);
                let y = 3.0;
                x *= y;
                assert_is_close(x, Force::newtons(6.0));
            }

            #[test]
            fn mul_float_quantity() {
                let x = 3.0;
                let y = Force::newtons(2.0);
                assert_is_close(x * y, Force::newtons(6.0));
            }

            #[test]
            fn mul_assign_float_quantity() {
                let mut x = 3.0;
                let y = Dimensionless::dimensionless(2.0);
                x *= y;
                assert_is_close_float(x, 6.0);
            }

            #[test]
            fn div_quantity_quantity() {
                let x = Length::meters(6.0);
                let y = Time::seconds(2.0);
                assert_is_close(x / y, Velocity::meters_per_second(3.0));
            }

            #[test]
            fn div_quantity_ref() {
                let x = Length::meters(6.0);
                let y = Time::seconds(2.0);
                assert_is_close(x / &y, Velocity::meters_per_second(3.0));
            }

            #[test]
            fn div_ref_quantity() {
                let x = Length::meters(6.0);
                let y = Time::seconds(2.0);
                assert_is_close(&x / y, Velocity::meters_per_second(3.0));
            }

            #[test]
            fn div_ref_ref() {
                let x = Length::meters(6.0);
                let y = Time::seconds(2.0);
                assert_is_close(&x / &y, Velocity::meters_per_second(3.0));
            }

            #[test]
            fn div_assign_quantity_quantity() {
                let mut x = Force::newtons(2.0);
                let y = Dimensionless::dimensionless(4.0);
                x /= y;
                assert_is_close(x, Force::newtons(0.5));
            }

            #[test]
            fn div_quantity_float() {
                let x = Length::meters(6.0);
                let y = 2.0;
                assert_is_close(x / y, Length::meters(3.0));
            }

            #[test]
            fn div_assign_quantity_float() {
                let mut x = Length::meters(6.0);
                let y = 2.0;
                x /= y;
                assert_is_close(x, Length::meters(3.0));
            }

            #[test]
            fn div_float_quantity() {
                let x = 2.0;
                let y = Velocity::meters_per_second(6.0);
                assert_is_close(x / y, Time::seconds(2.0) / Length::meters(6.0));
            }

            #[test]
            fn div_assign_float_quantity() {
                let mut x = 6.0;
                let y = Dimensionless::dimensionless(2.0);
                x /= y;
                assert_is_close_float(x, 3.0);
            }

            #[test]
            fn sqrt_float_quantity() {
                let x = Length::meters(6.0).powi::<2>();
                let y = Time::seconds(2.0).powi::<2>();
                assert_is_close((x / y).sqrt(), Velocity::meters_per_second(3.0));
            }

            #[test]
            fn cbrt_float_quantity() {
                let x = Length::meters(4.0).powi::<3>();
                let y = Time::seconds(1.0).powi::<3>();
                assert_is_close((x / y).cbrt(), Velocity::meters_per_second(4.0));
            }

            #[test]
            fn constant() {
                assert_is_close(SOLAR_MASS, Mass::kilograms(1.988477e30));
                assert_is_close(SOLAR_MASS_GRAMS, Mass::kilograms(1.988477e30));
                assert_is_close(SOLAR_MASS_AWKWARD, Mass::kilograms(1.988477e30));
            }

            #[test]
            fn log2() {
                let x = Dimensionless::dimensionless(128.0);
                assert_is_close(x.log2(), Dimensionless::dimensionless(7.0));
            }

            #[test]
            fn deref_dimensionless() {
                let x = Dimensionless::dimensionless(128.3);
                assert_eq!(x.round(), 128.0);
            }

            #[test]
            fn partial_eq_quantity_quantity() {
                let x = Length::meters(50.0);
                let y = Length::meters(50.0);
                assert!(x == y);
            }

            #[test]
            fn partial_eq_quantity_type() {
                let x = Dimensionless::dimensionless(50.0);
                let y = 50.0;
                assert!(x == y);
            }

            #[test]
            fn partial_eq_type_quantity() {
                let x = 50.0;
                let y = Dimensionless::dimensionless(50.0);
                assert!(x == y);
            }

            #[test]
            fn partial_ord_quantity_quantity() {
                let x = Length::meters(50.0);
                let y = Length::meters(49.0);
                assert!(x > y);
            }

            #[test]
            fn partial_ord_quantity_type() {
                let x = Dimensionless::dimensionless(50.0);
                let y = 49.0;
                assert!(x >= y);
            }

            #[test]
            fn partial_ord_type_quantity() {
                let x = 50.0;
                let y = Dimensionless::dimensionless(49.0);
                assert!(x > y);
            }

            #[test]
            fn clamp_quantity_quantity() {
                let x = Length::meters(10.0);
                let y = Length::kilometers(20.0);
                assert!(Length::meters(1.0).clamp(x, y) == x);
                assert!(Length::meters(50.0).clamp(x, y) == Length::meters(50.0));
                assert!(Length::meters(50000.0).clamp(x, y) == y);
            }

            #[test]
            fn clamp_quantity_type() {
                let x = 10.0;
                let y = 20.0;
                assert!(Dimensionless::dimensionless(5.0).clamp(x, y) == x);
                assert!(
                    Dimensionless::dimensionless(15.0).clamp(x, y)
                        == Dimensionless::dimensionless(15.0)
                );
                assert!(Dimensionless::dimensionless(50.0).clamp(x, y) == y);
            }

            #[test]
            fn max_quantity_quantity() {
                let x = Length::meters(10.0);
                assert!(Length::meters(5.0).max(x) == x);
                assert!(Length::meters(15.0).max(x) == Length::meters(15.0));
            }

            #[test]
            fn max_quantity_type() {
                let x = 10.0;
                assert!(Dimensionless::dimensionless(5.0).max(x) == x);
                assert!(
                    Dimensionless::dimensionless(15.0).max(x) == Dimensionless::dimensionless(15.0)
                );
            }

            #[test]
            fn min_quantity_quantity() {
                let x = Length::meters(10.0);
                assert!(Length::meters(5.0).min(x) == Length::meters(5.0));
                assert!(Length::meters(15.0).min(x) == x);
            }

            #[test]
            fn min_quantity_type() {
                let x = 10.0;
                assert!(
                    Dimensionless::dimensionless(5.0).min(x) == Dimensionless::dimensionless(5.0)
                );
                assert!(Dimensionless::dimensionless(15.0).min(x) == x);
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
