use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::types::Defs;

pub struct VectorType {
    pub name: Type,
    pub module_name: TokenStream,
    pub float_type: FloatType,
    pub num_dims: usize,
}

pub struct FloatType {
    pub name: Type,
    pub module_name: TokenStream,
    #[cfg(feature = "mpi")]
    pub mpi_type: TokenStream,
    #[cfg(feature = "hdf5")]
    pub hdf5_type: TokenStream,
    #[cfg(feature = "serde")]
    pub serialize_method: TokenStream,
}

pub trait StorageType {
    fn name(&self) -> &Type;
    fn base_storage(&self) -> &Type;
}

impl StorageType for VectorType {
    fn name(&self) -> &Type {
        &self.name
    }

    fn base_storage(&self) -> &Type {
        &self.float_type.name
    }
}

impl StorageType for FloatType {
    fn name(&self) -> &Type {
        &self.name
    }

    fn base_storage(&self) -> &Type {
        &self.name
    }
}

impl Defs {
    pub fn storage_type_names(&self) -> Vec<Type> {
        self.float_types()
            .into_iter()
            .map(|x| x.name)
            .chain(self.vector_types().into_iter().map(|x| x.name))
            .collect()
    }

    pub fn vector_types(&self) -> Vec<VectorType> {
        // I don't know if this is really the way to construct types
        let _vec2: Type = syn::parse2(quote! { ::glam::Vec2 }).unwrap();
        let _dvec2: Type = syn::parse2(quote! { ::glam::DVec2 }).unwrap();
        let _vec3: Type = syn::parse2(quote! { ::glam::Vec3 }).unwrap();
        let _dvec3: Type = syn::parse2(quote! { ::glam::DVec3 }).unwrap();
        vec![
            #[cfg(feature = "glam-vec2")]
            VectorType {
                name: _vec2,
                module_name: quote! { vec2 },
                float_type: self.f32_type(),
                num_dims: 2,
            },
            #[cfg(feature = "glam-dvec2")]
            VectorType {
                name: _dvec2,
                module_name: quote! { dvec2 },
                float_type: self.f64_type(),
                num_dims: 2,
            },
            #[cfg(feature = "glam-vec3")]
            VectorType {
                name: _vec3,
                module_name: quote! { vec3 },
                float_type: self.f32_type(),
                num_dims: 3,
            },
            #[cfg(feature = "glam-dvec3")]
            VectorType {
                name: _dvec3,
                module_name: quote! { dvec3 },
                float_type: self.f64_type(),
                num_dims: 3,
            },
        ]
    }

    #[cfg(feature = "f32")]
    fn f32_type(&self) -> FloatType {
        let f32_ty: Type = syn::parse2(quote! { f32 }).unwrap();
        FloatType {
            name: f32_ty,
            module_name: quote! { f32 },
            #[cfg(feature = "mpi")]
            mpi_type: quote! { ::mpi::ffi::RSMPI_FLOAT },
            #[cfg(feature = "hdf5")]
            hdf5_type: quote! { hdf5::types::FloatSize::U4 },
            #[cfg(feature = "serde")]
            serialize_method: quote! { serialize_f32 },
        }
    }

    #[cfg(feature = "f64")]
    fn f64_type(&self) -> FloatType {
        let f64_ty: Type = syn::parse2(quote! { f64 }).unwrap();
        FloatType {
            name: f64_ty,
            module_name: quote! { f64 },
            #[cfg(feature = "mpi")]
            mpi_type: quote! { ::mpi::ffi::RSMPI_DOUBLE },
            #[cfg(feature = "hdf5")]
            hdf5_type: quote! { hdf5::types::FloatSize::U8 },
            #[cfg(feature = "serde")]
            serialize_method: quote! { serialize_f64 },
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
