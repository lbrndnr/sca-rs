use crate::utils::def::ProtoDef;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Ident, Type};

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {            
    let impl_checked = proto_impl(name, def, true, crate_name);
    let impl_unchecked = proto_impl(name, def, false, crate_name);

    let expanded = quote! {
        macro_rules! wrap {
            (Option<NBitVec>, $cond: expr, $bits: expr) => {
                if $cond {
                    Some($bits)
                } else {
                    None
                }
            };
            (Option<$ty: ident>, $cond: expr, $bits: expr) => {
                if $cond {
                    Some($bits.load())
                } else {
                    None
                }
            };
            ($ty: ident, $cond: expr, $bits: expr) => {
                $bits.load()
            };
        }

        impl TryFrom<&[u8]> for #name {
            type Error = #crate_name::Error;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                use bitvec::prelude::*;
                #impl_checked
            }
        }

        impl #crate_name::TryFromUnchecked<&[u8]> for #name {
            type Error = #crate_name::Error;

            fn try_from_unchecked(value: &[u8]) -> Result<Self, Self::Error> {
                use bitvec::prelude::*;
                #impl_unchecked
            }
        }

    };

    expanded.into()
}

fn proto_impl(name: &Ident, def: &ProtoDef, checked: bool, crate_name: &Ident) -> proc_macro2::TokenStream {
    let field = &def.field;
    let ty = &def.ty;
    // let opt = &def.optional;

    let bit_len = &def.bit_len;
    let true_cond = &def.true_cond();
    let cond = if checked { &def.cond } else { true_cond };

    quote! {
        let mut s = 0;

        #(
            let bit_len = #bit_len;

            // TODO: check this at compile time
            if bit_len == 0 {
                return Err(#crate_name::Error::FieldDeserialization(stringify!(#field).to_string()));
            }

            let bits = value.view_bits::<Msb0>()[s..s+(bit_len as usize)].to_bitvec();

            let #field: #ty = wrap!(#ty, #cond, bits);

            s += bit_len as usize;
        )*

        Ok(Self {
            #(
                #field
            ),*
        })
    }
}