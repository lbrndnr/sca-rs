use crate::ProtoDef;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {
    let field = &def.field;
    let ty = &def.ty;
    let bit_len = &def.bit_len;
    let cond = &def.cond;

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
                use #crate_name::{
                    bit::BitRange,
                    BitLen
                };

                let len = (value.bit_len() as f32 / 8.0).ceil() as usize;
                let mut res = vec![0; len];
                let mut s = 0;

                #(
                    let #field = value.#field;
                    if #cond {
                        let bytes = unwrap!(#ty, #field).to_be_bytes();
                        let wrapping_bit_len = 8*size_of_val(&bytes);
                        for i in 0..#bit_len {
                            let bit = bytes.get_bit(wrapping_bit_len-#bit_len+i).unwrap();
                            let mask = (bit as u8) << (7 - ((s+i) % 8));
                            res[((s+i) / 8) as usize] |= mask;
                        }
                        s += #bit_len;
                    }
                )*

                res
            }
        }
    };

    expanded.into()
}