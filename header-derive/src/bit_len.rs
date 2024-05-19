use crate::HeaderField;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr,
    Ident
};

pub fn derive_proc_macro_impl(name: &Ident, hdr: &Vec<HeaderField>, crate_name: &Ident) -> TokenStream {
    let field: Vec<_> = hdr
        .iter()
        .map(|f| f.name.clone())
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

    let impl_checked = proto_impl(&field, &bit_len, &cond);
    let impl_unchecked = proto_impl(&field, &bit_len, &always_true);

    let expanded = quote! {
        impl #crate_name::BitLen for #name {
            fn bit_len(&self) -> usize {
                #impl_checked
            }

            fn bit_len_unchecked(&self) -> usize {
                #impl_unchecked
            }
        }
    };

    expanded.into()
}

fn proto_impl(field: &Vec<Ident>, bit_len: &Vec<usize>, cond: &Vec<Expr>) -> proc_macro2::TokenStream {
    quote! {
        let mut num = 0;
        #(
            let #field = self.#field;
            if #cond {
                num += #bit_len
            }
        )*

        num
    }
}