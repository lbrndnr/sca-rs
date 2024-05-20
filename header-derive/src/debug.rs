use crate::utils::def::ProtoDef;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, _crate_name: &Ident) -> TokenStream {
    let field = &def.field;

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