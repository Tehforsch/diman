use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index, Meta,
    NestedMeta, Path, };

#[proc_macro_derive(Verify, attributes(verified))]
pub fn derive_verify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let verified_type = get_verified_type_name(&input);

    let name = input.ident;

    // Add a bound `T: Verify` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let verify_impl = gen_verify_impl(&input.data, &verified_type);

    quote! {
        impl #impl_generics crate::verify::Verify for #name #ty_generics #where_clause {
            type Verified = #verified_type;
            fn verify(self) -> Result<Self::Verified> {
                Ok(#verify_impl)
            }
        }
    }
    .into()
}

fn get_verified_type_name(input: &DeriveInput) -> Path {
    for attribute in input.attrs.iter().filter_map(|attr| attr.parse_meta().ok()) {
        let name_value = if let Meta::List(name_value) = attribute {
            name_value
        } else {
            continue;
        };
        if let NestedMeta::Meta(x) = &name_value.nested[0] {
            return x.path().to_owned();
        }
    }
    panic!("Missing/malformed verified attribute.");
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(crate::verify::Verify));
        }
    }
    generics
}

fn gen_verify_impl(data: &Data, t: &Path) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let fields: TokenStream = fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident;
                        quote! {
                            #name: self.#name.verify()?,
                        }
                    })
                    .collect();
                quote! {
                    #t { #fields }
                }
            }
            Fields::Unnamed(ref fields) => {
                let fields: TokenStream = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let index = Index::from(i);
                        quote! {
                            &self.#index.verify()?,
                        }
                    })
                    .collect();
                quote! {
                    #t ( #fields )
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        Data::Enum(ref data) => {
            let variant_cons: TokenStream = data
                .variants
                .iter()
                .map(|variant| {
                    let ident = &variant.ident;
                    let fields_match: TokenStream = variant
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let ident = format_ident!("field{}", i);
                            quote! {
                                #ident,
                            }
                        })
                        .collect();
                    let fields_cons: TokenStream = variant
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let ident = format_ident!("field{}", i);
                            quote! {
                                #ident.verify()?,
                            }
                        })
                        .collect();
                    quote! {
                        Self::#ident(#fields_match) => #t::#ident(#fields_cons),
                    }
                })
                .collect();
            quote! {
                match self {
                    #variant_cons
                }
            }
        }
        Data::Union(_) => unimplemented!(),
    }
}
