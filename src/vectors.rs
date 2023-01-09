#[cfg(all(
    feature = "glam",
    any(feature = "default-2d", feature = "default-3d"),
    any(feature = "default-f32", feature = "default-f64")
))]
#[macro_export]
macro_rules! default_vector_quantity {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {
        paste! {
            pub type [<Vec $quantity_name>] = $quantity<MVec, $const>;
        }
    };
}

#[cfg(not(all(
    feature = "glam",
    any(feature = "default-2d", feature = "default-3d"),
    any(feature = "default-f32", feature = "default-f64")
)))]
#[macro_export]
macro_rules! default_vector_quantity {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {};
}

#[cfg(all(
    feature = "glam",
    any(feature = "default-f32", feature = "default-f64")
))]
#[macro_export]
macro_rules! default_vector_quantities {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {
        paste! {
            pub type [<Vec2 $quantity_name>] = $quantity<MVec2, $const>;
            pub type [<Vec3 $quantity_name>] = $quantity<MVec3, $const>;
        }

        $crate::default_vector_quantity!($quantity, $quantity_name, $const);
    };
}

#[cfg(not(all(
    feature = "glam",
    any(feature = "default-f32", feature = "default-f64")
)))]
#[macro_export]
macro_rules! default_vector_quantities {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {};
}

#[cfg(feature = "glam")]
#[macro_export]
macro_rules! vector_unit_constructors {
    ($quantity: ident, $const: ident, $unit: ident, $factor: literal) => {
        impl $quantity<glam::Vec2, $const> {
            pub fn $unit(x: f32, y: f32) -> $quantity<glam::Vec2, $const> {
                $quantity::<glam::Vec2, $const>(glam::Vec2::new(x, y) * $factor)
            }
        }
        impl $quantity<glam::Vec3, $const> {
            pub fn $unit(x: f32, y: f32, z: f32) -> $quantity<glam::Vec3, $const> {
                $quantity::<glam::Vec3, $const>(glam::Vec3::new(x, y, z) * $factor)
            }
        }
        impl $quantity<glam::DVec2, $const> {
            pub fn $unit(x: f64, y: f64) -> $quantity<glam::DVec2, $const> {
                $quantity::<glam::DVec2, $const>(glam::DVec2::new(x, y) * $factor)
            }
        }
        impl $quantity<glam::DVec3, $const> {
            pub fn $unit(x: f64, y: f64, z: f64) -> $quantity<glam::DVec3, $const> {
                $quantity::<glam::DVec3, $const>(glam::DVec3::new(x, y, z) * $factor)
            }
        }
    };
}

#[cfg(not(feature = "glam"))]
#[macro_export]
macro_rules! vector_unit_constructors {
    ($quantity: ident, $const: ident, $unit: ident, $factor: literal) => {};
}

