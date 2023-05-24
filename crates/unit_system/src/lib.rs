#![feature(proc_macro_diagnostic)]

mod codegen;
mod dimension_math;
mod expression;
mod parse;
mod resolve;
mod storage_types;
mod types;
mod verify;

use syn::*;
use verify::Verify;

#[proc_macro]
pub fn unit_system(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as parse::types::Defs);
    let defs: types::UnresolvedDefs = defs.verify().unwrap();
    let resolved: types::Defs = defs.resolve().unwrap_or_else(|e| {
        e.emit();
        panic!("Unresolvable definitions, see other errors.")
    });
    resolved.code_gen().into()
}
