#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]

mod floats;
mod helpers;
mod quantity;
mod traits;
mod unit_system;
mod vectors;

#[cfg(feature = "si")]
pub mod si;

#[cfg(feature = "hdf5")]
mod hdf5;

#[cfg(feature = "mpi")]
mod mpi;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rand")]
mod rand;

#[macro_export]
macro_rules! define_system {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $unit_names_array: ident) => {
        $crate::define_quantity!($quantity, $dimension, $dimensionless_const);
        $crate::impl_float_methods!($quantity, $dimension, $dimensionless_const);
        $crate::impl_concrete_float_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            f32
        );
        $crate::impl_concrete_float_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            f64
        );
        $crate::default_vector!();
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            glam::Vec2,
            f32,
            2
        );
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            glam::Vec3,
            f32,
            3
        );
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            glam::DVec2,
            f64,
            2
        );
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            glam::DVec3,
            f64,
            3
        );
        $crate::impl_vector2_methods!($quantity, $dimension, $dimensionless_const, glam::Vec2, f32);
        $crate::impl_vector3_methods!($quantity, $dimension, $dimensionless_const, glam::Vec3, f32);
        $crate::impl_vector2_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            glam::DVec2,
            f64
        );
        $crate::impl_vector3_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            glam::DVec3,
            f64
        );
        $crate::impl_hdf5_gated!($quantity, $dimension, $dimensionless_const);
        $crate::impl_mpi_gated!($quantity, $dimension);
        $crate::impl_rand_gated!($quantity, $dimension, $dimensionless_const);
        $crate::impl_serde_gated!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array
        );
    };
}

#[macro_export]
macro_rules! define_constant {
    ($quantity: ident, $float_type: ident, $dimensionless_const: ident, $constant_name: ident, $value_base: literal, $($dimension_ident: ident: $dimension_expr: literal),*) => {
        #[allow(clippy::needless_update)]
        pub const $constant_name: $quantity<$float_type, {Dimension {
            $(
                $dimension_ident: $dimension_expr,
            )*
                ..$dimensionless_const
        }}> =
            Quantity::new_unchecked($value_base);
    };
}

#[cfg(test)]
mod tests {
    use crate::si::dimension::NONE;
    use crate::si::Dimension;
    use crate::si::Dimensionless;
    use crate::si::Energy;
    use crate::si::Force;
    use crate::si::Length;
    use crate::si::Mass;
    use crate::si::Quantity;
    use crate::si::Time;
    use crate::si::Velocity;

    #[cfg(feature = "default-f32")]
    pub(crate) fn assert_is_close<const U: Dimension>(x: Quantity<f32, U>, y: Quantity<f32, U>) {
        assert!(
            (x - y).abs().value_unchecked() < f32::EPSILON,
            "{} {}",
            x.value_unchecked(),
            y.value_unchecked()
        )
    }

    #[cfg(not(feature = "default-f32"))]
    pub(crate) fn assert_is_close<const U: Dimension>(x: Quantity<f64, U>, y: Quantity<f64, U>) {
        assert!(
            (x - y).abs().value_unchecked() < f64::EPSILON,
            "{} {}",
            x.value_unchecked(),
            y.value_unchecked()
        )
    }

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
    fn sub_different_units() {
        let x = Length::meters(1.0);
        let y = Length::kilometers(10.0);
        assert_is_close(x - y, Length::meters(-9999.0));
    }

    #[test]
    fn mul_quantity_quantity() {
        let x = Force::newtons(2.0);
        let y = Length::meters(3.0);
        assert_is_close(x * y, Energy::joules(6.0));
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
    fn constant() {
        #[cfg(not(feature = "default-f32"))]
        define_constant!(Quantity, f64, NONE, CONSTANT, 5.0, length: 1, mass: 1);
        #[cfg(feature = "default-f32")]
        define_constant!(Quantity, f32, NONE, CONSTANT, 5.0, length: 1, mass: 1);
        assert_is_close(
            CONSTANT / Length::meters(5.0) / Mass::kilograms(1.0),
            Dimensionless::dimensionless(1.0),
        )
    }
}