#[macro_export]
macro_rules! impl_vector_methods {
    ($quantity: ident, $dimension: ty, $dimensionless_const: ident, $unit_names_array: ident, $vector_type: ty, $float_type: ty, $num_dims: literal) => {
        $crate::impl_mul_quantity_quantity!($quantity, $dimension, $vector_type, $float_type);
        $crate::impl_mul_quantity_quantity!($quantity, $dimension, $float_type, $vector_type);

        $crate::impl_div_quantity_quantity!($quantity, $dimension, $vector_type, $float_type);

        $crate::impl_mul_quantity_type!($quantity, $dimension, $vector_type, $float_type);
        $crate::impl_mul_assign_quantity_type!($quantity, $dimension, $vector_type, $float_type);
        $crate::impl_div_assign_quantity_type!($quantity, $dimension, $vector_type, $float_type);
        $crate::impl_mul_quantity_type!($quantity, $dimension, $float_type, $vector_type);
        $crate::impl_mul_type_quantity!($quantity, $dimension, $vector_type, $float_type);
        $crate::impl_mul_type_quantity!($quantity, $dimension, $float_type, $vector_type);

        $crate::impl_div_quantity_type!($quantity, $dimension, $vector_type, $float_type);
        $crate::impl_div_type_quantity!($quantity, $dimension, $vector_type, $float_type);

        $crate::impl_method!($quantity, $dimension, $vector_type, abs);

        $crate::impl_method!($quantity, $dimension, $vector_type, exp);

        impl<const D: $dimension> $quantity<$vector_type, D> {
            pub fn from_vector_and_scale(
                vec: $vector_type,
                scale: $quantity<$float_type, D>,
            ) -> Self {
                Self(vec) * scale.0
            }

            pub fn in_units(self, other: $quantity<$float_type, D>) -> $vector_type
            where
                $quantity<$float_type, { D.dimension_div(D) }>:,
            {
                (self / other).value_unchecked()
            }

            pub fn x(&self) -> $quantity<$float_type, D> {
                $quantity(self.0.x)
            }

            pub fn y(&self) -> $quantity<$float_type, D> {
                $quantity(self.0.y)
            }

            pub fn set_x(&mut self, new_x: $quantity<$float_type, D>) {
                self.0.x = new_x.value_unchecked();
            }

            pub fn set_y(&mut self, new_y: $quantity<$float_type, D>) {
                self.0.y = new_y.value_unchecked();
            }

            pub fn min(self, rhs: Self) -> Self {
                Self(self.0.min(rhs.0))
            }

            pub fn max(self, rhs: Self) -> Self {
                Self(self.0.max(rhs.0))
            }

            pub fn length(&self) -> $quantity<$float_type, D> {
                $quantity::<$float_type, D>(self.0.length())
            }

            pub fn distance(&self, other: &Self) -> $quantity<$float_type, D> {
                $quantity::<$float_type, D>(self.0.distance(other.0))
            }

            pub fn distance_squared(
                &self,
                other: &Self,
            ) -> $quantity<$float_type, { D.dimension_powi(2) }>
            where
                $quantity<$float_type, { D.dimension_powi(2) }>:,
            {
                $quantity::<$float_type, { D.dimension_powi(2) }>(self.0.distance_squared(other.0))
            }

            pub fn normalize(&self) -> $quantity<$vector_type, $dimensionless_const> {
                $quantity::<$vector_type, $dimensionless_const>(self.0.normalize())
            }

            pub fn dot<const DR: Dimension>(
                self,
                rhs: Quantity<$vector_type, DR>,
            ) -> $quantity<$float_type, { D.dimension_mul(DR) }> {
                $quantity(self.0.dot(rhs.0))
            }
        }

        impl<const D: $dimension> std::fmt::Debug for $quantity<$vector_type, D> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let unit_name = $unit_names_array
                    .iter()
                    .filter(|(d, _, _)| d == &D)
                    .filter(|(_, _, val)| *val == 1.0)
                    .map(|(_, name, _)| name)
                    .next()
                    .unwrap_or(&"unknown unit");
                write!(f, "[")?;
                let array = self.0.to_array();
                for dim in 0..$num_dims {
                    array[dim].fmt(f)?;
                    if dim != $num_dims - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "] {}", unit_name)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vector2_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $vector_type: ty, $float_type: ty) => {
        impl<const D: $dimension> $quantity<$vector_type, D> {
            pub fn new(x: $quantity<$float_type, D>, y: $quantity<$float_type, D>) -> Self {
                Self(<$vector_type>::new(
                    x.value_unchecked(),
                    y.value_unchecked(),
                ))
            }

            pub fn new_x(x: $quantity<$float_type, D>) -> Self {
                Self(<$vector_type>::new(x.value_unchecked(), 0.0))
            }

            pub fn new_y(y: $quantity<$float_type, D>) -> Self {
                Self(<$vector_type>::new(0.0, y.value_unchecked()))
            }

            pub fn zero() -> Self {
                Self(<$vector_type>::new(0.0, 0.0))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_vector3_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $vector_type: ty, $float_type: ty) => {
        impl<const D: $dimension> $quantity<$vector_type, D> {
            pub fn new(
                x: $quantity<$float_type, D>,
                y: $quantity<$float_type, D>,
                z: $quantity<$float_type, D>,
            ) -> Self {
                Self(<$vector_type>::new(
                    x.value_unchecked(),
                    y.value_unchecked(),
                    z.value_unchecked(),
                ))
            }

            pub fn new_x(x: $quantity<$float_type, D>) -> Self {
                Self(<$vector_type>::new(x.value_unchecked(), 0.0, 0.0))
            }

            pub fn new_y(y: $quantity<$float_type, D>) -> Self {
                Self(<$vector_type>::new(0.0, y.value_unchecked(), 0.0))
            }

            pub fn new_z(z: $quantity<$float_type, D>) -> Self {
                Self(<$vector_type>::new(0.0, 0.0, z.value_unchecked()))
            }

            pub fn z(&self) -> $quantity<$float_type, D> {
                $quantity(self.0.z)
            }

            pub fn set_z(&mut self, new_z: $quantity<$float_type, D>) {
                self.0.z = new_z.value_unchecked();
            }

            pub fn zero() -> Self {
                Self(<$vector_type>::new(0.0, 0.0, 0.0))
            }
        }
    };
}

#[cfg(test)]
#[cfg(feature = "glam")]
#[cfg(any(feature = "default-f32", feature = "default-f64"))]
mod tests {
    #[test]
    fn debug_vector_2() {
        assert_eq!(
            format!("{:?}", crate::si::Vec2Length::meters(1.0, 5.0)),
            "[1.0 5.0] m"
        );
    }

    #[test]
    fn debug_vector_3() {
        assert_eq!(
            format!("{:?}", crate::si::Vec3Length::meters(1.0, 5.0, 6.0)),
            "[1.0 5.0 6.0] m"
        );
    }

    use crate::si::{Length, MVec2, MVec3, Time, Vec3Velocity};
    use crate::test_utils::assert_is_close;

    #[test]
    fn mul_vec3() {
        let multiplied = MVec3::new(1.0, 2.0, 3.0) * Length::meters(5.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
        let multiplied = Length::meters(5.0) * MVec3::new(1.0, 2.0, 3.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
    }

    #[test]
    fn mul_assign_vec3() {
        let mut vec = crate::si::Vec3Length::meters(1.0, 2.0, 3.0);
        vec *= 3.0;
        assert_is_close(vec.x(), Length::meters(3.0));
        assert_is_close(vec.y(), Length::meters(6.0));
        assert_is_close(vec.z(), Length::meters(9.0));
    }

    #[test]
    fn div_assign_vec3() {
        let mut vec = crate::si::Vec3Length::meters(1.0, 2.0, 3.0);
        vec /= 2.0;
        assert_is_close(vec.x(), Length::meters(0.5));
        assert_is_close(vec.y(), Length::meters(1.0));
        assert_is_close(vec.z(), Length::meters(1.5));
    }

    #[test]
    fn mul_quantity_vec3() {
        let multiplied = Vec3Velocity::meters_per_second(1.0, 2.0, 3.0) * Time::seconds(5.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
        let multiplied = Time::seconds(5.0) * Vec3Velocity::meters_per_second(1.0, 2.0, 3.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
    }

    #[test]
    fn div_vec3() {
        let divided = MVec3::new(1.0, 2.0, 3.0) / Length::meters(0.2);
        let base = 1.0 / Length::meters(1.0);
        assert_is_close(divided.x(), 5.0 * base);
        assert_is_close(divided.y(), 10.0 * base);
        assert_is_close(divided.z(), 15.0 * base);
    }

    #[test]
    fn mul_vec2() {
        let multiplied = MVec2::new(1.0, 2.0) * Length::meters(5.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        let multiplied = Length::meters(5.0) * MVec2::new(1.0, 2.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
    }

    #[test]
    fn mul_assign_vec2() {
        let mut vec = crate::si::Vec2Length::meters(1.0, 2.0);
        vec *= 3.0;
        assert_is_close(vec.x(), Length::meters(3.0));
        assert_is_close(vec.y(), Length::meters(6.0));
    }

    #[test]
    fn div_assign_vec2() {
        let mut vec = crate::si::Vec2Length::meters(1.0, 2.0);
        vec /= 2.0;
        assert_is_close(vec.x(), Length::meters(0.5));
        assert_is_close(vec.y(), Length::meters(1.0));
    }

    #[test]
    fn div_vec2() {
        let divided = MVec2::new(1.0, 2.0) / Length::meters(0.2);
        let base = 1.0 / Length::meters(1.0);
        assert_is_close(divided.x(), 5.0 * base);
        assert_is_close(divided.y(), 10.0 * base);
    }
}
