use proc_macro::TokenStream;
use syn::{
    parse_macro_input, Data::Struct, DeriveInput, Expr, Ident, Lit, LitInt, Type
};

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
            let bit_len: Option<LitInt> = match field.parse_args() {
                Ok(syn::Meta::NameValue(nv)) => {
                    if nv.path.is_ident("bit_len") {
                        if let Expr::Lit(val) = nv.value {
                            if let Lit::Int(val) = val.lit {
                                Some(val)
                            }
                            else {
                                None
                            }
                        }
                        else {
                            None
                        }
                    }
                    else {
                        None
                    }
                },
                _ => None
            };

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
    let ast = parse_macro_input!(input as DeriveInput);

    if let Struct(_) = ast.data {
        let hdr = parse_struct(&ast);
        let mut to_bits_impl = to_bits::derive_proc_macro_impl(&ast.ident, &hdr);
        let try_from_impl = try_from::derive_proc_macro_impl(&ast.ident, &hdr);

        to_bits_impl.extend(try_from_impl);
        to_bits_impl.into()
    }
    else {
        unimplemented!()
    }
}