#[macro_export]
macro_rules! impl_mul_quantity_quantity {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const DL: $dimension, const DR: $dimension> std::ops::Mul<$quantity<$type_rhs, DR>>
            for $quantity<$type_lhs, DL>
        where
            $quantity<$type_lhs, { DL.dimension_mul(DR) }>:,
        {
            type Output = $quantity<
                <$type_lhs as std::ops::Mul<$type_rhs>>::Output,
                { DL.dimension_mul(DR) },
            >;

            fn mul(self, rhs: $quantity<$type_rhs, DR>) -> Self::Output {
                $quantity(self.0 * rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_quantity_type {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const D: $dimension> std::ops::Mul<$type_rhs> for $quantity<$type_lhs, D> {
            type Output = $quantity<<$type_lhs as std::ops::Mul<$type_rhs>>::Output, D>;

            fn mul(self, rhs: $type_rhs) -> Self::Output {
                $quantity(self.0 * rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_assign_quantity_type {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const D: $dimension> std::ops::MulAssign<$type_rhs> for $quantity<$type_lhs, D> {
            fn mul_assign(&mut self, rhs: $type_rhs) {
                self.0 *= rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_type_quantity {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const D: $dimension> std::ops::Mul<$quantity<$type_rhs, D>> for $type_lhs {
            type Output = $quantity<<$type_lhs as std::ops::Mul<$type_rhs>>::Output, D>;

            fn mul(self, rhs: $quantity<$type_rhs, D>) -> Self::Output {
                $quantity(self * rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div_quantity_quantity {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const DL: $dimension, const DR: $dimension> std::ops::Div<$quantity<$type_rhs, DR>>
            for $quantity<$type_lhs, DL>
        where
            $quantity<$type_lhs, { DL.dimension_div(DR) }>:,
        {
            type Output = $quantity<
                <$type_lhs as std::ops::Div<$type_rhs>>::Output,
                { DL.dimension_div(DR) },
            >;

            fn div(self, rhs: $quantity<$type_rhs, DR>) -> Self::Output {
                $quantity(self.0 / rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div_quantity_type {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const D: $dimension> std::ops::Div<$type_rhs> for $quantity<$type_lhs, D> {
            type Output = $quantity<<$type_lhs as std::ops::Div<$type_rhs>>::Output, D>;

            fn div(self, rhs: $type_rhs) -> Self::Output {
                $quantity(self.0 / rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div_assign_quantity_type {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const D: $dimension> std::ops::DivAssign<$type_rhs> for $quantity<$type_lhs, D> {
            fn div_assign(&mut self, rhs: $type_rhs) {
                self.0 /= rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div_type_quantity {
    ($quantity: ident, $dimension: ty, $type_lhs: ty, $type_rhs: ty) => {
        impl<const D: $dimension> std::ops::Div<$quantity<$type_rhs, D>> for $type_lhs
        where
            $quantity<$type_lhs, { D.dimension_inv() }>:,
        {
            type Output =
                $quantity<<$type_lhs as std::ops::Div<$type_rhs>>::Output, { D.dimension_inv() }>;

            fn div(self, rhs: $quantity<$type_rhs, D>) -> Self::Output {
                $quantity(self / rhs.0)
            }
        }
    };
}
