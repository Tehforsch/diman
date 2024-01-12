#![feature(proc_macro_diagnostic)]

mod codegen;
mod dimension_math;
mod parse;
mod resolve;
mod types;

use proc_macro2::TokenStream;
use syn::*;

#[proc_macro]
pub fn unit_system(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as types::UnresolvedTemplates);
    let expanded = defs.expand_templates();
    let resolved = expanded.resolve();
    let impls: TokenStream = resolved.code_gen();
    impls.into()
}
