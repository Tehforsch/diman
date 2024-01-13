use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    dimension_math::BaseDimensions,
    types::{base_dimension::BaseDimension, Unit},
};

use super::Codegen;

impl Codegen {
    pub fn units_array<'a>(&self, units: impl Iterator<Item = &'a Unit>) -> TokenStream {
        let units: TokenStream = units
            .filter_map(|unit| {
                let dim = self.get_dimension_expr(&unit.dimensions);
                let magnitude = unit.magnitude;
                let symbol = &unit.symbol.as_ref()?.0.to_string();
                Some(quote! {
                    (#dim, #symbol, #magnitude),
                })
            })
            .collect();
        quote! { [ #units ] }
    }

    pub fn gen_debug_trait_impl(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let units = self.units_array(self.defs.units.iter().filter(|unit| unit.magnitude == 1.0));
        let get_base_dimension_symbols = self
            .defs
            .base_dimensions
            .iter()
            .map(|base_dim| self.get_base_dimension_symbol(base_dim))
            .collect::<TokenStream>();
        quote! {
            fn get_symbol<const D: #dimension_type>() -> Option<&'static str> {
                let units: &[(#dimension_type, &str, f64)] = &#units;
                units
                    .iter()
                    .filter(|(d, name, _)|  d == &D )
                    .map(|(_, name, _)| name)
                    .next()
                    .copied()
            }

            impl<const D: #dimension_type, S: core::fmt::Display> core::fmt::Debug for #quantity_type<S, D> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    self.0.fmt(f)?;
                    if let Some(symbol) = get_symbol::<D>() {
                        write!(f, " {}", symbol)
                    }
                    else {
                        #get_base_dimension_symbols
                        Ok(())
                    }
                }
            }
        }
    }

    fn get_base_dimension_symbol(&self, base_dim: &BaseDimension) -> TokenStream {
        let dim = self.get_dimension_expr(&BaseDimensions::for_base_dimension(base_dim.clone()));
        // We know that symbols exist for base dimensions, so we can unwrap here.
        let base_dimension_type_zero = self.base_dimension_type_zero();
        let base_dimension_type_one = self.base_dimension_type_one();
        let base_dim = &base_dim.0;
        quote! {
            if D.#base_dim == #base_dimension_type_one {
                write!(f, " {}", get_symbol::< { #dim }>().unwrap())?;
            }
            else if D.#base_dim != #base_dimension_type_zero {
                write!(f, " {}^{}", get_symbol::< { #dim }>().unwrap(), D.#base_dim)?;
            }
        }
    }
}
