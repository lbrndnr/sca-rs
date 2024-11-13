use crate::utils::def::ProtoDef;

use anyhow::Result;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Ident, Type};

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {            
    let impl_checked = proto_impl(name, def, true, crate_name);
    let impl_unchecked = proto_impl(name, def, false, crate_name);

    let expanded = quote! {
        macro_rules! wrap {
            (true, $cond: expr, $bits: expr) => {
                if $cond {
                    Some($bits.try_into_bytes()?)
                } else {
                    None
                }
            };
            (false, $cond: expr, $bits: expr) => {
                $bits.try_into_bytes()?
            };
        }

        impl TryFrom<&[u8]> for #name {
            type Error = #crate_name::Error;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                use #crate_name::bit::BitRange;
                #impl_checked
            }
        }

        impl #crate_name::TryFromUnchecked<&[u8]> for #name {
            type Error = #crate_name::Error;

            fn try_from_unchecked(value: &[u8]) -> Result<Self, Self::Error> {
                use #crate_name::bit::BitRange;
                #impl_unchecked
            }
        }

    };

    expanded.into()
}

fn proto_impl(name: &Ident, def: &ProtoDef, checked: bool, crate_name: &Ident) -> proc_macro2::TokenStream {
    let field = &def.field;
    let ty = &def.ty;
    let opt = &def.optional;

    let bit_len = &def.bit_len;
    let true_cond = &def.true_cond();
    let cond = if checked { &def.cond } else { true_cond };

    quote! {
        use #crate_name::{
            TryIntoBytes
        };

        let mut s = 0;

        #(
            let bit_len = #bit_len;
            let bits = if bit_len > 0 {
                value.get_bit_range(s..s+(bit_len as usize))
                    .map_err(|_| #crate_name::Error::FieldDeserialization(stringify!(#field).to_string()))
            }
            else {
                Err(#crate_name::Error::FieldDeserialization(stringify!(#field).to_string()))
            }?;

            let #field: #ty = wrap!(#opt, #cond, bits);

            s += bit_len as usize;
        )*

        Ok(Self {
            #(
                #field
            ),*
        })
    }
}