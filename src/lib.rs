#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]

mod floats;
mod quantity;
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
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            Vec2,
            f32,
            2
        );
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            Vec3,
            f32,
            3
        );
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            DVec2,
            f64,
            2
        );
        $crate::impl_vector_methods!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array,
            DVec3,
            f64,
            3
        );
        $crate::impl_vector2_methods!($quantity, $dimension, $dimensionless_const, Vec2, f32);
        $crate::impl_vector3_methods!($quantity, $dimension, $dimensionless_const, Vec3, f32);
        $crate::impl_vector2_methods!($quantity, $dimension, $dimensionless_const, DVec2, f64);
        $crate::impl_vector3_methods!($quantity, $dimension, $dimensionless_const, DVec3, f64);
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

#[cfg(test)]
mod tests {
    use crate::si::Dimension;
    use crate::si::Dimensionless;
    use crate::si::Length;
    use crate::si::Quantity;

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
    fn div_same_unit() {
        let x = Length::meters(1.0);
        let y = Length::meters(10.0);
        assert_is_close(x / y, Dimensionless::dimensionless(0.1));
    }
}
