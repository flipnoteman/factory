extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

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
        quote! { self.#name = 0; }
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