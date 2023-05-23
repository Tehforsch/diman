#![feature(proc_macro_diagnostic)]

mod expression;
mod parse;
mod types;
mod verify;
// mod storage_types;
// mod codegen;

use syn::*;
// use types::Defs;
use verify::Verify;

#[proc_macro]
pub fn unit_system(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as parse::types::Defs);
    let _defs: types::Defs = defs.verify().unwrap();
    // defs.code_gen().into()
    proc_macro::TokenStream::new()
}
