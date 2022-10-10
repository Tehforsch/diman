#[cfg(feature = "default-f32")]
#[macro_export]
macro_rules! default_quantity {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {
        pub type $quantity_name = $quantity<f32, $const>;
    };
}

#[cfg(not(feature = "default-f32"))]
#[macro_export]
macro_rules! default_quantity {
    ($quantity: ident, $quantity_name: ident, $const: ident) => {
        pub type $quantity_name = $quantity<f64, $const>;
    };
}

#[cfg(all(feature = "default-2d", feature = "default-f32"))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = Vec2;
        pub type MVec2 = Vec2;
        pub type MVec3 = Vec3;
    };
}

#[cfg(all(not(feature = "default-2d"), feature = "default-f32"))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = Vec3;
        pub type MVec2 = Vec2;
        pub type MVec3 = Vec3;
    };
}

#[cfg(all(feature = "default-2d", not(feature = "default-f32")))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = DVec2;
        pub type MVec2 = DVec2;
        pub type MVec3 = DVec3;
    };
}

#[cfg(all(not(feature = "default-2d"), not(feature = "default-f32")))]
#[macro_export]
macro_rules! default_vector {
    () => {
        pub type MVec = DVec3;
        pub type MVec2 = DVec2;
        pub type MVec3 = DVec3;
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

            paste! {
                pub type [<Vec2 $quantity_name>] = $quantity<MVec2, $const>;
                pub type [<Vec3 $quantity_name>] = $quantity<MVec3, $const>;
                pub type [<Vec $quantity_name>] = $quantity<MVec, $const>;
            }

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

            impl $quantity<glam::Vec2, $const> {
                $(
                    pub fn $unit(x: f32, y: f32) -> $quantity::<glam::Vec2, $const> {
                        $quantity::<glam::Vec2, $const>(glam::Vec2::new(x, y) * $factor)
                    }
                )*
            }

            impl $quantity<glam::Vec3, $const> {
                $(
                    pub fn $unit(x: f32, y: f32, z: f32) -> $quantity::<glam::Vec3, $const> {
                        $quantity::<glam::Vec3, $const>(glam::Vec3::new(x, y, z) * $factor)
                    }
                )*
            }

            impl $quantity<glam::DVec2, $const> {
                $(
                    pub fn $unit(x: f64, y: f64) -> $quantity::<glam::DVec2, $const> {
                        $quantity::<glam::DVec2, $const>(glam::DVec2::new(x, y) * $factor)
                    }
                )*
            }

            impl $quantity<glam::DVec3, $const> {
                $(
                    pub fn $unit(x: f64, y: f64, z: f64) -> $quantity::<glam::DVec3, $const> {
                        $quantity::<glam::DVec3, $const>(glam::DVec3::new(x, y, z) * $factor)
                    }
                )*
            }

            impl<S> $quantity<S, $const> where S: Div<f64, Output = S> {
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
