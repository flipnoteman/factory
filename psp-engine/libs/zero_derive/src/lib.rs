extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Type, Data, DeriveInput};

#[proc_macro_derive(Zero)]
pub fn zero_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse(input).unwrap(); // Build the impl
    impl_zero(&ast)
}

fn impl_zero(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Zero can only be derived for structs"),
    };
    let field_zeroes = fields.iter().map(|field| {
        let name = &field.ident;
        match &field.ty { 
            Type::Path(type_path) if type_path.path.segments.iter().any(|seg| seg.ident == "f32" || seg.ident == "f64") => {
                quote! { self.#name = 0.0; }
            },
            Type::Path(type_path) if type_path.path.segments.iter().any(|seg| seg.ident == "i16" || seg.ident == "i32" || seg.ident == "i64" || seg.ident == "u16" || seg.ident == "u32" || seg.ident == "u64") => {
                quote! { self.#name = 0; }
            },
            _ => panic!("Zero can only be derived for float and integer fields.")
        }
    });
    let gen = quote! {
        impl #name {
            pub fn zero(&mut self) {
                #(#field_zeroes)*
            }
        }
    };

    gen.into()
}
