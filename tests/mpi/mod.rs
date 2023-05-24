#[cfg(test)]
#[cfg(feature = "mpi")]
#[cfg(feature = "f64")]
mod tests {
    use mpi::environment::Universe;
    use mpi::traits::Communicator;
    use mpi::Threading;

    use crate::example_system::f64::Length;

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
        use crate::example_system::vec2::Length as VecLength;
        let world = MPI_UNIVERSE.world();
        let q1 = VecLength::meters(1.0, 2.0);
        let mut q2 = VecLength::meters(3.0, 4.0);
        let a = world.pack(&q1);
        unsafe {
            world.unpack_into(&a, &mut q2, 0);
        }
        assert_eq!(q1, q2);
    }
}
