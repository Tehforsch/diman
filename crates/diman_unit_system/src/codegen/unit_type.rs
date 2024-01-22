use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use super::Codegen;

impl Codegen {
    pub fn gen_unit_type(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let trait_impls = self.gen_unit_trait_impls();
        let storage_type_impls = self.gen_unit_trait_impls_for_storage_types();
        quote! {
            pub struct Unit<const D: #dimension_type, const F: Magnitude>;
            #trait_impls
            #storage_type_impls
        }
    }

    fn gen_unit_trait_impls(&self) -> TokenStream {
        quote! {
            use core::ops::{Mul, Div};
            // The following would be possible if
            // Unit::mul / Unit::div could be made const.
            // // Unit * Unit
            // impl<const DL: Dimension, const DR: Dimension, const FL: Magnitude, const FR: Magnitude>
            //     Mul<Unit<DR, FR>> for Unit<DL, FL>
            // where
            //     Unit<{ DL.add(DR) }, { FL.mul(FR) }>:,
            // {
            //     type Output = Unit<{ DL.add(DR) }, { FL.mul(FR) }>;
            //     fn mul(self, _: Unit<DR, FR>) -> Self::Output {
            //         Unit
            //     }
            // }

            // // Unit / Unit
            // impl<const DL: Dimension, const DR: Dimension, const FL: Magnitude, const FR: Magnitude>
            //     Div<Unit<DR, FR>> for Unit<DL, FL>
            // where
            //     Unit<{ DL.sub(DR) }, { FL.div(FR) }>:,
            // {
            //     type Output = Unit<{ DL.sub(DR) }, { FL.div(FR) }>;
            //     fn div(self, _: Unit<DR, FR>) -> Self::Output {
            //         Unit
            //     }
            // }

            // Unit * Quantity<S>
            impl<const DL: Dimension, const DR: Dimension, const FL: Magnitude, S> Mul<Quantity<S, DR>>
                for Unit<DL, FL>
            where
                S: Mul<Magnitude, Output = S>,
                Quantity<(), { DL.add(DR) }>:,
            {
                type Output = Quantity<S, { DL.add(DR) }>;
                fn mul(self, x: Quantity<S, DR>) -> Self::Output {
                    Quantity(x.value_unchecked() * FL)
                }
            }

            // Quantity<S> * Unit
            impl<const DL: Dimension, const DR: Dimension, const FR: Magnitude, S> Mul<Unit<DR, FR>>
                for Quantity<S, DL>
            where
                S: Mul<Magnitude, Output = S>,
                Quantity<(), { DL.add(DR) }>:,
            {
                type Output = Quantity<S, { DL.add(DR) }>;
                fn mul(self, _: Unit<DR, FR>) -> Self::Output {
                    Quantity(self.value_unchecked() * FR)
                }
            }

            // Unit / Quantity<S>
            impl<const DL: Dimension, const DR: Dimension, const FL: Magnitude, S> Div<Quantity<S, DR>>
                for Unit<DL, FL>
            where
                S: Div<Magnitude, Output = S>,
                Quantity<(), { DL.sub(DR) }>:,
            {
                type Output = Quantity<S, { DL.sub(DR) }>;
                fn div(self, x: Quantity<S, DR>) -> Self::Output {
                    Quantity(x.value_unchecked() / FL)
                }
            }

            // Quantity<S> / Unit
            impl<const DL: Dimension, const DR: Dimension, const FR: Magnitude, S> Div<Unit<DR, FR>>
                for Quantity<S, DL>
            where
                S: Div<Magnitude, Output = S>,
                Quantity<(), { DL.sub(DR) }>:,
            {
                type Output = Quantity<S, { DL.sub(DR) }>;
                fn div(self, _: Unit<DR, FR>) -> Self::Output {
                    Quantity(self.value_unchecked() / FR)
                }
            }

            impl<const D: Dimension, const F: Magnitude> Unit<D, F> {
                pub fn new<S>(self, val: S) -> Quantity<S, D>
                where
                    S: Mul<Magnitude, Output = S>,
                {
                    Quantity(val * F)
                }
            }
        }
    }

    fn gen_unit_trait_impls_for_storage_types(&self) -> TokenStream {
        self.storage_types()
            .map(|ty| {
                let name = &ty.name();
                let conversion_method = &ty.base_storage().conversion_method;
                self.gen_unit_numeric_traits_impls_for_type(name, conversion_method)
            })
            .collect()
    }

    fn gen_unit_numeric_traits_impls_for_type(
        &self,
        name: &Type,
        conversion_to_float: &TokenStream,
    ) -> TokenStream {
        let into = quote! {
            F.#conversion_to_float()
        };
        let res = quote! {
            // X * Unit
            impl<const D: Dimension, const F: Magnitude> Mul<Unit<D, F>> for #name {
                type Output = Quantity<#name, D>;
                fn mul(self, _: Unit<D, F>) -> Self::Output {
                    Quantity(self * #into)
                }
            }

            // X / Unit
            impl<const D: Dimension, const F: Magnitude> Div<Unit<D, F>> for #name {
                type Output = Quantity<#name, D>;
                fn div(self, _: Unit<D, F>) -> Self::Output {
                    Quantity(self / #into)
                }
            }

            // Unit * X
            impl<const D: Dimension, const F: Magnitude> Mul<#name> for Unit<D, F> {
                type Output = Quantity<#name, D>;
                fn mul(self, f: #name) -> Self::Output {
                    Quantity(#into * f)
                }
            }

            // Unit / X
            impl<const D: Dimension, const F: Magnitude> Div<#name> for Unit<D, F> {
                type Output = Quantity<#name, D>;
                fn div(self, f: #name) -> Self::Output {
                    Quantity(#into / f)
                }
            }
        };
        res
    }
}
