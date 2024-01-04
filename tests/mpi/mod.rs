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
            use crate::example_system::$float_name::Length;
            use mpi::traits::Communicator;

            #[test]
            fn pack_unpack_f64_quantity() {
                let world = super::MPI_UNIVERSE.world();
                let q1 = Length::meters(1.0);
                let mut q2 = Length::meters(2.0);
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
    ($vec_mod_name: ident) => {
        mod $vec_mod_name {
            use crate::example_system::$vec_mod_name::Length as VecLength;
            use mpi::topology::Communicator;

            #[test]
            fn pack_unpack_vec_quantity() {
                let world = super::MPI_UNIVERSE.world();
                let q1 = VecLength::meters(1.0, 2.0);
                let mut q2 = VecLength::meters(3.0, 4.0);
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
    ($vec_mod_name: ident) => {
        mod $vec_mod_name {
            use crate::example_system::$vec_mod_name::Length as VecLength;
            use mpi::topology::Communicator;

            #[test]
            fn pack_unpack_vec_quantity() {
                let world = super::MPI_UNIVERSE.world();
                let q1 = VecLength::meters(1.0, 2.0, 3.0);
                let mut q2 = VecLength::meters(3.0, 4.0, 5.0);
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
gen_tests_for_vector_2!(vec2);

#[cfg(all(feature = "f64", feature = "glam-dvec2"))]
gen_tests_for_vector_2!(dvec2);

#[cfg(all(feature = "f32", feature = "glam-vec3"))]
gen_tests_for_vector_3!(vec3);

#[cfg(all(feature = "f64", feature = "glam-dvec3"))]
gen_tests_for_vector_3!(dvec3);
