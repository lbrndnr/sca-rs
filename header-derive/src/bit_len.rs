use crate::ProtoDef;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr,
    Ident
};

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {
    let impl_checked = proto_impl(&def.field, &def.bit_len, &def.cond);
    let impl_unchecked = proto_impl(&def.field, &def.bit_len, &def.true_cond());

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