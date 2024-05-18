use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr,
    Ident
};

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>, crate_name: &Ident) -> TokenStream {
    let field: Vec<_> = hdr
        .iter()
        .map(|f| f.name.clone())
        .collect();
    let bit_len: Vec<_> = hdr
        .iter()
        .map(|f| f.bit_len.clone())
        .collect();
    let cond: Vec<_> = hdr
        .iter()
        .map(|f| f.cond.clone().unwrap_or(Expr::Verbatim(quote! { true })))
        .collect();

    let expanded = quote! {
        impl #crate_name::ToBits for #name {
            fn num_bits(&self) -> usize {
                let mut num = 0;
                #(
                    let #field = self.#field;
                    if #cond {
                        num += #bit_len
                    }
                )*

                num
            }
        }
    };

    expanded.into()
}