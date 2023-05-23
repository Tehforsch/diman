#![feature(proc_macro_diagnostic)]

mod expression;
mod parse;
mod types;
mod verify;
mod resolve;
mod dimension_math;
mod storage_types;
mod codegen;

use syn::*;
use verify::Verify;

#[proc_macro]
pub fn unit_system(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as parse::types::Defs);
    let defs: types::UnresolvedDefs = defs.verify().unwrap();
    let resolved: types::Defs = defs.resolve().unwrap_or_else(|e| panic!("{}", e));
    resolved.code_gen().into()
}
