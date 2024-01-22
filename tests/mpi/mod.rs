use mpi::environment::Universe;
use mpi::Threading;

lazy_static::lazy_static! {
    pub static ref MPI_UNIVERSE: Universe = {
        let threading = Threading::Single;
        let (universe, _) =
            mpi::initialize_with_threading(threading).unwrap();
        universe
    };
}

macro_rules! gen_tests_for_float {
    ($float_name: ident) => {
        mod $float_name {
            use crate::example_system::units::meters;
            use mpi::traits::Communicator;

            #[test]
            fn pack_unpack_float_quantity() {
                let world = super::MPI_UNIVERSE.world();
                let q1 = 1.0 as $float_name * meters;
                let mut q2 = 2.0 as $float_name * meters;
                let a = world.pack(&q1);
                unsafe {
                    world.unpack_into(&a, &mut q2, 0);
                }
                assert_eq!(q1, q2);
            }
        }
    };
}

#[cfg(any(feature = "glam-vec2", feature = "glam-dvec2"))]
macro_rules! gen_tests_for_vector_2 {
    ($vec_mod_name: ident, $vec_name: ident) => {
        mod $vec_mod_name {
            use crate::example_system::units::meters;
            use glam::$vec_name;
            use mpi::topology::Communicator;

            #[test]
            fn pack_unpack_vec_quantity() {
                let world = super::MPI_UNIVERSE.world();
                let q1 = <$vec_name>::new(1.0, 2.0) * meters;
                let mut q2 = <$vec_name>::new(3.0, 4.0) * meters;
                let a = world.pack(&q1);
                unsafe {
                    world.unpack_into(&a, &mut q2, 0);
                }
                assert_eq!(q1, q2);
            }
        }
    };
}

#[cfg(any(feature = "glam-vec3", feature = "glam-dvec3"))]
macro_rules! gen_tests_for_vector_3 {
    ($vec_mod_name: ident, $vec_name: ident) => {
        mod $vec_mod_name {
            use crate::example_system::units::meters;
            use glam::$vec_name;
            use mpi::topology::Communicator;

            #[test]
            fn pack_unpack_vec_quantity() {
                let world = super::MPI_UNIVERSE.world();
                let q1 = <$vec_name>::new(1.0, 2.0, 3.0) * meters;
                let mut q2 = <$vec_name>::new(4.0, 5.0, 6.0) * meters;
                let a = world.pack(&q1);
                unsafe {
                    world.unpack_into(&a, &mut q2, 0);
                }
                assert_eq!(q1, q2);
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64);

#[cfg(all(feature = "f32", feature = "glam-vec2"))]
gen_tests_for_vector_2!(vec2, Vec2);

#[cfg(all(feature = "f64", feature = "glam-dvec2"))]
gen_tests_for_vector_2!(dvec2, DVec2);

#[cfg(all(feature = "f32", feature = "glam-vec3"))]
gen_tests_for_vector_3!(vec3, Vec3);

#[cfg(all(feature = "f64", feature = "glam-dvec3"))]
gen_tests_for_vector_3!(dvec3, DVec3);
