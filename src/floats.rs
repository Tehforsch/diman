#[macro_export]
macro_rules! impl_float_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        impl<const D: $dimension, S: num_traits::Float> $quantity<S, D> {
            pub fn squared(&self) -> $quantity<S, { D.dimension_powi(2) }>
            where
                $quantity<S, { D.dimension_powi(2) }>:,
            {
                $quantity::<S, { D.dimension_powi(2) }>(self.0.powi(2))
            }

            pub fn cubed(&self) -> $quantity<S, { D.dimension_powi(3) }>
            where
                $quantity<S, { D.dimension_powi(3) }>:,
            {
                $quantity::<S, { D.dimension_powi(3) }>(self.0.powi(3))
            }

            pub fn powi<const I: i32>(&self) -> $quantity<S, { D.dimension_powi(I) }>
            where
                $quantity<S, { D.dimension_powi(I) }>:,
            {
                $quantity::<S, { D.dimension_powi(I) }>(self.0.powi(I))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_concrete_float_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $unit_names_array: ident, $float_type: ident) => {
        impl<const D: $dimension> $quantity<$float_type, D> {
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            pub fn zero() -> Self {
                Self(0.0)
            }

            pub fn one() -> Self {
                Self(1.0)
            }

            pub fn abs(&self) -> Self {
                Self(self.0.abs())
            }
        }

        $crate::impl_mul_quantity_quantity!($quantity, $dimension, $float_type, $float_type);

        $crate::impl_mul_quantity_type!($quantity, $dimension, $float_type, $float_type);
        $crate::impl_mul_type_quantity!($quantity, $dimension, $float_type, $float_type);

        $crate::impl_div_quantity_quantity!($quantity, $dimension, $float_type, $float_type);

        $crate::impl_div_quantity_type!($quantity, $dimension, $float_type, $float_type);
        $crate::impl_div_type_quantity!($quantity, $dimension, $float_type, $float_type);

        impl<const D: $dimension> $quantity<$float_type, D> {
            pub fn in_units(self, other: $quantity<$float_type, D>) -> $float_type
            where
                $quantity<$float_type, { D.dimension_div(D) }>:,
            {
                (self / other).value_unchecked()
            }
        }

        impl<const D: $dimension> std::fmt::Debug for $quantity<$float_type, D> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let unit_name = $unit_names_array
                    .iter()
                    .filter(|(d, _, _)| d == &D)
                    .filter(|(_, _, val)| *val == 1.0)
                    .map(|(_, name, _)| name)
                    .next()
                    .unwrap_or(&"unknown unit");
                self.0.fmt(f).and_then(|_| write!(f, " {}", unit_name))
            }
        }
    };
}
