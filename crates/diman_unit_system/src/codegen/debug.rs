use proc_macro2::TokenStream;
use quote::quote;

use crate::types::{Defs, Unit};

impl Defs {
    pub fn units_array<'a>(&self, units: impl Iterator<Item = &'a Unit>) -> TokenStream {
        let units: TokenStream = units
            .filter_map(|unit| {
                let dim = self.get_dimension_expr(&unit.dimensions);
                let factor = unit.factor;
                let symbol = &unit.symbol.as_ref()?.0.to_string();
                Some(quote! {
                    (#dim, #symbol, #factor),
                })
            })
            .collect();
        quote! { [ #units ] }
    }

    pub fn debug_trait(&self) -> TokenStream {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        let units = self.units_array(self.units.iter().filter(|unit| unit.factor == 1.0));
        quote! {
            impl<const D: #dimension_type, S: diman::DebugStorageType + core::fmt::Display> core::fmt::Debug for #quantity_type<S, D> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    let closeness = |value: f64, unit_factor: f64| {
                        let (mantissa, exponent, _sign) = (value / unit_factor).integer_decode();
                        (exponent, mantissa)
                    };
                    let val = self.0.representative_value();
                    let units: &[(#dimension_type, _, _)] = &#units;
                    let (unit_name, unit_value) = units
                        .iter()
                        .filter(|(d, _, _)| d == &D)
                        .min_by(|(_, _, x), (_, _, y)| {
                            closeness(val, *x)
                                .partial_cmp(&closeness(val, *y))
                                .unwrap_or(core::cmp::Ordering::Equal)
                        })
                        .map(|(_, name, val)| (name, val))
                        .unwrap_or((&"unknown unit", &1.0));
                    (self.0.div_f64(*unit_value))
                        .fmt(f)
                        .and_then(|_| write!(f, " {}", unit_name))
                }
            }
        }
    }
}
