#[macro_export]
macro_rules! impl_rand {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        use rand::distributions::uniform::SampleBorrow;
        use rand::distributions::uniform::SampleUniform;
        use rand::distributions::uniform::UniformFloat;
        use rand::distributions::uniform::UniformSampler;
        use rand::prelude::*;

        #[derive(Clone, Copy, Debug)]
        pub struct UniformQuantity<S, const D: $dimension>(UniformFloat<S>);

        $crate::impl_rand_floats!($quantity, $dimension, $dimensionless_const, f32);
        $crate::impl_rand_floats!($quantity, $dimension, $dimensionless_const, f64);
    };
}
#[macro_export]
macro_rules! impl_rand_floats {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $float_type: ident) => {
        impl<const D: $dimension> UniformSampler for UniformQuantity<$float_type, D> {
            type X = $quantity<$float_type, D>;
            fn new<B1, B2>(low: B1, high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                UniformQuantity::<$float_type, D>(UniformFloat::<$float_type>::new(
                    low.borrow().0,
                    high.borrow().0,
                ))
            }
            fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                UniformQuantity::<$float_type, D>(UniformFloat::<$float_type>::new_inclusive(
                    low.borrow().0,
                    high.borrow().0,
                ))
            }
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                $quantity::<$float_type, D>(self.0.sample(rng))
            }
        }

        impl<const D: $dimension> SampleUniform for $quantity<$float_type, D> {
            type Sampler = UniformQuantity<$float_type, D>;
        }
    };
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::si::Length;

    #[test]
    fn test_random_quantity_generation() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen_range(Length::meters(0.0)..Length::meters(1.0));
            assert!(Length::meters(0.0) <= x);
            assert!(x < Length::meters(1.0));
        }
    }
}
