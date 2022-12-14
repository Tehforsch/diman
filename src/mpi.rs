#[macro_export]
macro_rules! impl_mpi_float {
    ($quantity: ident, $dimension: ident, $float_type: ty, $ffi_type: expr) => {
        unsafe impl<const D: $dimension> ::mpi::traits::Equivalence for $quantity<$float_type, D> {
            type Out = ::mpi::datatype::SystemDatatype;

            fn equivalent_datatype() -> Self::Out {
                unsafe {
                    <::mpi::datatype::DatatypeRef as ::mpi::raw::FromRaw>::from_raw($ffi_type)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mpi_vector {
    ($quantity: ident, $dimension: ident, $vector_type: ty, $float_type: ty, $num_dims: literal) => {
        unsafe impl<const D: $dimension> ::mpi::traits::Equivalence for $quantity<$vector_type, D> {
            type Out = ::mpi::datatype::DatatypeRef<'static>;

            fn equivalent_datatype() -> Self::Out {
                static DATATYPE: ::once_cell::sync::Lazy<::mpi::datatype::UserDatatype> =
                    ::once_cell::sync::Lazy::new(|| {
                        ::mpi::datatype::UserDatatype::contiguous(
                            $num_dims,
                            &<$float_type>::equivalent_datatype(),
                        )
                    });
                DATATYPE.as_ref()
            }
        }
    };
}

#[cfg(feature = "glam")]
#[macro_export]
macro_rules! impl_mpi_glam {
    ($quantity: ident, $dimension: ident) => {
        $crate::impl_mpi_vector!($quantity, $dimension, ::glam::Vec2, f32, 2);
        $crate::impl_mpi_vector!($quantity, $dimension, ::glam::DVec2, f64, 2);
        $crate::impl_mpi_vector!($quantity, $dimension, ::glam::Vec3, f32, 3);
        $crate::impl_mpi_vector!($quantity, $dimension, ::glam::DVec3, f64, 3);
    };
}

#[cfg(not(feature = "glam"))]
#[macro_export]
macro_rules! impl_mpi_glam {
    ($quantity: ident, $dimension: ident) => {};
}

#[macro_export]
macro_rules! impl_mpi {
    ($quantity: ident, $dimension: ident) => {
        $crate::impl_mpi_float!($quantity, $dimension, f32, ::mpi::ffi::RSMPI_FLOAT);
        $crate::impl_mpi_float!($quantity, $dimension, f64, ::mpi::ffi::RSMPI_DOUBLE);

        $crate::impl_mpi_glam!($quantity, $dimension);
    };
}

#[cfg(test)]
#[cfg(feature = "mpi")]
#[cfg(any(feature = "default-f32", feature = "default-f64"))]
mod tests {
    use mpi::environment::Universe;
    use mpi::traits::Communicator;
    use mpi::Threading;

    use crate::si::Length;

    lazy_static::lazy_static! {
        pub static ref MPI_UNIVERSE: Universe = {
            let threading = Threading::Single;
            let (universe, _) =
                mpi::initialize_with_threading(threading).unwrap();
            universe
        };
    }

    #[test]
    fn pack_unpack_f64_quantity() {
        let world = MPI_UNIVERSE.world();
        let q1 = Length::meters(1.0);
        let mut q2 = Length::meters(2.0);
        let a = world.pack(&q1);
        unsafe {
            world.unpack_into(&a, &mut q2, 0);
        }
        assert_eq!(q1, q2);
    }

    #[cfg(feature = "glam")]
    #[test]
    fn pack_unpack_vec_quantity() {
        use crate::si::Vec2Length;
        let world = MPI_UNIVERSE.world();
        let q1 = Vec2Length::meters(1.0, 2.0);
        let mut q2 = Vec2Length::meters(3.0, 4.0);
        let a = world.pack(&q1);
        unsafe {
            world.unpack_into(&a, &mut q2, 0);
        }
        assert_eq!(q1, q2);
    }
}
