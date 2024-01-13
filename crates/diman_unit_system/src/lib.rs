#![feature(proc_macro_diagnostic)]

mod codegen;
mod dimension_math;
mod parse;
mod resolve;
mod types;

use codegen::{CallerType, Codegen};
use proc_macro2::TokenStream;
use syn::*;

fn run_unit_system(
    item: proc_macro::TokenStream,
    caller_type: CallerType,
) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as types::UnresolvedTemplates);
    let expanded = defs.expand_templates();
    let resolved = expanded.resolve();
    let impls: TokenStream = Codegen {
        defs: resolved,
        caller_type,
    }
    .code_gen();
    impls.into()
}

#[proc_macro]
pub fn unit_system(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    run_unit_system(input, CallerType::External)
}

// I would like to make this private, but I am not allowed.
// Also #[cfg(test)] doesn't apply here.
#[proc_macro]
pub fn unit_system_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    run_unit_system(input, CallerType::Internal)
}
