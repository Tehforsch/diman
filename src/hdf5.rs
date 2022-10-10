#[macro_export]
macro_rules! impl_hdf5_float {
    ($quantity: ident, $dimension: ident, $float_type: ident, $float_size_type: ident) => {
        unsafe impl<const D: $dimension> H5Type for $quantity<$float_type, D> {
            fn type_descriptor() -> hdf5::types::TypeDescriptor {
                TypeDescriptor::Float($float_size_type)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_hdf5_vector {
    ($quantity: ident, $dimension: ident, $vector_type: ident, $float_size_type: ident, $num_dims: literal) => {
        unsafe impl<const D: $dimension> H5Type for $quantity<$vector_type, D> {
            fn type_descriptor() -> hdf5::types::TypeDescriptor {
                TypeDescriptor::FixedArray(
                    Box::new(TypeDescriptor::Float($float_size_type)),
                    $num_dims,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! impl_hdf5 {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        use hdf5::types::FloatSize::U4;
        use hdf5::types::FloatSize::U8;
        use hdf5::types::TypeDescriptor;
        use hdf5::H5Type;

        $crate::impl_hdf5_float!($quantity, $dimension, f32, U4);
        $crate::impl_hdf5_float!($quantity, $dimension, f64, U8);

        $crate::impl_hdf5_vector!($quantity, $dimension, Vec2, U4, 2);
        $crate::impl_hdf5_vector!($quantity, $dimension, DVec2, U8, 2);
        $crate::impl_hdf5_vector!($quantity, $dimension, Vec3, U4, 3);
        $crate::impl_hdf5_vector!($quantity, $dimension, DVec3, U8, 3);
    };
}
