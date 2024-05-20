use crate::ProtoDef;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {            
    let impl_checked = proto_impl(name, def, true, crate_name);
    let impl_unchecked = proto_impl(name, def, false, crate_name);

    let expanded = quote! {
        macro_rules! wrap_checked {
            (Option<$ty: ident>, $val: expr, $cond: expr) => {
                if $cond {
                    let v = $val?;
                    Some($ty::from(v))
                } else {
                    None
                }
            };
            ($ty: ident, $val: expr, $cond: expr) => {
                $val.unwrap() as $ty
            };
        }

        macro_rules! wrap_unchecked {
            (Option<$ty: ident>, $val: expr, $cond: expr) => {
                if $cond {
                    $val.map(|v| $ty::from(v))
                        .ok()
                } else {
                    None
                }
            };
            ($ty: ident, $val: expr, $cond: expr) => {
                $val.unwrap() as $ty
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
    let bit_ty = &def.bit_ty;
    let bit_len = &def.bit_len;
    let true_cond = &def.true_cond();
    let cond = if checked { &def.cond } else { true_cond };

    let wrap = if checked {
        Ident::new("wrap_checked", name.span())
    }
    else {
        Ident::new("wrap_unchecked", name.span())
    };

    quote! {
        let mut s = 0;

        #(
            let bits: Result<#bit_ty, _>  = value
                .get_bit_range(s..s+#bit_len)
                .map_err(|_| #crate_name::Error::FieldDeserialization(stringify!(#field).to_string()));

            let #field: #ty = #wrap!(#ty, bits, #cond);

            s += #bit_len;
        )*

        Ok(Self {
            #(
                #field
            ),*
        })
    }
}