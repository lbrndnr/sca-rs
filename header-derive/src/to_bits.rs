use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>) -> TokenStream {
    let bit_len: Vec<_> = hdr
        .iter()
        .map(|f| f.bit_len.clone())
        .collect();

    let expanded = quote! {
        impl scars::ToBits for #name {
            fn num_bits() -> usize {
                #(#bit_len + )* 0
            }
        }
    };

    expanded.into()
}