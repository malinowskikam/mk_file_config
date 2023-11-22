mod derive;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use derive::derive_file_config_impl;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FileConfig)]
pub fn derive_file_config(input: TokenStream) -> TokenStream {
    derive_file_config_impl(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
