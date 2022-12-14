#[macro_export]
macro_rules! define_quantity {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
        pub struct $quantity<S: 'static, const D: $dimension>(pub(crate) S);

        impl<S> $quantity<S, { $dimensionless_const }> {
            /// Get the value of a dimensionless quantity
            pub fn value(self) -> S {
                self.0
            }

            /// Get a reference to the value of a dimensionless quantity
            pub fn value_ref(&self) -> &S {
                &self.0
            }
        }

        impl<S, const D: $dimension> $quantity<S, D> {
            /// Return the value of a quantity, regardless of whether
            /// it is dimensionless or not. Use this carefully, since the
            /// result depends on the underlying base units
            pub fn value_unchecked(self) -> S {
                self.0
            }

            /// Create a new quantity for the dimension with a given value.
            /// Use carefully, since the constructed quantity depends on the
            /// used base units.
            pub const fn new_unchecked(s: S) -> Self {
                Self(s)
            }
        }

        impl<S, const D: $dimension> std::ops::Add for $quantity<S, D>
        where
            S: std::ops::Add<Output = S>,
        {
            type Output = $quantity<S, D>;

            fn add(self, rhs: Self) -> Self::Output {
                $quantity::<S, D>(self.0 + rhs.0)
            }
        }

        impl<S, const D: $dimension> std::ops::AddAssign for $quantity<S, D>
        where
            S: std::ops::AddAssign<S>,
        {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl<S, const D: $dimension> std::ops::Sub for $quantity<S, D>
        where
            S: std::ops::Sub<Output = S>,
        {
            type Output = $quantity<S, D>;

            fn sub(self, rhs: Self) -> Self::Output {
                $quantity::<S, D>(self.0 - rhs.0)
            }
        }

        impl<S, const D: $dimension> std::ops::SubAssign for $quantity<S, D>
        where
            S: std::ops::SubAssign<S>,
        {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl<S, const D: $dimension> std::ops::Neg for $quantity<S, D>
        where
            S: std::ops::Neg<Output = S>,
        {
            type Output = $quantity<S, D>;

            fn neg(self) -> Self::Output {
                $quantity::<S, D>(-self.0)
            }
        }

        impl<const D: $dimension, S: Default + std::ops::AddAssign<S>> std::iter::Sum
            for $quantity<S, D>
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut total = Self::default();
                for item in iter {
                    total += item;
                }
                total
            }
        }

        impl<S> std::ops::Deref for $quantity<S, $dimensionless_const> {
            type Target = S;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<S> std::ops::DerefMut for $quantity<S, $dimensionless_const> {
            fn deref_mut(&mut self) -> &mut S {
                &mut self.0
            }
        }

        impl<S> std::fmt::Display for $quantity<S, { $dimensionless_const }>
        where
            S: std::fmt::Display,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value_ref())
            }
        }
    };
}

// #[cfg(...)] attributes are evaluated in the context of the caller.
// This makes passing feature flags of the macro crate into the macro
// tricky, which is why each of the following macros are defined twice.
// There has to be a better way to do this but I don't know what it is.

#[cfg(feature = "hdf5")]
#[macro_export]
macro_rules! impl_hdf5_gated {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        $crate::impl_hdf5!($quantity, $dimension, $dimensionless_const);
    };
}

#[cfg(not(feature = "hdf5"))]
#[macro_export]
macro_rules! impl_hdf5_gated {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {};
}

#[cfg(feature = "mpi")]
#[macro_export]
macro_rules! impl_mpi_gated {
    ($quantity: ident, $dimension: ident) => {
        $crate::impl_mpi!($quantity, $dimension);
    };
}

#[cfg(not(feature = "mpi"))]
#[macro_export]
macro_rules! impl_mpi_gated {
    ($quantity: ident, $dimension: ident) => {};
}

#[cfg(feature = "rand")]
#[macro_export]
macro_rules! impl_rand_gated {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        $crate::impl_rand!($quantity, $dimension, $dimensionless_const);
    };
}

#[cfg(not(feature = "rand"))]
#[macro_export]
macro_rules! impl_rand_gated {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {};
}

#[cfg(feature = "serde")]
#[macro_export]
macro_rules! impl_serde_gated {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $unit_names_array: ident) => {
        $crate::impl_serde!(
            $quantity,
            $dimension,
            $dimensionless_const,
            $unit_names_array
        );
    };
}

#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! impl_serde_gated {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $unit_names_array: ident) => {};
}
