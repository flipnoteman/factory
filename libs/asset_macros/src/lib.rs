extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn AssetType(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = syn::parse_macro_input!(input as DeriveInput);
    let attrs = &ast.attrs;
    let vis = &ast.vis;
    let name = &ast.ident;

    let mut new_fields = Vec::new();

    if let syn::Data::Struct(data) = &ast.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                new_fields.push(field.clone());
            }
        }
    }

    new_fields.push(syn::Field::parse_named.parse2(quote! { pub handle: *const core::ffi::c_void }).unwrap());
    new_fields.push(syn::Field::parse_named.parse2(quote! { file_descriptor: psp::sys::SceUid }).unwrap());

    let expanded = quote! {
        #(#attrs)*
        #vis struct #name {
            #(#new_fields),*
        }
    };

    TokenStream::from(expanded)
}