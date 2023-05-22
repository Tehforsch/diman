use proc_macro2::TokenStream;

use crate::types::Defs;

pub struct VectorType {
    pub name: TokenStream,
    pub module_name: TokenStream,
    pub float_type: FloatType,
    pub num_dims: usize,
}

pub struct FloatType {
    pub name: TokenStream,
    pub module_name: TokenStream,
    #[cfg(feature = "mpi")]
    pub mpi_type: TokenStream,
}

impl Defs {
    pub fn storage_type_names(&self) -> Vec<TokenStream> {
        self.float_types()
            .into_iter()
            .map(|x| x.name)
            .chain(self.vector_types().into_iter().map(|x| x.name))
            .collect()
    }

    pub fn vector_types(&self) -> Vec<VectorType> {
        vec![
            #[cfg(feature = "glam-vec2")]
            VectorType {
                name: quote::quote! {::glam::Vec2},
                module_name: quote::quote! { vec2 },
                float_type: self.f32_type(),
                num_dims: 2,
            },
            #[cfg(feature = "glam-dvec2")]
            VectorType {
                name: quote::quote! {::glam::DVec2},
                module_name: quote::quote! { dvec2 },
                float_type: self.f64_type(),
                num_dims: 2,
            },
            #[cfg(feature = "glam-vec3")]
            VectorType {
                name: quote::quote! {::glam::Vec3},
                module_name: quote::quote! { vec3 },
                float_type: self.f32_type(),
                num_dims: 3,
            },
            #[cfg(feature = "glam-dvec3")]
            VectorType {
                name: quote::quote! {::glam::DVec3},
                module_name: quote::quote! { dvec3 },
                float_type: self.f64_type(),
                num_dims: 3,
            },
        ]
    }

    fn f32_type(&self) -> FloatType {
        FloatType {
            name: quote::quote! { f32 },
            module_name: quote::quote! { f32 },
            #[cfg(feature = "mpi")]
            mpi_type: quote::quote! { ::mpi::ffi::RSMPI_FLOAT },
        }
    }

    fn f64_type(&self) -> FloatType {
        FloatType {
            name: quote::quote! { f64 },
            module_name: quote::quote! { f64 },
            #[cfg(feature = "mpi")]
            mpi_type: quote::quote! { ::mpi::ffi::RSMPI_DOUBLE },
        }
    }

    pub fn float_types(&self) -> Vec<FloatType> {
        vec![
            #[cfg(feature = "f32")]
            self.f32_type(),
            #[cfg(feature = "f64")]
            self.f64_type(),
        ]
    }
}
