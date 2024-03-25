use proc_macro::TokenStream;
use syn::{
    DeriveInput,
    parse_macro_input,
    Data::Struct,
};

mod to_bits;

#[proc_macro_derive(Header, attributes(bits))]
pub fn header(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    if let Struct(_) = ast.data {
        let to_bits_impl = to_bits::derive_proc_macro_impl(&ast);
        to_bits_impl.into()
    }
    else {
        unimplemented!()
    }
}