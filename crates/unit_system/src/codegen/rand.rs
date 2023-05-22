use quote::quote;

use proc_macro2::TokenStream;

use crate::{types::Defs, storage_types::FloatType};

impl Defs {
    pub fn rand_impl(&self) -> TokenStream {
        let float_impls: TokenStream = self.float_types().iter().map(|float_type| self.rand_impl_float(float_type)).collect();
        let Defs {
            dimension_type,
            ..
        } = self;
        quote! {
            use ::rand::distributions::uniform::SampleBorrow;
            use ::rand::distributions::uniform::SampleUniform;
            use ::rand::distributions::uniform::UniformFloat;
            use ::rand::distributions::uniform::UniformSampler;
            use ::rand::prelude::*;

            #[derive(Clone, Copy, Debug)]
            pub struct UniformQuantity<S, const D: #dimension_type>(UniformFloat<S>);

            #float_impls
        }
    }

    fn rand_impl_float(&self, float_type: &FloatType) -> TokenStream {
        let Defs {
            dimension_type,
            quantity_type,
            ..
        } = self;
        let float_type = &float_type.name;
        quote! { 
            impl<const D: #dimension_type> UniformSampler for UniformQuantity<#float_type, D> {
                type X = #quantity_type::<#float_type, D>;
                fn new<B1, B2>(low: B1, high: B2) -> Self
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    UniformQuantity::<#float_type, D>(UniformFloat::<#float_type>::new(
                        low.borrow().0,
                        high.borrow().0,
                    ))
                }
                fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    UniformQuantity::<#float_type, D>(UniformFloat::<#float_type>::new_inclusive(
                        low.borrow().0,
                        high.borrow().0,
                    ))
                }

                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                    #quantity_type::<#float_type, D>(self.0.sample(rng))
                }
            }

            impl<const D: #dimension_type> SampleUniform for #quantity_type<#float_type, D> {
                type Sampler = UniformQuantity<#float_type, D>;
            }
        }
    }
}
