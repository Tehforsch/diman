use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::Codegen;
use crate::types::{Constant, Unit};
use diman_lib::magnitude::Magnitude;

impl Codegen {
    pub fn gen_units_and_constants(&self) -> TokenStream {
        let def_unit_type = self.gen_unit_type();
        let units: TokenStream = self
            .defs
            .units
            .iter()
            .map(|unit| {
                let unit = self.gen_unit_def(unit);
                quote! {
                    #unit
                }
            })
            .collect();
        let constants: TokenStream = self
            .defs
            .constants
            .iter()
            .map(|unit| {
                let constant = self.gen_constant_def(unit);
                quote! {
                    #constant
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
            #[allow(unused)]
            pub use unit_type::Unit;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub mod units {
                use super::Magnitude;
                use super::Unit;
                use super::Dimension;
                use super::Exponent;
                #units
            }
            #[allow(unused)]
            pub mod constants {
                use super::Magnitude;
                use super::Unit;
                use super::Dimension;
                use super::Exponent;
                #constants
            }
        }
    }

    fn gen_unit_def(&self, unit: &Unit) -> TokenStream {
        let dimension = self.get_dimension_expr(&unit.dimensions);
        let name = &unit.name;
        let magnitude = self.get_magnitude_expr(unit.magnitude);
        let span = self.defs.dimension_type.span();
        quote_spanned! {span=>
            pub const #name: Unit<{ #dimension }, { #magnitude }> = Unit;
        }
    }

    fn gen_constant_def(&self, constant: &Constant) -> TokenStream {
        let dimension = self.get_dimension_expr(&constant.dimensions);
        let name = &constant.name;
        let magnitude = self.get_magnitude_expr(constant.magnitude);
        let span = self.defs.dimension_type.span();
        quote_spanned! {span=>
            pub const #name: Unit<{ #dimension }, { #magnitude }> = Unit;
        }
    }

    pub fn get_magnitude_expr(&self, magnitude: Magnitude) -> TokenStream {
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
