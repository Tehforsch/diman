#![feature(proc_macro_diagnostic)]

mod parse;
mod types;
mod expression;
// mod storage_types;
// mod codegen;

use syn::*;
use types::{Defs};

#[proc_macro]
pub fn unit_system(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _defs = parse_macro_input!(item as Defs);
    // defs.code_gen().into()
    proc_macro::TokenStream::new()
}
