use proc_macro2::TokenStream;

pub fn join<const D: usize>(streams: [TokenStream; D]) -> TokenStream {
    streams.into_iter().collect()
}
