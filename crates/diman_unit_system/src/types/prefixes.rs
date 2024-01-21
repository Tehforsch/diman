use diman_lib::magnitude::Magnitude;

macro_rules! make_prefix_enum {
    ($enum_name: ident, $(($variant_name: ident, $lowercase_name: ident, $name: literal, $short: literal, $factor: literal)),*) => {
        #[derive(Clone, Copy, PartialEq)]
        pub enum $enum_name {
            $(
                $variant_name,
            )*
        }

        impl $enum_name {
            pub fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name => $name,
                    )*
                }
            }

            pub fn short(self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name => $short,
                    )*
                }
            }

            pub fn factor(self) -> Magnitude {
                match self {
                    $(
                        Self::$variant_name => Magnitude::from_f64($factor),
                    )*
                }
            }
        }

        impl ::syn::parse::Parse for $enum_name {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                mod kws {
                    $(
                        syn::custom_keyword!($lowercase_name);
                    )*
                }
                let lookahead = input.lookahead1();
                $(
                    if lookahead.peek(kws::$lowercase_name) {
                        let _: kws::$lowercase_name = input.parse()?;
                        return Ok(Self::$variant_name);
                    }
                )*
                Err(lookahead.error())
            }
        }
    }
}

make_prefix_enum! {
    Prefix,
    (Exa, exa, "exa",  "E",  1e18),
    (Peta, peta, "peta",  "P",  1e15),
    (Tera, tera, "tera",  "T",  1e12),
    (Giga, giga, "giga",  "G",  1e9),
    (Mega, mega, "mega",  "M",  1e6),
    (Kilo, kilo, "kilo",  "k",  1e3),
    (Hecto, hecto, "hecto",  "h",  1e2),
    (Deca, deca, "deca",  "da",  1e1),
    (Deci, deci, "deci",  "d",  1e-1),
    (Centi, centi, "centi",  "c",  1e-2),
    (Milli, milli, "milli",  "m",  1e-3),
    (Micro, micro, "micro",  "μ",  1e-6),
    (Nano, nano, "nano",  "n",  1e-9),
    (Pico, pico, "pico",  "p",  1e-12),
    (Femto, femto, "femto",  "f",  1e-15),
    (Atto, atto, "atto",  "a",  1e-18)
}

pub struct MetricPrefixes {
    pub skip: Vec<Prefix>,
}

impl From<MetricPrefixes> for Vec<Prefix> {
    fn from(def: MetricPrefixes) -> Self {
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
        .into_iter()
        .filter(|prefix| !def.skip.contains(prefix))
        .collect()
    }
}

pub struct ExplicitPrefixes(pub Vec<Prefix>);
