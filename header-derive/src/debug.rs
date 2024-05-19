use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>, _crate_name: &Ident) -> TokenStream {
    let field: Vec<_> = hdr
        .iter()
        .map(|f| f.name.clone())
        .collect();

    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                #(
                    .field(stringify!(#field), &self.#field)
                )*
                 .finish()
            } 
        }
    };

    expanded.into()
}