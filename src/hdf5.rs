#[macro_export]
macro_rules! impl_hdf5 {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        use hdf5::types::FloatSize;
        use hdf5::types::TypeDescriptor;
        use hdf5::H5Type;

        unsafe impl<const D: $dimension> H5Type for $quantity<f64, D> {
            fn type_descriptor() -> hdf5::types::TypeDescriptor {
                TypeDescriptor::Float(FloatSize::U8)
            }
        }

        unsafe impl<const D: $dimension> H5Type for $quantity<glam::DVec2, D> {
            fn type_descriptor() -> hdf5::types::TypeDescriptor {
                TypeDescriptor::FixedArray(Box::new(TypeDescriptor::Float(FloatSize::U8)), 2)
            }
        }
    };
}
