use crate::utils::def::ProtoDef;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn derive_proc_macro_impl(name: &Ident, def: &ProtoDef, crate_name: &Ident) -> TokenStream {
    let impl_checked = proto_impl(def, true);
    let impl_unchecked = proto_impl(def, false);

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

fn proto_impl(def: &ProtoDef, checked: bool) -> proc_macro2::TokenStream {
    let field = &def.field;
    let bit_len = &def.bit_len;
    let true_cond = &def.true_cond();
    let cond = if checked { &def.cond } else { true_cond };

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