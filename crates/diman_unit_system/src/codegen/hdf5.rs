use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    storage_types::{FloatType, VectorType},
    types::Defs,
};

use super::join;

impl Defs {
    pub fn gen_hdf5_impl(&self) -> TokenStream {
        join([self.hdf5_floats_impl(), self.hdf5_vectors_impl()])
    }

    fn hdf5_floats_impl(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| self.hdf5_float_impl(float_type))
            .collect()
    }

    fn hdf5_float_impl(&self, float_type: &FloatType) -> TokenStream {
        let float_type_name = &float_type.name;
        let hdf5_type = &float_type.hdf5_type;
        let Defs {
            dimension_type,
            quantity_type,
            ..
        } = self;
        quote! {
            unsafe impl<const D: #dimension_type> hdf5::H5Type for #quantity_type<#float_type_name, D> {
                fn type_descriptor() -> hdf5::types::TypeDescriptor {
                    hdf5::types::TypeDescriptor::Float(#hdf5_type)
                }
            }
        }
    }

    fn hdf5_vectors_impl(&self) -> TokenStream {
        self.vector_types()
            .iter()
            .map(|vector_type| self.hdf5_vector_impl(vector_type))
            .collect()
    }

    fn hdf5_vector_impl(&self, vector_type: &VectorType) -> TokenStream {
        let vector_type_name = &vector_type.name;
        let hdf5_type = &vector_type.float_type.hdf5_type;
        let num_dims = vector_type.num_dims;
        let Defs {
            dimension_type,
            quantity_type,
            ..
        } = self;
        quote! {
            unsafe impl<const D: #dimension_type> hdf5::H5Type for #quantity_type<#vector_type_name, D> {
                fn type_descriptor() -> hdf5::types::TypeDescriptor {
                    hdf5::types::TypeDescriptor::FixedArray(
                        Box::new(hdf5::types::TypeDescriptor::Float(#hdf5_type)),
                        #num_dims,
                    )
                }
            }
        }
    }
}
