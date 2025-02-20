extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn AssetType(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let ast = syn::parse_macro_input!(input as DeriveInput);
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

    // Prepare field identifiers and types for the Default impl
    let field_idents: Vec<_> = new_fields.clone()
        .into_iter()
        .map(|f| f.ident.unwrap())
        .collect();

    new_fields.push(syn::Field::parse_named.parse2(quote! { pub handle: Option<*mut core::ffi::c_void> }).unwrap());
    new_fields.push(syn::Field::parse_named.parse2(quote! { file_descriptor: psp::sys::SceUid }).unwrap());
    new_fields.push(syn::Field::parse_named.parse2(quote! { pub size: u32 }).unwrap());


    // Generate the Default impl
    let default_impl = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self {
                    handle: None,
                    file_descriptor: psp::sys::SceUid(1),
                    size: 0,
                    #( #field_idents: Default::default() ),*
                }
            }
        }
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #name {
            #(#new_fields),*
        }

        #default_impl
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn AssetHandler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // let mut new_fields = Vec::new();

    // if let syn::Data::Struct(data) = &ast.data {
    //     if let Fields::Named(fields) = &data.fields {
    //         for field in &fields.named {
    //             new_fields.push(field.clone());
    //         }
    //     }
    // }

    let expanded = quote!{
        use asset_handling::asset_handler::AssetHandler;
        use lazy_static::*;
        use spin::mutex::Mutex;

        lazy_static! {
            static ref #name: Mutex<AssetHandler> = Mutex::new(AssetHandler::new());
        }
    };


    TokenStream::from(expanded)
}
