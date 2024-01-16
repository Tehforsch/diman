use proc_macro2::TokenStream;
use quote::quote;

use super::Codegen;

impl Codegen {
    pub fn gen_unit_type(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let trait_impls = self.gen_unit_trait_impls();
        quote! {
            pub struct Unit<const D: #dimension_type, const F: Magnitude>;
            #trait_impls
        }
    }

    fn gen_unit_trait_impls(&self) -> TokenStream {
        quote! {
            use core::ops::{Mul, Div};
            // Unit * Unit
            impl<const DL: Dimension, const DR: Dimension, const FL: Magnitude, const FR: Magnitude>
                Mul<Unit<DR, FR>> for Unit<DL, FL>
            where
                Unit<{ DL.add(DR) }, { FL.mul(FR) }>:,
            {
                type Output = Unit<{ DL.add(DR) }, { FL.mul(FR) }>;
                fn mul(self, _: Unit<DR, FR>) -> Self::Output {
                    Unit
                }
            }

            // Unit / Unit
            impl<const DL: Dimension, const DR: Dimension, const FL: Magnitude, const FR: Magnitude>
                Div<Unit<DR, FR>> for Unit<DL, FL>
            where
                Unit<{ DL.sub(DR) }, { FL.div(FR) }>:,
            {
                type Output = Unit<{ DL.sub(DR) }, { FL.div(FR) }>;
                fn div(self, _: Unit<DR, FR>) -> Self::Output {
                    Unit
                }
            }

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

            // f64 * Unit
            impl<const D: Dimension, const F: Magnitude> Mul<Unit<D, F>> for f64 {
                type Output = Quantity<f64, D>;
                fn mul(self, _: Unit<D, F>) -> Self::Output {
                    Quantity(self * F)
                }
            }

            // f64 / Unit
            impl<const D: Dimension, const F: Magnitude> Div<Unit<D, F>> for f64 {
                type Output = Quantity<f64, D>;
                fn div(self, _: Unit<D, F>) -> Self::Output {
                    Quantity(self / F)
                }
            }

            // Unit * f64
            impl<const D: Dimension, const F: Magnitude> Mul<f64> for Unit<D, F> {
                type Output = Quantity<f64, D>;
                fn mul(self, f: f64) -> Self::Output {
                    Quantity(F.as_f64() * f)
                }
            }

            // Unit / f64
            impl<const D: Dimension, const F: Magnitude> Div<f64> for Unit<D, F> {
                type Output = Quantity<f64, D>;
                fn div(self, f: f64) -> Self::Output {
                    Quantity(F.as_f64() / f)
                }
            }

            // f32 * Unit
            impl<const D: Dimension, const F: Magnitude> Mul<Unit<D, F>> for f32 {
                type Output = Quantity<f32, D>;
                fn mul(self, _: Unit<D, F>) -> Self::Output {
                    Quantity(self * F)
                }
            }

            // f32 / Unit
            impl<const D: Dimension, const F: Magnitude> Div<Unit<D, F>> for f32 {
                type Output = Quantity<f32, D>;
                fn div(self, _: Unit<D, F>) -> Self::Output {
                    Quantity(self / F)
                }
            }

            // Unit * f32
            impl<const D: Dimension, const F: Magnitude> Mul<f32> for Unit<D, F> {
                type Output = Quantity<f32, D>;
                fn mul(self, f: f32) -> Self::Output {
                    Quantity(F.as_f32() * f)
                }
            }

            // Unit / f32
            impl<const D: Dimension, const F: Magnitude> Div<f32> for Unit<D, F> {
                type Output = Quantity<f32, D>;
                fn div(self, f: f32) -> Self::Output {
                    Quantity(F.as_f32() / f)
                }
            }

            impl<const D: Dimension, const F: Magnitude> Unit<D, F> {
                pub fn new<S>(self, val: S) -> Quantity<S, D>
                where
                    S: Mul<f64, Output = S>,
                {
                    Quantity(val * F.as_f64())
                }
            }
        }
    }
}
