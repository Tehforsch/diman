#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "default-f32", feature = "default-f64"))]
compile_error!("Both 'default-f32' and 'default-f64' are activated. This is impossible.");

#[cfg(all(feature = "default-2d", feature = "default-3d"))]
compile_error!("Both 'default-2d' and 'default-3d' are activated. This is impossible.");

mod debug_storage_type;
mod quantity;
mod type_aliases;
mod vectors;

#[cfg(test)]
pub mod test_system;

#[cfg(feature = "mpi")]
mod mpi;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rand")]
mod rand;

#[cfg(test)]
mod test_utils;

pub use debug_storage_type::DebugStorageType;
pub use derive_dimension::diman_dimension;
pub use type_aliases::Product;
pub use type_aliases::QProduct;
pub use type_aliases::Quotient;
pub use unit_system::unit_system;

#[cfg(test)]
#[cfg(any(feature = "f64"))]
mod tests {
    use crate::test_system::f64::Dimensionless;
    use crate::test_system::f64::Energy;
    use crate::test_system::f64::Force;
    use crate::test_system::f64::Length;
    use crate::test_system::f64::Mass;
    use crate::test_system::f64::Time;
    use crate::test_system::f64::Velocity;
    use crate::test_system::f64::SOLAR_MASS;
    use crate::test_system::f64::SOLAR_MASS_AWKWARD;
    use crate::test_system::f64::SOLAR_MASS_GRAMS;
    use crate::test_utils::assert_is_close_f64 as assert_is_close;
    use crate::test_utils::assert_is_close_float;

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
    fn add_assign_quantity_quantity() {
        let mut x = Length::meters(1.0);
        let y = Length::kilometers(10.0);
        x += y;
        assert_is_close(x, Length::meters(10001.0));
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
    fn sub_assign_quantity_quantity() {
        let mut x = Length::meters(1.0);
        let y = Length::kilometers(10.0);
        x -= y;
        assert_is_close(x, Length::meters(-9999.0));
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
    fn mul_quantity_quantity() {
        let x = Force::newtons(2.0);
        let y = Length::meters(3.0);
        assert_is_close(x * y, Energy::joules(6.0));
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
    fn mul_float_quantity() {
        let x = 3.0;
        let y = Force::newtons(2.0);
        assert_is_close(x * y, Force::newtons(6.0));
    }

    #[test]
    fn div_quantity_quantity() {
        let x = Length::meters(6.0);
        let y = Time::seconds(2.0);
        assert_is_close(x / y, Velocity::meters_per_second(3.0));
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
    fn div_float_quantity() {
        let x = 2.0;
        let y = Velocity::meters_per_second(6.0);
        assert_is_close(x / y, Time::seconds(2.0) / Length::meters(6.0));
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
}

pub mod temporary_test_system;
