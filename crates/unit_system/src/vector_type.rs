use proc_macro2::TokenStream;

pub struct VectorType {
    pub name: TokenStream,
    pub module_name: TokenStream,
    pub float_type: TokenStream,
    pub num_dims: usize,
}
