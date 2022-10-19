#[macro_export]
macro_rules! impl_dimensionless_method {
    ($quantity: ident, $dimensionless_const: ident, $storage_type: ty, $method_name: ident) => {
        impl $quantity<$storage_type, $dimensionless_const> {
            pub fn $method_name(&self) -> $quantity<$storage_type, $dimensionless_const> {
                Self(self.0.$method_name())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_method {
    ($quantity: ident, $dimension: ty, $storage_type: ty, $method_name: ident) => {
        impl<const D: $dimension> $quantity<$storage_type, D> {
            pub fn $method_name(&self) -> $quantity<$storage_type, D> {
                Self(self.0.$method_name())
            }
        }
    };
}
