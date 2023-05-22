#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "default-f32", feature = "default-f64"))]
compile_error!("Both 'default-f32' and 'default-f64' are activated. This is impossible.");

#[cfg(all(feature = "default-2d", feature = "default-3d"))]
compile_error!("Both 'default-2d' and 'default-3d' are activated. This is impossible.");

mod floats;
mod helpers;
mod quantity;
mod traits;
mod type_aliases;
mod unit_system;
mod vectors;

#[cfg(test)]
mod test_system;

#[cfg(feature = "hdf5")]
mod hdf5;

#[cfg(feature = "mpi")]
mod mpi;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rand")]
mod rand;

#[cfg(test)]
mod test_utils;

pub use derive_dimension::diman_dimension;
pub use type_aliases::Product;
pub use type_aliases::QProduct;
pub use type_aliases::Quotient;

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

        $crate::impl_glam!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array
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
#[cfg(not(feature = "glam"))]
macro_rules! impl_glam {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $unit_names_array: ident) => {};
}

#[macro_export]
#[cfg(feature = "glam")]
macro_rules! impl_glam {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $unit_names_array: ident) => {
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
    };
}

#[macro_export]
macro_rules! define_constant {
    ($quantity: ident, $float_type: ident, $dimensionless_const: ident, $constant_name: ident, $value_base: expr, $($dimension_ident: ident: $dimension_expr: literal),*) => {
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
#[cfg(any(feature = "f64"))]
mod tests {
    use crate::test_system::f64::Dimensionless;
    use crate::test_system::f64::Energy;
    use crate::test_system::f64::Force;
    use crate::test_system::f64::Length;
    use crate::test_system::f64::Mass;
    use crate::test_system::f64::Time;
    use crate::test_system::f64::Velocity;
    use crate::test_system::Dimension;
    use crate::test_system::Quantity;
    use crate::test_utils::assert_is_close;

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

    // #[test]
    // fn mul_quantity_float() {
    //     let x = Force::newtons(2.0);
    //     let y = 3.0;
    //     assert_is_close(x * y, Force::newtons(6.0));
    // }

    // #[test]
    // fn mul_float_quantity() {
    //     let x = 3.0;
    //     let y = Force::newtons(2.0);
    //     assert_is_close(x * y, Force::newtons(6.0));
    // }

    #[test]
    fn div_quantity_quantity() {
        let x = Length::meters(6.0);
        let y = Time::seconds(2.0);
        assert_is_close(x / y, Velocity::meters_per_second(3.0));
    }

    // #[test]
    // fn div_quantity_float() {
    //     let x = Length::meters(6.0);
    //     let y = 2.0;
    //     assert_is_close(x / y, Length::meters(3.0));
    // }

    // #[test]
    // fn div_float_quantity() {
    //     let x = 2.0;
    //     let y = Velocity::meters_per_second(6.0);
    //     assert_is_close(x / y, Time::seconds(2.0) / Length::meters(6.0));
    // }

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

    // #[test]
    // fn constant() {
    //     #[cfg(feature = "default-f32")]
    //     define_constant!(Quantity, f32, NONE, CONSTANT, 5.0, length: 1, mass: 1);
    //     #[cfg(feature = "default-f64")]
    //     define_constant!(Quantity, f64, NONE, CONSTANT, 5.0, length: 1, mass: 1);
    //     assert_is_close(
    //         CONSTANT / Length::meters(5.0) / Mass::kilograms(1.0),
    //         Dimensionless::dimensionless(1.0),
    //     )
    // }

    #[test]
    fn log2() {
        let x = Dimensionless::dimensionless(128.0);
        assert_is_close(x.log2(), Dimensionless::dimensionless(7.0));
    }
}
