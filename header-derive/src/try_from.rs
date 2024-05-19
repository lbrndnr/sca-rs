use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident,
    Expr,
    Type
};

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>, crate_name: &Ident) -> TokenStream {         
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
    
    let true_expr = Expr::Verbatim(quote! { true });
    let cond: Vec<_> = hdr
        .iter()
        .map(|f| f.cond.clone().unwrap_or(true_expr.clone()))
        .collect();
    let always_true = vec![true_expr; cond.len()];

    let wrap_checked = Ident::new("wrap_checked", name.span());
    let wrap_unchecked = Ident::new("wrap_unchecked", name.span());

    let impl_checked = proto_impl(&field, &ty, &bit_ty, &bit_len, &cond, &wrap_checked);
    let impl_unchecked = proto_impl(&field, &ty, &bit_ty, &bit_len, &always_true, &wrap_unchecked);

    let expanded = quote! {
        macro_rules! wrap_checked {
            (Option<$ty: ident>, $val: expr, $cond: expr) => {
                if $cond {
                    let v = $val.map_err(|_| #crate_name::Error::Decoding)?;
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
                    $val.map_err(|_| #crate_name::Error::Decoding)
                        .map(|v| $ty::from(v))
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

fn proto_impl(field: &Vec<Ident>, ty: &Vec<Type>, bit_ty: &Vec<Type>, bit_len: &Vec<usize>, cond: &Vec<Expr>, wrap: &Ident) -> proc_macro2::TokenStream {
    quote! {
        let mut s = 0;

        #(
            let bits: Result<#bit_ty, _>  = value
                .get_bit_range(s..s+#bit_len);
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