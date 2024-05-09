use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>) -> TokenStream {
    let field: Vec<_> = hdr
        .iter()
        .map(|f| f.name.clone())
        .collect();
    let ty: Vec<_> = hdr
        .iter()
        .map(|f| f.ty.clone())
        .collect();
    let bit_len: Vec<_> = hdr
        .iter()
        .map(|f| f.bit_len.clone())
        .collect();

    let expanded = quote! {
        macro_rules! unwrap {
            (Option<$ty: ident>, $val: expr) => {
                if let Some(val) = $val {
                    $ty::from(val)
                } else {
                    $ty::default()
                }
            };
            ($ty: ident, $val: expr) => {
                $val as $ty
            };
        }

        impl From<#name> for Vec<u8> {
            fn from(value: #name) -> Vec<u8> {
                use core::mem::size_of_val;
                use scars::{
                    bit::BitRange,
                    ToBits
                };

                let len = (#name::num_bits() as f32 / 8.0).ceil() as usize;
                let mut res = vec![0; len];
                let mut s = 0;

                #(
                    let bytes = unwrap!(#ty, value.#field).to_be_bytes();
                    let wrapping_bit_len = 8*size_of_val(&bytes);
                    for i in 0..#bit_len {
                        let bit = bytes.get_bit(wrapping_bit_len-#bit_len+i).unwrap();
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