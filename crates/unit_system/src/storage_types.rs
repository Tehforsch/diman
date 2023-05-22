use proc_macro2::TokenStream;
use quote::quote;

use crate::types::Defs;

pub struct VectorType {
    pub name: TokenStream,
    pub module_name: TokenStream,
    pub float_type: TokenStream,
    pub num_dims: usize,
}

pub struct FloatType {
    pub name: TokenStream,
    pub module_name: TokenStream,
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
                name: quote! {::glam::Vec2},
                module_name: quote! { vec2 },
                float_type: quote! { f32 },
                num_dims: 2,
            },
            #[cfg(feature = "glam-dvec2")]
            VectorType {
                name: quote! {::glam::DVec2},
                module_name: quote! { dvec2 },
                float_type: quote! { f64 },
                num_dims: 2,
            },
            #[cfg(feature = "glam-vec3")]
            VectorType {
                name: quote! {::glam::Vec3},
                module_name: quote! { vec3 },
                float_type: quote! { f32 },
                num_dims: 3,
            },
            #[cfg(feature = "glam-dvec3")]
            VectorType {
                name: quote! {::glam::DVec3},
                module_name: quote! { dvec3 },
                float_type: quote! { f64 },
                num_dims: 3,
            },
        ]
    }

    pub fn float_types(&self) -> Vec<FloatType> {
        vec![
            #[cfg(feature = "f32")]
            FloatType {
                name: quote! { f32 },
                module_name: quote! { f32 },
            },
            #[cfg(feature = "f64")]
            FloatType {
                name: quote! { f64 },
                module_name: quote! { f64 },
            },
        ]
    }
}
