#[macro_export]
macro_rules! impl_hdf5_float {
    ($quantity: ident, $dimension: ident, $float_type: ty, $float_size_type: expr) => {
        unsafe impl<const D: $dimension> hdf5::H5Type for $quantity<$float_type, D> {
            fn type_descriptor() -> hdf5::types::TypeDescriptor {
                hdf5::types::TypeDescriptor::Float($float_size_type)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_hdf5_vector {
    ($quantity: ident, $dimension: ident, $vector_type: ty, $float_size_type: expr, $num_dims: literal) => {
        unsafe impl<const D: $dimension> hdf5::H5Type for $quantity<$vector_type, D> {
            fn type_descriptor() -> hdf5::types::TypeDescriptor {
                hdf5::types::TypeDescriptor::FixedArray(
                    Box::new(hdf5::types::TypeDescriptor::Float($float_size_type)),
                    $num_dims,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! impl_hdf5_glam {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        $crate::impl_hdf5_vector!(
            $quantity,
            $dimension,
            glam::Vec2,
            hdf5::types::FloatSize::U4,
            2
        );
        $crate::impl_hdf5_vector!(
            $quantity,
            $dimension,
            glam::DVec2,
            hdf5::types::FloatSize::U8,
            2
        );
        $crate::impl_hdf5_vector!(
            $quantity,
            $dimension,
            glam::Vec3,
            hdf5::types::FloatSize::U4,
            3
        );
        $crate::impl_hdf5_vector!(
            $quantity,
            $dimension,
            glam::DVec3,
            hdf5::types::FloatSize::U8,
            3
        );
    };
}

#[macro_export]
macro_rules! impl_hdf5 {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        $crate::impl_hdf5_float!($quantity, $dimension, f32, hdf5::types::FloatSize::U4);
        $crate::impl_hdf5_float!($quantity, $dimension, f64, hdf5::types::FloatSize::U8);

        #[cfg(feature = "glam")]
        $crate::impl_hdf5_glam!($quantity, $dimension, $dimensionless_const);
    };
}
