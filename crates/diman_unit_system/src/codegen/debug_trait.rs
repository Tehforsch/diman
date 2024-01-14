use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::CallerType,
    dimension_math::BaseDimensions,
    types::{base_dimension::BaseDimension, Unit},
};

use super::Codegen;

impl Codegen {
    pub fn runtime_unit_storage<'a>(&self, units: impl Iterator<Item = &'a Unit>) -> TokenStream {
        let runtime_unit_storage = match self.caller_type {
            CallerType::Internal => quote! { diman_lib::runtime_unit_storage::RuntimeUnitStorage },
            CallerType::External => {
                quote! { ::diman::internal::runtime_unit_storage::RuntimeUnitStorage }
            }
        };
        let runtime_unit = match self.caller_type {
            CallerType::Internal => quote! { diman_lib::runtime_unit_storage::RuntimeUnit },
            CallerType::External => quote! { ::diman::internal::runtime_unit_storage::RuntimeUnit },
        };
        let units: TokenStream = units
            .filter_map(|unit| {
                let dim = self.get_dimension_expr(&unit.dimensions);
                let magnitude = unit.magnitude;
                let symbol = &unit.symbol.as_ref()?.0.to_string();
                Some(quote! {
                    #runtime_unit::new(
                         #symbol,
                         #dim,
                         #magnitude,
                    ),
                })
            })
            .collect();
        quote! {
            let units_array = &[#units];
            let units = #runtime_unit_storage::new(units_array);
        }
    }

    pub fn gen_debug_trait_impl(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let units_storage =
            self.runtime_unit_storage(self.defs.units.iter().filter(|unit| unit.magnitude == 1.0));
        let get_base_dimension_symbols = self
            .defs
            .base_dimensions
            .iter()
            .map(|base_dim| self.get_base_dimension_symbol(base_dim))
            .collect::<TokenStream>();
        quote! {
            impl<const D: #dimension_type, S: core::fmt::Display> core::fmt::Debug for #quantity_type<S, D> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    #units_storage
                    self.0.fmt(f)?;
                    if let Some(symbol) = units.get_first_symbol(D) {
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
        let base_dim = &base_dim.0;
        quote! {
            if D.#base_dim == Exponent::one() {
                write!(f, " {}", units.get_first_symbol(#dim).unwrap())?;
            }
            else if D.#base_dim != Exponent::zero() {
                write!(f, " {}^{}", units.get_first_symbol(#dim).unwrap(), D.#base_dim)?;
            }
        }
    }
}
