#[macro_export]
macro_rules! impl_vector_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $vector_type: ident, $float_type: ident, $num_dims: literal) => {
        impl<const D: $dimension> $quantity<$vector_type, D> {
            pub fn from_vector_and_scale(
                vec: $vector_type,
                scale: $quantity<$float_type, D>,
            ) -> Self {
                Self(vec) * scale.0
            }

            pub fn abs(&self) -> Self {
                Self(self.0.abs())
            }

            pub fn x(&self) -> $quantity<$float_type, D> {
                $quantity(self.0.x)
            }

            pub fn y(&self) -> $quantity<$float_type, D> {
                $quantity(self.0.y)
            }

            pub fn set_x(&mut self, new_x: $quantity<$float_type, D>) {
                self.0.x = new_x.unwrap_value();
            }

            pub fn set_y(&mut self, new_y: $quantity<$float_type, D>) {
                self.0.y = new_y.unwrap_value();
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

            pub fn normalize(&self) -> $quantity<$vector_type, NONE> {
                $quantity::<$vector_type, NONE>(self.0.normalize())
            }
        }

        impl<const D: $dimension> std::fmt::Debug for $quantity<$vector_type, D> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let unit_name = UNIT_NAMES
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
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $vector_type: ident, $float_type: ident) => {
        impl<const D: $dimension> $quantity<$vector_type, D> {
            pub fn new(x: $quantity<$float_type, D>, y: $quantity<$float_type, D>) -> Self {
                Self($vector_type::new(x.unwrap_value(), y.unwrap_value()))
            }

            pub fn new_x(x: $quantity<$float_type, D>) -> Self {
                Self($vector_type::new(x.unwrap_value(), 0.0))
            }

            pub fn new_y(y: $quantity<$float_type, D>) -> Self {
                Self($vector_type::new(0.0, y.unwrap_value()))
            }

            pub fn zero() -> Self {
                Self($vector_type::new(0.0, 0.0))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_vector3_methods {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $vector_type: ident, $float_type: ident) => {
        impl<const D: $dimension> $quantity<$vector_type, D> {
            pub fn new(
                x: $quantity<$float_type, D>,
                y: $quantity<$float_type, D>,
                z: $quantity<$float_type, D>,
            ) -> Self {
                Self($vector_type::new(
                    x.unwrap_value(),
                    y.unwrap_value(),
                    z.unwrap_value(),
                ))
            }

            pub fn new_x(x: $quantity<$float_type, D>) -> Self {
                Self($vector_type::new(x.unwrap_value(), 0.0, 0.0))
            }

            pub fn new_y(y: $quantity<$float_type, D>) -> Self {
                Self($vector_type::new(0.0, y.unwrap_value(), 0.0))
            }

            pub fn new_z(z: $quantity<$float_type, D>) -> Self {
                Self($vector_type::new(0.0, 0.0, z.unwrap_value()))
            }

            pub fn z(&self) -> $quantity<$float_type, D> {
                $quantity(self.0.z)
            }

            pub fn set_z(&mut self, new_z: $quantity<$float_type, D>) {
                self.0.z = new_z.unwrap_value();
            }

            pub fn zero() -> Self {
                Self($vector_type::new(0.0, 0.0, 0.0))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug_vector_2() {
        assert_eq!(
            format!("{:?}", crate::si::DVec2Length::meters(1.0, 5.0)),
            "[1.0 5.0] m"
        );
    }

    #[test]
    fn debug_vector_3() {
        assert_eq!(
            format!("{:?}", crate::si::DVec3Length::meters(1.0, 5.0, 6.0)),
            "[1.0 5.0 6.0] m"
        );
    }
}
