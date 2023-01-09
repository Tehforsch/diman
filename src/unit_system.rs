#[cfg(feature = "default-f32")]
#[macro_export]
macro_rules! default_quantity {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {
        pub type $quantity_name = $quantity<f32, $const>;
    };
}

#[cfg(feature = "default-f64")]
#[macro_export]
macro_rules! default_quantity {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {
        pub type $quantity_name = $quantity<f64, $const>;
    };
}

#[cfg(all(not(feature = "default-f32"), not(feature = "default-f64")))]
#[macro_export]
macro_rules! default_quantity {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {};
}

#[cfg(all(not(feature = "default-f32"), not(feature = "default-f64")))]
#[macro_export]
macro_rules! default_vector {
    () => {};
}

#[cfg(all(
    not(feature = "default-2d"),
    not(feature = "default-3d"),
    feature = "default-f32"
))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec2 = glam::Vec2;
        pub type MVec3 = glam::Vec3;
    };
}

#[cfg(all(
    not(feature = "default-2d"),
    not(feature = "default-3d"),
    feature = "default-f64"
))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec2 = glam::DVec2;
        pub type MVec3 = glam::DVec3;
    };
}

#[cfg(all(feature = "default-2d", feature = "default-f32"))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = glam::Vec2;
        pub type MVec2 = glam::Vec2;
        pub type MVec3 = glam::Vec3;
    };
}

#[cfg(all(feature = "default-3d", feature = "default-f32"))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = glam::Vec3;
        pub type MVec2 = glam::Vec2;
        pub type MVec3 = glam::Vec3;
    };
}

#[cfg(all(feature = "default-2d", feature = "default-f64"))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = glam::DVec2;
        pub type MVec2 = glam::DVec2;
        pub type MVec3 = glam::DVec3;
    };
}

#[cfg(all(feature = "default-3d", feature = "default-f64"))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = glam::DVec3;
        pub type MVec2 = glam::DVec2;
        pub type MVec3 = glam::DVec3;
    };
}

#[macro_export]
macro_rules! unit_system {
    ($dimension: ident, $quantity: ident, $dimensionless_const: ident, $unit_names_array: ident, $($const: ident, $quantity_name:ident, $($dimension_name: ident: $dimension_value: literal),*, {$($unit:ident, $factor:literal, $($unit_symbol:literal)?),*}),+) => {
        use paste::paste;
        pub const $unit_names_array: &[($dimension, &str, f64)] = &[
        $(
            $(
                $(
                    ($const, $unit_symbol, $factor),
                )*
            )*
        )*
        ];
        $(
            #[allow(clippy::needless_update)]
            pub const $const: $dimension = $dimension {
                $(
                    $dimension_name: $dimension_value,
                )*
                .. $dimensionless_const };



            $crate::default_quantity!($quantity, $quantity_name, $const);

            $crate::default_vector_quantities!($quantity, $quantity_name, $const);

            paste!{
                pub type [<F32 $quantity_name>] = $quantity<f32, $const>;
                pub type [<F64 $quantity_name>] = $quantity<f64, $const>;
            }

            impl $quantity::<f64, $const> {
                $(
                    pub const fn $unit(v: f64) -> $quantity::<f64, $const> {
                        $quantity::<f64, $const>(v * $factor)
                    }

                )*
            }

            impl $quantity::<f32, $const> {
                $(
                    pub const fn $unit(v: f64) -> $quantity::<f32, $const> {
                        $quantity::<f32, $const>((v * ($factor as f64)) as f32)
                    }
                )*
            }

            $(
                $crate::vector_unit_constructors!($quantity, $const, $unit, $factor);
            )*

            impl<S> $quantity<S, $const> where S: std::ops::Div<f64, Output = S> {
                paste! {
                    $(
                        pub fn [<in_ $unit>](self) -> S {
                            self.0 / $factor
                        }
                    )*
                }
            }
        )*
    }
}
