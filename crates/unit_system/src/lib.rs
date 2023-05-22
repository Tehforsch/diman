mod types;
mod parse;
mod gen_traits;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::*;
use types::{Defs, QuantityEntry};

#[proc_macro]
pub fn unit_system_2(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as Defs);
    let stream: TokenStream = [
        defs.type_definition(),
        defs.type_functions(),
        defs.unit_array(),
        defs.quantity_definitions(),
        defs.unit_definitions(),
        defs.qproduct_trait(),
        defs.numeric_traits(),
    ].into_iter().collect();

    stream.into()
}

impl Defs {
    fn get_dimension_definition(&self, q: &QuantityEntry) -> TokenStream {
        let dimension_type = &self.dimension_type;
        let field_updates: TokenStream = q
            .dimensions_def
            .fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let value = &field.value.val;
                quote! { #ident: #value, }
            })
            .collect();
        quote! {
            #dimension_type {
                #field_updates
                ..#dimension_type::none()
            }
        }
    }

    pub(crate) fn type_definition(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
            #[repr(transparent)]
            pub struct #quantity_type<S: 'static, const D: #dimension_type>(pub(crate) S);
        }
    }

    pub(crate) fn type_functions(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            impl<S> #quantity_type<S, { #dimension_type::none() }> {
                /// Get the value of a dimensionless quantity
                pub fn value(self) -> S {
                    self.0
                }

                /// Get a reference to the value of a dimensionless quantity
                pub fn value_ref(&self) -> &S {
                    &self.0
                }
            }

            impl<S, const D: #dimension_type> #quantity_type<S, D> {
                /// Return the value of a quantity, regardless of whether
                /// it is dimensionless or not. Use this carefully, since the
                /// result depends on the underlying base units
                pub fn value_unchecked(self) -> S {
                    self.0
                }

                /// Create a new quantity for the dimension with a given value.
                /// Use carefully, since the constructed quantity depends on the
                /// used base units.
                pub const fn new_unchecked(s: S) -> Self {
                    Self(s)
                }
            }
        }
    }

    pub(crate) fn quantity_definitions(&self) -> TokenStream {
        self
            .quantities
            .iter()
            .map(|quantity| {
                let dimension = self.get_dimension_definition(&quantity);
                let quantity_type = &self.quantity_type;
                let quantity_name = &quantity.name;
                let _vec2_ident = format_ident!("Vec2{}", quantity_name);
                let _vec3_ident = format_ident!("Vec2{}", quantity_name);
                let _vec_ident = format_ident!("Vec{}", quantity_name);
                quote! {
                    #[cfg(feature = "default-f64")]
                    pub type #quantity_name = #quantity_type::<f64, { #dimension }>;
                    #[cfg(feature = "default-f32")]
                    pub type #quantity_name = #quantity_type::<f32, { #dimension }>;

                    #[cfg(all(
                        feature = "glam",
                        any(feature = "default-f32", feature = "default-f64")
                    ))]
                    pub type #_vec2_ident = #quantity_type<MVec2, { #dimension }>;
                    #[cfg(all(
                        feature = "glam",
                        any(feature = "default-f32", feature = "default-f64")
                    ))]
                    pub type #_vec3_ident = #quantity_type<MVec3, { #dimension }>;
                    #[cfg(all(
                        feature = "glam",
                        feature = "default-f32")
                    )]
                    pub type #_vec_ident = #quantity_type<MVec, { #dimension }>;
                    #[cfg(all(
                        feature = "glam",
                        feature = "default-f64")
                    )]
                    pub type #_vec_ident = #quantity_type<MVec, { #dimension }>;
                }
        }).collect()
    }

    pub(crate) fn unit_definitions(&self) -> TokenStream {
    self
        .quantities
        .iter()
        .map(|quantity| {
            let dimension = self.get_dimension_definition(&quantity);
            let quantity_type = &self.quantity_type;
            let quantity_name = &quantity.name;
            let _vec2_ident = format_ident!("Vec2{}", quantity_name);
            let _vec3_ident = format_ident!("Vec2{}", quantity_name);
            let _vec_ident = format_ident!("Vec{}", quantity_name);

            quantity
                .units_def
                .units
                .iter()
                .map(|unit| {
                    let unit_name = &unit.name;
                    let factor = &unit.factor;
                    let conversion_method_name = format_ident!("in_{}", unit_name);
                    quote! {
                        impl #quantity_type::<f64, {#dimension}> {
                            pub fn #unit_name(v: f64) -> #quantity_type<f64, { #dimension }> {
                                #quantity_type::<f64, { #dimension }>(v * #factor)
                            }

                        }
                        impl #quantity_type::<f32, {#dimension}> {
                            pub fn #unit_name(v: f32) -> #quantity_type<f32, { #dimension }> {
                                #quantity_type::<f32, { #dimension }>(v * (#factor as f32))
                            }
                        }
                        impl<S> #quantity_type<S, {#dimension}> where S: std::ops::Div<f64, Output = S> {
                            pub fn #conversion_method_name(self) -> S {
                                self.0 / #factor
                            }
                        }

                        #[cfg(feature = "glam")]
                        impl #quantity_type<::glam::Vec2, {#dimension}> {
                            pub fn #unit_name(x: f32, y: f32) -> #quantity_type<::glam::Vec2, {#dimension}> {
                                #quantity_type::<::glam::Vec2, {#dimension}>(::glam::Vec2::new(x, y) * #factor)
                            }
                        }
                        #[cfg(feature = "glam")]
                        impl #quantity_type<::glam::Vec3, {#dimension}> {
                            pub fn #unit_name(x: f32, y: f32, z: f32) -> #quantity_type<::glam::Vec3, {#dimension}> {
                                #quantity_type::<::glam::Vec3, {#dimension}>(::glam::Vec3::new(x, y, z) * #factor)
                            }
                        }
                        #[cfg(feature = "glam")]
                        impl #quantity_type<::glam::DVec2, {#dimension}> {
                            pub fn #unit_name(x: f64, y: f64) -> #quantity_type<::glam::DVec2, {#dimension}> {
                                #quantity_type::<::glam::DVec2, {#dimension}>(::glam::DVec2::new(x, y) * #factor)
                            }
                        }
                        #[cfg(feature = "glam")]
                        impl #quantity_type<::glam::DVec3, {#dimension}> {
                            pub fn #unit_name(x: f64, y: f64, z: f64) -> #quantity_type<::glam::DVec3, {#dimension}> {
                                #quantity_type::<::glam::DVec3, {#dimension}>(::glam::DVec3::new(x, y, z) * #factor)
                            }
                        }
                    }
                }).collect::<TokenStream>()
        }).collect()
    }

    // Only temporary to make the transition less menacing
    pub(crate) fn unit_array(&self) -> TokenStream {
        let dimension_type = &self.dimension_type;
        let unit_names_type = &self.unit_names_type;
        let unit_names_array_gen: TokenStream = self
            .quantities
            .iter()
            .flat_map(|quantity| {
                let dimension = self.get_dimension_definition(&quantity);
                quantity.units_def.units.iter().map(move |unit| {
                    let unit_symbol = unit.symbol.as_ref().unwrap();
                    let unit_factor = unit.factor;
                    quote! {
                        ({ #dimension }, #unit_symbol, #unit_factor),
                    }
                })
            })
            .collect();

        quote! {
            pub const #unit_names_type: &[(#dimension_type, &str, f64)] = &[
                #unit_names_array_gen
            ];
        }
    }
}
