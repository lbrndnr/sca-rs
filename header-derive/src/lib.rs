use proc_macro::TokenStream;
use quote::quote;
use syn::{
    spanned::Spanned, Data::Struct, DeriveInput, Error, Expr, Ident, LitInt, Type
};

mod bit_len;
mod debug;
mod into;
mod try_from;

struct FieldDef {
    name: Ident,
    ty: Type,
    bit_ty: Type,
    bit_len: usize,
    cond: Option<Expr>
}

struct ProtoDef {
    field: Vec<Ident>,
    ty: Vec<Type>,
    bit_ty: Vec<Type>,
    bit_len: Vec<usize>,
    cond: Vec<Expr>
}

impl ProtoDef {
    
    fn new() -> Self {
        ProtoDef {
            field: Vec::new(),
            ty: Vec::new(),
            bit_ty: Vec::new(),
            bit_len: Vec::new(),
            cond: Vec::new()
        }
    }

    fn push(&mut self, field: FieldDef) {
        self.field.push(field.name);
        self.ty.push(field.ty);
        self.bit_ty.push(field.bit_ty);
        self.bit_len.push(field.bit_len);
        self.cond.push(field.cond.unwrap_or(Expr::Verbatim(quote! { true })));
    }

    fn true_cond(&self) -> Vec<Expr> {
        let true_expr = Expr::Verbatim(quote! { true });
        vec![true_expr; self.cond.len()]
    }

}

fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }
    None
}

fn parse_field(field: &syn::Field) -> Result<Option<FieldDef>, Error> {
    let attr = field.attrs.iter().find(|attr| {
        attr.path().is_ident("field")
    });

    if attr.is_none() {
        return Ok(None)
    }
    let attr = attr.unwrap();

    let mut bit_len = 0;
    let mut cond = None;

    let res = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("bit_len") {
            let content;
            syn::parenthesized!(content in meta.input);
            let lit: LitInt = content.parse()?;
            bit_len = lit.base10_parse()?;
            Ok(())
        }
        else if meta.path.is_ident("cond") {
            let content;
            syn::parenthesized!(content in meta.input);
            let expr: Expr = content.parse()?;
            cond = Some(expr);
            Ok(())
        }
        else {
            Err(syn::Error::new_spanned(meta.path, "Unknown field attribute"))
        }
    });

    if let Err(e) = res {
        Err(e)
    }
    else {
        let bit_ty = if let Some(ity) = ty_inner_type("Option", &field.ty) {
            ity.clone()
        }
        else {
            field.ty.clone()
        };

        Ok(Some(FieldDef {
            name: field.ident.clone().unwrap(),
            ty: field.ty.clone(),
            bit_ty: bit_ty,
            bit_len,
            cond  
        }))
    }
}

fn parse_struct(ast: &DeriveInput) -> Result<ProtoDef, Error> {
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data {
        named
    } else {
        unimplemented!();
    };

    let mut res = ProtoDef::new();
    for f in fields {
        match parse_field(f) {
            Ok(Some(hdr)) => {
                res.push(hdr);
            },
            Ok(None) => { },
            Err(e) => {
                return Err(e)
            }
        }
    }

    Ok(res)
}


#[proc_macro_derive(Header, attributes(field))]
pub fn scars_header(input: TokenStream) -> TokenStream {
    header_impl(input, "scars")
}

#[proc_macro_derive(CrateHeader, attributes(field))]
pub fn crate_header(input: TokenStream) -> TokenStream {
    header_impl(input, "crate")
}

fn header_impl(input: TokenStream, crate_name: &str) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let crate_name = Ident::new(crate_name, ast.span());

    if let Struct(_) = ast.data {
        match parse_struct(&ast) {
            Ok(def) => {
                let mut hdr_impl = TokenStream::new();
        
                let bit_len_impl = bit_len::derive_proc_macro_impl(&ast.ident, &def, &crate_name);
                hdr_impl.extend(bit_len_impl);

                let debug_impl = debug::derive_proc_macro_impl(&ast.ident, &def, &crate_name);
                hdr_impl.extend(debug_impl);

                let into_impl = into::derive_proc_macro_impl(&ast.ident, &def, &crate_name);
                hdr_impl.extend(into_impl);
        
                let try_from_impl = try_from::derive_proc_macro_impl(&ast.ident, &def, &crate_name);
                hdr_impl.extend(try_from_impl);
        
                hdr_impl.into()
            },
            Err(e) => {
                e.to_compile_error().into()
            }
        }
    }
    else {
        unimplemented!()
    }
}