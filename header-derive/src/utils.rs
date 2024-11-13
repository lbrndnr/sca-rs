pub mod def {

    use quote::quote;
    use syn::{
        Expr, 
        Ident, 
        Type
    };
    
    pub struct FieldDef {
        pub name: Ident,
        pub ty: Type,
        pub optional: bool,
        pub bit_len: Expr,
        pub cond: Option<Expr>
    }

    pub struct ProtoDef {
        pub field: Vec<Ident>,
        pub ty: Vec<Type>,
        pub optional: Vec<bool>,
        pub bit_len: Vec<Expr>,
        pub cond: Vec<Expr>
    }

    impl ProtoDef {
        
        pub fn new() -> Self {
            ProtoDef {
                field: Vec::new(),
                ty: Vec::new(),
                optional: Vec::new(),
                bit_len: Vec::new(),
                cond: Vec::new()
            }
        }

        pub fn push(&mut self, field: FieldDef) {
            self.field.push(field.name);
            self.ty.push(field.ty);
            self.optional.push(field.optional);
            self.bit_len.push(field.bit_len);
            self.cond.push(field.cond.unwrap_or(Expr::Verbatim(quote! { true })));
        }

        pub fn true_cond(&self) -> Vec<Expr> {
            let true_expr = Expr::Verbatim(quote! { true });
            vec![true_expr; self.cond.len()]
        }

    }

}