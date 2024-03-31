use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>) -> TokenStream {
    let field: Vec<syn::Ident> = hdr
        .iter()
        .map(|f| f.name.clone())
        .collect();
    let ty: Vec<syn::Type> = hdr
        .iter()
        .map(|f| f.ty.clone())
        .collect();
    let bit_len: Vec<syn::LitInt> = hdr
        .iter()
        .map(|f| f.bit_len.clone())
        .collect();

    let expanded = quote! {
        impl TryFrom<&[u8]> for #name {
            type Error = scars::Error;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                use scars::bit::BitRange;
                let mut s = 0;

                let mut res = Self::default();

                #(
                    let val: #ty = value
                        .get_bit_range(s..s+#bit_len)
                        .map_err(|_| Self::Error::Decoding)?;
                    res.#field = val;

                    s += #bit_len;
                )*

                Ok(res)
            }
        }
    };

    expanded.into()
}