macro_rules! make_prefix_enum {
    ($enum_name: ident, $(($variant_name: ident, $name: literal, $short: literal, $factor: literal)),*) => {
        #[derive(Clone)]
        pub enum $enum_name {
            $(
                $variant_name,
            )*
        }

        impl $enum_name {
            fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name => $name,
                    )*
                }
            }

            fn short(self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name => $short,
                    )*
                }
            }

            fn factor(self) -> f64 {
                match self {
                    $(
                        Self::$variant_name => $factor,
                    )*
                }
            }
        }
    }
}

make_prefix_enum! {
    Prefix,
    (Exa, "exa",  "E",  1e18),
    (Peta, "peta",  "P",  1e15),
    (Tera, "tera",  "T",  1e12),
    (Giga, "giga",  "G",  1e9),
    (Mega, "mega",  "M",  1e6),
    (Kilo, "kilo",  "k",  1e3),
    (Hecto, "hecto",  "h",  1e2),
    (Deca, "deca",  "da",  1e1),
    (Deci, "deci",  "d",  1e-1),
    (Centi, "centi",  "c",  1e-2),
    (Milli, "milli",  "m",  1e-3),
    (Micro, "micro",  "μ",  1e-6),
    (Nano, "nano",  "n",  1e-9),
    (Pico, "pico",  "p",  1e-12),
    (Femto, "femto",  "f",  1e-15),
    (Atto, "atto",  "a",  1e-18)
}

pub struct MetricPrefixes;

impl From<MetricPrefixes> for Vec<Prefix> {
    fn from(_: MetricPrefixes) -> Self {
        vec![
            Prefix::Exa,
            Prefix::Peta,
            Prefix::Tera,
            Prefix::Giga,
            Prefix::Mega,
            Prefix::Kilo,
            Prefix::Hecto,
            Prefix::Deca,
            Prefix::Deci,
            Prefix::Centi,
            Prefix::Milli,
            Prefix::Micro,
            Prefix::Nano,
            Prefix::Pico,
            Prefix::Femto,
            Prefix::Atto,
        ]
    }
}
