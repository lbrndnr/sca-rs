use proc_macro::TokenStream;
use syn::{
    Data::Struct, DeriveInput, Expr, Ident, Lit, LitInt, Type
};

mod into;
mod to_bits;
mod try_from;

struct HeaderField {
    name: Ident,
    ty: Type,
    bit_len: LitInt
}

fn field_of(f: &syn::Field) -> Option<&syn::Attribute> {
    for attr in &f.attrs {
        if attr.path().is_ident("field") {
            return Some(attr);
        }
    }

    None
}

fn named_value_of(attr: &syn::Attribute, name: &str) -> Option<Expr> {
    match attr.parse_args() {
        Ok(syn::Meta::NameValue(nv)) => {
            if nv.path.is_ident(name) {
                return Some(nv.value);
            }
            None
        },
        _ => None
    }
}

fn named_lit_int_of(attr: &syn::Attribute, name: &str) -> Option<LitInt> {
    let val = named_value_of(attr, name);
    if val.is_none() { 
        return None
    }

    if let Expr::Lit(val) = val.unwrap() {
        if let Lit::Int(val) = val.lit {
            return Some(val)
        }
    }

    None
}

fn parse_struct(ast: &DeriveInput) -> Vec<HeaderField> {
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data {
        named
    } else {
        unimplemented!();
    };

    fields.iter().filter_map(|f| {
        if let Some(field) = field_of(f) {
            // first we verify whether field uses the expected syntax
            let bit_len = named_lit_int_of(field, "bit_len");

            return bit_len.map(|bit_len| {
                HeaderField {
                    name: f.ident.clone().unwrap(),
                    ty: f.ty.clone(),
                    bit_len
                }
            })
        }
        
        None
    })
    .collect::<Vec<HeaderField>>()
    .into()
}


#[proc_macro_derive(Header, attributes(field))]
pub fn header(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    if let Struct(_) = ast.data {
        let hdr = parse_struct(&ast);
        let mut hdr_impl = TokenStream::new();

        let into_impl = into::derive_proc_macro_impl(&ast.ident, &hdr);
        hdr_impl.extend(into_impl);

        let to_bits_impl = to_bits::derive_proc_macro_impl(&ast.ident, &hdr);
        hdr_impl.extend(to_bits_impl);

        let try_from_impl = try_from::derive_proc_macro_impl(&ast.ident, &hdr);
        hdr_impl.extend(try_from_impl);

        hdr_impl.into()
    }
    else {
        unimplemented!()
    }
}