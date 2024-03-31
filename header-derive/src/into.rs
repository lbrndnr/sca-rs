use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>) -> TokenStream {
    let field: Vec<syn::Ident> = hdr
        .iter()
        .map(|f| f.name.clone())
        .collect();
    let ty: Vec<syn::Type> = hdr
        .iter()
        .map(|f| f.ty.clone())
        .collect();
    let bit_len: Vec<syn::LitInt> = hdr
        .iter()
        .map(|f| f.bit_len.clone())
        .collect();

    let expanded = quote! {
        impl From<#name> for Vec<u8> {
            fn from(value: #name) -> Vec<u8> {
                use core::mem::size_of;
                use scars::{
                    bit::BitRange,
                    ToBits
                };

                let len = (#name::num_bits() as f32 / 8.0).ceil() as usize;
                let mut res = vec![0; len];
                let mut s = 0;

                #(
                    let bytes = value.#field.to_be_bytes();
                    for i in 0..#bit_len {
                        let bit = bytes.get_bit(8*size_of::<#ty>()-#bit_len+i).unwrap();
                        let mask = (bit as u8) << (7 - ((s+i) % 8));
                        println!("{i} -> {bit} flags: {:#b}", mask);
                        res[((s+i) / 8) as usize] |= mask;
                    }
                    s += #bit_len;
                )*

                res
            }
        }
    };

    expanded.into()
}