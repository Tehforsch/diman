use proc_macro2::TokenStream;

use super::Codegen;

use quote::{quote, quote_spanned};

impl Codegen {
    pub(crate) fn gen_quantity(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let span = quantity_type.span();
        let functions = self.quantity_functions();
        quote_spanned! {span =>
            #[derive(Clone, Copy, Eq, Default)]
            #[repr(transparent)]
            pub struct #quantity_type<S, const D: #dimension_type>(pub(crate) S);
            #functions
        }
    }

    fn quantity_functions(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        quote! {
            impl<S> #quantity_type<S, { #dimension_type::none() }> {
                /// Return the stored value of a dimensionless quantity.
                pub fn value(self) -> S {
                    self.0
                }

                /// Get a reference to the stored value of a dimensionless quantity.
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

                /// Return a reference to the value of a quantity, regardless of whether
                /// it is dimensionless or not. Use this carefully, since the
                /// result depends on the underlying base units
                pub fn value_unchecked_ref(&self) -> &S {
                    &self.0
                }

                /// Create a new quantity for the dimension with a given value.
                /// Use carefully, since the constructed quantity depends on the
                /// used base units.
                pub const fn new_unchecked(s: S) -> Self {
                    Self(s)
                }
            }

            impl<const D: #dimension_type, S> #quantity_type<S, D>
            where
                S: core::ops::Div<Magnitude, Output = S> + core::fmt::Debug,
            {
                pub fn value_in<A: Into<Magnitude>>(self, a: A) -> S {
                    self.value_unchecked() / a.into()
                }
            }

            impl<S> core::ops::Deref for #quantity_type<S, { #dimension_type::none() }> {
                type Target = S;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }
}
