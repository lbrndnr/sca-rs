use crate::utils::def::ProtoDef;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {
    let impl_checked = proto_impl(def, true);
    let impl_unchecked = proto_impl(def, false);

    let expanded = quote! {
        macro_rules! unwrap {
            (Option<NBitVec>, $val: expr) => {
                if let Some(val) = $val {
                    val.len()
                } else {
                    0usize
                }
            };
            (Option<$ty: ident>, $val: expr) => {
                if let Some(val) = $val {
                    val as usize
                } else {
                    0usize
                }
            };
            ($ty: ident, $val: expr) => {
                $val as usize
            };
        }

        impl From<#name> for Vec<u8> {
            fn from(value: #name) -> Vec<u8> {
                use bitvec::prelude::*;
                use core::mem::size_of_val;
                use #crate_name::{
                    BitLen, hdr::NBitVec
                };

                #impl_checked
            }
        }

        impl #crate_name::FromUnchecked<#name> for Vec<u8> {
            fn from_unchecked(value: #name) -> Vec<u8> {
                use bitvec::prelude::*;
                use core::mem::size_of_val;
                use #crate_name::{
                    BitLen, hdr::NBitVec
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
        let mut res = NBitVec::new();

        #(
            let #field = unwrap!(#ty, value.#field);
            // TODO: check this at compile time
            if #cond && #bit_len > 0 {
                let bits = #field.view_bits::<Msb0>();
                res.extend_from_bitslice(&bits[bits.len()-#bit_len..bits.len()]);
            }
        )*

        res.into()
    }
}
