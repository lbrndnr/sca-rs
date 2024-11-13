use crate::utils::def::ProtoDef;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {
    let impl_checked = proto_impl(def, true);
    let impl_unchecked = proto_impl(def, false);

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

                #impl_checked
            }
        }

        impl #crate_name::FromUnchecked<#name> for Vec<u8> {
            fn from_unchecked(value: #name) -> Vec<u8> {
                use core::mem::size_of_val;
                use #crate_name::{
                    bit::BitRange,
                    BitLen
                };

                #impl_unchecked
            }
        }
    };

    expanded.into()
}

fn proto_impl(def: &ProtoDef, checked: bool) -> proc_macro2::TokenStream {
    let field = &def.field;
    let ty = &def.ty;
    let bit_len = &def.bit_len;
    let true_cond = &def.true_cond();
    let cond = if checked { &def.cond } else { true_cond };

    quote! {
        let len = (value.bit_len() as f32 / 8.0).ceil() as usize;
        let mut res = vec![0; len];
        let mut s = 0;

        #(
            let #field = unwrap!(#ty, value.#field) as i64;
            let bit_len = #bit_len;
            if #cond && bit_len > 0 {
                let bit_len = bit_len as usize;
                let bytes = #field.to_be_bytes();
                let wrapping_bit_len = 8*size_of_val(&bytes);
                for i in 0..bit_len {
                    let bit = bytes.get_bit(wrapping_bit_len-bit_len+i).unwrap();
                    let mask = (bit as u8) << (7 - ((s+i) % 8));
                    res[((s+i) / 8) as usize] |= mask;
                }
                s += bit_len;
            }
        )*

        res
    }
}