#[macro_export]
macro_rules! unit_system {
    ($dimension: ident, $quantity: ident, $dimensionless_const: ident, $($const: ident, $quantity_name:ident, $($dimension_name: ident: $dimension_value: literal),*, {$($unit:ident, $factor:literal, $($unit_symbol:literal)?),*}),+) => {
        use paste::paste;
        pub const UNIT_NAMES: &[($dimension, &str, f64)] = &[
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
            pub type $quantity_name = $quantity<f64, $const>;
            paste!{
                pub type [<Vec $quantity_name>] = $quantity<glam::DVec2, $const>;
                pub type [<DVec2 $quantity_name>] = $quantity<glam::DVec2, $const>;
                pub type [<DVec3 $quantity_name>] = $quantity<glam::DVec3, $const>;
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
