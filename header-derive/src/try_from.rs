use crate::utils::def::ProtoDef;
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {            
    let impl_checked = proto_impl(name, def, true, crate_name);
    let impl_unchecked = proto_impl(name, def, false, crate_name);

    let expanded = quote! {
        macro_rules! wrap {
            (Option<NBitVec>, $cond: expr, $bits: expr) => {
                if $cond {
                    Some($bits?)
                } else {
                    None
                }
            };
            (Option<$ty: ident>, $cond: expr, $bits: expr) => {
                if $cond {
                    Some($bits?.load_be())
                } else {
                    None
                }
            };
            ($ty: ident, $cond: expr, $bits: expr) => {
                $bits?.load_be()
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

fn proto_impl(_: &Ident, def: &ProtoDef, checked: bool, crate_name: &Ident) -> proc_macro2::TokenStream {
    let field = &def.field;
    let ty = &def.ty;

    let bit_len = &def.bit_len;
    let true_cond = &def.true_cond();
    let cond = if checked { &def.cond } else { true_cond };

    quote! {
        let mut s = 0;

        #(
            let bit_range = if #checked {
                s..s+#bit_len as usize
            }
            else {
                s..s+(#bit_len as usize).min(value.len() * 8 - s)
            };

            let bits = if bit_range.end <= value.len() * 8 {
                Ok(value.view_bits::<Msb0>()[bit_range.clone()].to_bitvec())
            }
            else {
                Err(#crate_name::Error::FieldDeserialization(stringify!(#field).to_string()))
            };

            let valid = if #checked {
                #cond
            }
            else {
                bits.is_ok()
            };

            let #field: #ty = wrap!(#ty, valid && bit_range.len() > 0, bits);

            s += bit_range.len();
        )*

        Ok(Self {
            #(
                #field
            ),*
        })
    }
}