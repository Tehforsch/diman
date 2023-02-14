#[macro_export]
macro_rules! impl_float_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {};
}

#[macro_export]
macro_rules! impl_concrete_float_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $unit_names_array: ident, $float_type: ident) => {
        impl<const D: $dimension> $quantity<$float_type, D> {
            pub fn squared(&self) -> $quantity<$float_type, { D.dimension_powi(2) }>
            where
                $quantity<$float_type, { D.dimension_powi(2) }>:,
            {
                $quantity::<$float_type, { D.dimension_powi(2) }>(self.0.powi(2))
            }

            pub fn cubed(&self) -> $quantity<$float_type, { D.dimension_powi(3) }>
            where
                $quantity<$float_type, { D.dimension_powi(3) }>:,
            {
                $quantity::<$float_type, { D.dimension_powi(3) }>(self.0.powi(3))
            }

            pub fn powi<const I: i32>(&self) -> $quantity<$float_type, { D.dimension_powi(I) }>
            where
                $quantity<$float_type, { D.dimension_powi(I) }>:,
            {
                $quantity::<$float_type, { D.dimension_powi(I) }>(self.0.powi(I))
            }

            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }

            pub fn zero() -> Self {
                Self(0.0)
            }

            pub fn is_positive(&self) -> bool {
                self.0 > 0.0
            }

            pub fn is_positive_or_zero(&self) -> bool {
                self.0 >= 0.0
            }

            pub fn is_negative(&self) -> bool {
                self.0 < 0.0
            }

            pub fn is_negative_or_zero(&self) -> bool {
                self.0 <= 0.0
            }
        }

        impl<const D: $dimension> $quantity<$float_type, D>
        where
            $quantity<$float_type, { D.dimension_sqrt() }>:,
        {
            pub fn sqrt(self) -> $quantity<$float_type, { D.dimension_sqrt() }> {
                $quantity::<$float_type, { D.dimension_sqrt() }>(self.0.sqrt())
            }
        }

        impl<const D: $dimension> $quantity<$float_type, D>
        where
            $quantity<$float_type, { D.dimension_cbrt() }>:,
        {
            pub fn cbrt(self) -> $quantity<$float_type, { D.dimension_cbrt() }> {
                $quantity::<$float_type, { D.dimension_cbrt() }>(self.0.cbrt())
            }
        }

        impl std::ops::Add<$float_type> for $quantity<$float_type, $dimensionless_const> {
            type Output = $quantity<$float_type, $dimensionless_const>;

            fn add(self, rhs: $float_type) -> Self::Output {
                Self(self.0 + rhs)
            }
        }

        impl std::ops::AddAssign<$float_type> for $quantity<$float_type, $dimensionless_const> {
            fn add_assign(&mut self, rhs: $float_type) {
                self.0 += rhs;
            }
        }

        impl std::ops::Sub<$float_type> for $quantity<$float_type, $dimensionless_const> {
            type Output = $quantity<$float_type, $dimensionless_const>;

            fn sub(self, rhs: $float_type) -> Self::Output {
                Self(self.0 - rhs)
            }
        }

        impl std::ops::SubAssign<$float_type> for $quantity<$float_type, $dimensionless_const> {
            fn sub_assign(&mut self, rhs: $float_type) {
                self.0 -= rhs;
            }
        }

        impl std::ops::Add<$quantity<$float_type, $dimensionless_const>> for $float_type {
            type Output = $float_type;

            fn add(self, rhs: $quantity<$float_type, $dimensionless_const>) -> Self::Output {
                self + rhs.0
            }
        }

        impl std::ops::AddAssign<$quantity<$float_type, $dimensionless_const>> for $float_type {
            fn add_assign(&mut self, rhs: $quantity<$float_type, $dimensionless_const>) {
                *self += rhs.0;
            }
        }

        impl std::ops::Sub<$quantity<$float_type, $dimensionless_const>> for $float_type {
            type Output = $float_type;

            fn sub(self, rhs: $quantity<$float_type, $dimensionless_const>) -> Self::Output {
                self - rhs.0
            }
        }

        impl std::ops::SubAssign<$quantity<$float_type, $dimensionless_const>> for $float_type {
            fn sub_assign(&mut self, rhs: $quantity<$float_type, $dimensionless_const>) {
                *self -= rhs.0
            }
        }

        $crate::impl_mul_quantity_quantity!($quantity, $dimension, $float_type, $float_type);

        $crate::impl_mul_quantity_type!($quantity, $dimension, $float_type, $float_type);
        $crate::impl_mul_assign_quantity_type!($quantity, $dimension, $float_type, $float_type);
        $crate::impl_div_assign_quantity_type!($quantity, $dimension, $float_type, $float_type);
        $crate::impl_mul_type_quantity!($quantity, $dimension, $float_type, $float_type);

        $crate::impl_div_quantity_quantity!($quantity, $dimension, $float_type, $float_type);

        $crate::impl_div_quantity_type!($quantity, $dimension, $float_type, $float_type);
        $crate::impl_div_type_quantity!($quantity, $dimension, $float_type, $float_type);

        $crate::impl_method!($quantity, $dimension, $float_type, abs);

        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, log2);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, ln);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, log10);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, exp);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, exp2);

        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, ceil);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, floor);

        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, sin);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, cos);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, tan);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, asin);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, acos);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, atan);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, sinh);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, cosh);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, tanh);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, asinh);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, acosh);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, atanh);

        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, exp_m1);
        $crate::impl_dimensionless_method!($quantity, $dimensionless_const, $float_type, ln_1p);

        impl $quantity<$float_type, $dimensionless_const> {
            pub const EPSILON: Self = Self($float_type::EPSILON);
            pub const PI: Self = Self(::std::$float_type::consts::PI);
            pub const E: Self = Self(::std::$float_type::consts::E);
        }

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
                let closeness = |value: f64, unit_factor: f64| {
                    if value == 0.0 {
                        1.0
                    } else {
                        (value / unit_factor).abs().ln().abs()
                    }
                };
                let (unit_name, unit_value) = $unit_names_array
                    .iter()
                    .filter(|(d, _, _)| d == &D)
                    .min_by(|(_, _, x), (_, _, y)| {
                        closeness(self.0 as f64, *x)
                            .partial_cmp(&closeness(self.0 as f64, *y))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(_, name, val)| (name, val))
                    .unwrap_or((&"unknown unit", &1.0));
                (self.0 as f64 / unit_value)
                    .fmt(f)
                    .and_then(|_| write!(f, " {}", unit_name))
            }
        }
    };
}
