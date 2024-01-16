use proc_macro2::TokenStream;
use quote::quote;

use super::Codegen;
use crate::types::Unit;
use diman_lib::magnitude::Magnitude;

impl Codegen {
    pub fn gen_units(&self) -> TokenStream {
        let def_unit_type = self.gen_unit_type();
        let units: TokenStream = self
            .defs
            .units
            .iter()
            .map(|unit| {
                let unit = self.unit_const(unit);
                quote! {
                    #unit
                }
            })
            .collect();
        let path_prefix = self.caller_type.path_prefix();
        quote! {
            pub use #path_prefix::magnitude::Magnitude;
            mod unit_type {
                use super::Dimension;
                use super::Magnitude;
                use super::Quantity;
                #def_unit_type
            }
            pub use unit_type::Unit;
            #[allow(non_upper_case_globals)]
            pub mod units {
                use super::Magnitude;
                use super::Unit;
                use super::Dimension;
                #units
            }
        }
    }

    fn unit_const(&self, unit: &Unit) -> TokenStream {
        let dimension = self.get_dimension_expr(&unit.dimensions);
        let name = &unit.name;
        let magnitude = self.get_magnitude_expr(unit.magnitude);
        quote! {
            pub const #name: Unit<{ #dimension }, { #magnitude }> = Unit;
        }
    }

    fn get_magnitude_expr(&self, magnitude: Magnitude) -> TokenStream {
        let (mantissa, exponent, sign) = (magnitude.mantissa, magnitude.exponent, magnitude.sign);
        quote! {
            Magnitude {
                mantissa: #mantissa,
                exponent: #exponent,
                sign: #sign,
            }
        }
    }
}
