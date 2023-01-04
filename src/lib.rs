#![allow(clippy::needless_doctest_main)]
//! # Automation of Destructure Pattern
//! `destructure` is a automation library for `destructure pattern`.
//! 
//! ## Usage
//! ```rust, no_run
//! use destructure::Destructure;
//! 
//! #[derive(Destructure)]
//! pub struct AuthenticateResponse {
//!     id: Uuid,
//!     user_code: String,
//!     verification_uri: String,
//!     expires_in: i32,
//!     message: String,
//!     ... // too many fields...
//! }
//! 
//! fn main() {
//!     let res = reqwest::get("http://example.com")
//!         .send().await.unwrap()
//!         .json::<AuthenticateResponse>().await.unwrap();
//! 
//!     // Auto generate
//!     let des: DestructAuthenticateResponse = res.into_destruct();
//! 
//!     println!("{:?}", des.id);
//! }
//! ```
//! 
//! ### What is `destructure pattern`?
//! A structure with too many fields makes it hard to call constructors, but it is also hard work to prepare a `Getter/Setter` for each one. There are macros for this purpose, but even so, a large number of macros reduces readability. This is especially true when using `From<T>` Trait.  
//! 
//! So how can this be simplified? It is the technique of "converting all fields to public". 
//!   
//! This allows for a simplified representation, as in the following example
//! 
//! ```rust, no_run
//! pub struct AuthenticateResponse {
//!     id: Uuid,
//!     user_code: String,
//!     verification_uri: String,
//!     expires_in: i32,
//!     message: String,
//!     ... // too many fields...
//! }
//! 
//! impl AuthenticateResponse {
//!     pub fn into_destruct(self) -> DestructAuthenticateResponse {
//!         DestructAuthenticateResponse {
//!             id: self.id,
//!             user_code: self.user_code,
//!             verification_uri: self.verification_uri,
//!             expires_in: self.expires_in,
//!             message: self.message,
//!             ...
//!         }
//!     }
//! }
//! 
//! pub struct DestructAuthenticateResponse {
//!     pub id: Uuid,
//!     pub user_code: String,
//!     pub verification_uri: String,
//!     pub expires_in: i32,
//!     pub message: String,
//!     ... // too many fields (All `public`.)...
//! }
//! 
//! fn main() {
//!     let res = reqwest::get("http://example.com")
//!         .send().await.unwrap()
//!         .json::<AuthenticateResponse>().await.unwrap();
//!     
//!     let des = res.into_destruct();
//! 
//!     println!("{:?}", des.id);
//! }
//! ```
//!   
//! There are several problems with this method, the most serious of which is the increase in boilerplate.  
//! Using the multi-cursor feature of the editor, this can be done by copy-pasting, but it is still a hassle.  
//! 
//! Therefore, I created a *Procedural Macro* that automatically generates structures and methods:
//! 
//! ```rust, no_run
//! use destructure::Destructure;
//! 
//! #[derive(Destructure)]
//! pub struct AuthenticateResponse {
//!     id: Uuid,
//!     user_code: String,
//!     verification_uri: String,
//!     expires_in: i32,
//!     message: String,
//!     ... // too many fields...
//! }
//! 
//! fn main() {
//!     let res = reqwest::get("http://example.com")
//!         .send().await.unwrap()
//!         .json::<AuthenticateResponse>().await.unwrap();
//! 
//!     // Auto generate
//!     let des: DestructAuthenticateResponse = res.into_destruct();
//! 
//!     println!("{:?}", des.id);
//! }
//!```

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput,
    Ident,
    Data,
    DataStruct,
    Fields,
    FieldsNamed,
};

#[proc_macro_derive(Destructure)]
pub fn derive_destructure(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let generate = format!("Destruct{}", name);
    let generate_ident = Ident::new(&generate, name.span());

    let fields = if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { ref named, ..}), .. }) = ast.data {
        named
    } else {
        unimplemented!()
    };

    let destruction = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub #name: #ty
        }
    });

    let expanded = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: self.#name
        }
    });

    quote::quote! {
        /// DO NOT USE IMPL
        pub struct #generate_ident {
            #(#destruction,)*
        }

        impl #name {
            pub fn into_destruct(self) -> #generate_ident {
                #generate_ident { #(#expanded,)* }
            }
        }
    }.into()
}