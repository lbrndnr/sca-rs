use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident,
    Expr,
    Type
};

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>, crate_name: &Ident) -> TokenStream {            
    let true_expr = Expr::Verbatim(quote! { true });
    let cond: Vec<_> = hdr
        .iter()
        .map(|f| f.cond.clone().unwrap_or(true_expr.clone()))
        .collect();
    let always_true = vec![true_expr; cond.len()];

    let wrap_checked = Ident::new("wrap_checked", name.span());
    let wrap_unchecked = Ident::new("wrap_unchecked", name.span());

    let impl_checked = proto_impl(&hdr, &cond, &wrap_checked, crate_name);
    let impl_unchecked = proto_impl(&hdr, &always_true, &wrap_unchecked, crate_name);

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

fn proto_impl(hdr: &Vec<HeaderField>, cond: &Vec<Expr>, wrap: &Ident, crate_name: &Ident) -> proc_macro2::TokenStream {
    let field: Vec<_> = hdr
        .iter()
        .map(|f| f.name.clone())
        .collect();
    let ty: Vec<_> = hdr
        .iter()
        .map(|f| f.ty.clone())
        .collect();
    let bit_ty: Vec<_> = hdr
        .iter()
        .map(|f| f.bit_ty.clone())
        .collect();
    let bit_len: Vec<_> = hdr
        .iter()
        .map(|f| f.bit_len.clone())
        .collect();

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