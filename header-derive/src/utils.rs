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
        pub bit_ty: Type,
        pub bit_len: usize,
        pub cond: Option<Expr>
    }

    pub struct ProtoDef {
        pub field: Vec<Ident>,
        pub ty: Vec<Type>,
        pub bit_ty: Vec<Type>,
        pub bit_len: Vec<usize>,
        pub cond: Vec<Expr>
    }

    impl ProtoDef {
        
        pub fn new() -> Self {
            ProtoDef {
                field: Vec::new(),
                ty: Vec::new(),
                bit_ty: Vec::new(),
                bit_len: Vec::new(),
                cond: Vec::new()
            }
        }

        pub fn push(&mut self, field: FieldDef) {
            self.field.push(field.name);
            self.ty.push(field.ty);
            self.bit_ty.push(field.bit_ty);
            self.bit_len.push(field.bit_len);
            self.cond.push(field.cond.unwrap_or(Expr::Verbatim(quote! { true })));
        }

        pub fn true_cond(&self) -> Vec<Expr> {
            let true_expr = Expr::Verbatim(quote! { true });
            vec![true_expr; self.cond.len()]
        }

    }

}