use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

pub fn derive_file_config_impl(input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput { ident, .. } = input;

    Ok(quote! {
        impl mk_file_config_core::FileConfig for #ident {}
    })
}
