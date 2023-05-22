use proc_macro2::TokenStream;
use quote::quote;

use crate::{storage_types::VectorType, types::Defs};

impl Defs {
    pub fn vector_methods(&self) -> TokenStream {
        self.vector_types()
            .iter()
            .map(|vector_type| self.impl_vector_methods(vector_type))
            .collect()
    }

    pub fn impl_vector_methods(&self, vector_type: &VectorType) -> TokenStream {
        let Defs {
            dimension_type,
            quantity_type,
            ..
        } = self;
        let VectorType {
            name: vector_type_name,
            float_type,
            num_dims,
            ..
        } = vector_type;
        let float_type = &float_type.name;
        let z_impl = if *num_dims == 3 {
            quote! {
                pub fn z(&self) -> #quantity_type<#float_type, D> {
                    #quantity_type(self.0.z)
                }
            }
        } else {
            quote! {}
        };
        let set_z_impl = if *num_dims == 3 {
            quote! {
                pub fn set_z(&mut self, new_z: #quantity_type<#float_type, D>) {
                    self.0.z = new_z.value_unchecked();
                }
            }
        } else {
            quote! {}
        };
        quote! {
            impl<const D: #dimension_type> #quantity_type<#vector_type_name, D> {
                pub fn x(&self) -> #quantity_type<#float_type, D> {
                    #quantity_type(self.0.x)
                }

                pub fn y(&self) -> #quantity_type<#float_type, D> {
                    #quantity_type(self.0.y)
                }

                #z_impl

                pub fn set_x(&mut self, new_x: #quantity_type<#float_type, D>) {
                    self.0.x = new_x.value_unchecked();
                }

                pub fn set_y(&mut self, new_y: #quantity_type<#float_type, D>) {
                    self.0.y = new_y.value_unchecked();
                }

                #set_z_impl

                pub fn min(self, rhs: Self) -> Self {
                    Self(self.0.min(rhs.0))
                }

                pub fn max(self, rhs: Self) -> Self {
                    Self(self.0.max(rhs.0))
                }

                pub fn length(&self) -> #quantity_type<#float_type, D> {
                    #quantity_type::<#float_type, D>(self.0.length())
                }

                pub fn distance(&self, other: &Self) -> #quantity_type<#float_type, D> {
                    #quantity_type::<#float_type, D>(self.0.distance(other.0))
                }

                pub fn distance_squared(
                    &self,
                    other: &Self,
                ) -> #quantity_type<#float_type, { D.dimension_powi(2) }>
                where
                    #quantity_type<#float_type, { D.dimension_powi(2) }>:,
                {
                    #quantity_type::<#float_type, { D.dimension_powi(2) }>(self.0.distance_squared(other.0))
                }

                pub fn normalize(&self) -> #quantity_type<#vector_type_name, { #dimension_type::none() }> {
                    #quantity_type::<#vector_type_name, { #dimension_type::none() }>(self.0.normalize())
                }

                pub fn dot<const DR: Dimension>(
                    self,
                    rhs: Quantity<#vector_type_name, DR>,
                ) -> #quantity_type<#float_type, { D.dimension_mul(DR) }> {
                    #quantity_type(self.0.dot(rhs.0))
                }
            }
        }
    }
}
