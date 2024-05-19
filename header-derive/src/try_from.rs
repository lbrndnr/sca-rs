use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident,
    Expr
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
    let cond: Vec<_> = hdr
        .iter()
        .map(|f| f.cond.clone().unwrap_or(Expr::Verbatim(quote! { true })))
        .collect();

    let expanded = quote! {
        macro_rules! wrap_if {
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

        impl TryFrom<&[u8]> for #name {
            type Error = #crate_name::Error;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                use #crate_name::bit::BitRange;
                let mut s = 0;

                #(
                    let bits: Result<#bit_ty, _>  = value
                        .get_bit_range(s..s+#bit_len);
                    let #field: #ty = wrap_if!(#ty, bits, #cond);

                    s += #bit_len;
                )*

                Ok(Self {
                    #(
                        #field
                    ),*
                })
            }

        }

    };

    expanded.into()
}