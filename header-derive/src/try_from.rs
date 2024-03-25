use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive_proc_macro_impl(ast: &DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let expanded = quote! {
        impl TryFrom<&[u8]> for #ident {
            type Error = DecodingError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                use crate::bit::BitRange;
                let mut s = 0;

                let mut res = Self::default();

                $(
                    let val: $type = value
                        .get_bit_range(s..s+$size)
                        .map_err(|_| Self::Error::TypeTooShort)?;
                    res.$field = make_opt![val; $($cond)*];

                    s += $size;
                )*

                Ok(res)
            }
        }
    };

    expanded.into()
}