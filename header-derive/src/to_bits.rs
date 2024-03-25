use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive_proc_macro_impl(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;

    let expanded = quote! {
        impl scars::ToBits for #ident {
            fn num_bits() -> usize {
                8
            }
        }
    };

    expanded.into()
}