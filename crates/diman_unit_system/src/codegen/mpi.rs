use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    storage_types::{FloatType, VectorType},
    types::Defs,
};

use super::join;

impl Defs {
    pub fn gen_mpi_impl(&self) -> TokenStream {
        join([self.mpi_floats_impl(), self.mpi_vectors_impl()])
    }

    fn mpi_floats_impl(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| self.mpi_float_impl(float_type))
            .collect()
    }

    fn mpi_float_impl(&self, float_type: &FloatType) -> TokenStream {
        let float_type_name = &float_type.name;
        let mpi_type = &float_type.mpi_type;
        let Defs {
            dimension_type,
            quantity_type,
            ..
        } = self;
        quote! {
            unsafe impl<const D: #dimension_type> ::mpi::traits::Equivalence for #quantity_type<#float_type_name, D> {
                type Out = ::mpi::datatype::SystemDatatype;

                fn equivalent_datatype() -> Self::Out {
                    unsafe {
                        <::mpi::datatype::DatatypeRef as ::mpi::raw::FromRaw>::from_raw(#mpi_type)
                    }
                }
            }
        }
    }

    fn mpi_vectors_impl(&self) -> TokenStream {
        self.vector_types()
            .iter()
            .map(|vector_type| self.mpi_vector_impl(vector_type))
            .collect()
    }

    fn mpi_vector_impl(&self, vector_type: &VectorType) -> TokenStream {
        let vector_type_name = &vector_type.name;
        let float_type = &vector_type.float_type.name;
        let num_dims = vector_type.num_dims as i32;
        let Defs {
            dimension_type,
            quantity_type,
            ..
        } = self;
        quote! {
            unsafe impl<const D: #dimension_type> ::mpi::traits::Equivalence for #quantity_type<#vector_type_name, D> {
                type Out = ::mpi::datatype::DatatypeRef<'static>;

                fn equivalent_datatype() -> Self::Out {
                    static DATATYPE: ::once_cell::sync::Lazy<::mpi::datatype::UserDatatype> =
                        ::once_cell::sync::Lazy::new(|| {
                            ::mpi::datatype::UserDatatype::contiguous(
                                #num_dims,
                                &<#float_type>::equivalent_datatype(),
                            )
                        });
                    DATATYPE.as_ref()
                }
            }
        }
    }
}
