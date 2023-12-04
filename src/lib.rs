#![allow(clippy::needless_doctest_main)]
//! # Automation of Destructure Pattern
//! `destructure` is a automation library for `destructure pattern`.
//! 
//! ## Usage
//! ```rust
//! use destructure::Destructure;
//! 
//! #[derive(Destructure)]
//! pub struct Book {
//!     id: u64,
//!     name: String,
//!     stocked_at: String,
//!     author: String,
//!     // ... too many fields...
//! }
//! 
//! fn main() {
//!     let book = Book {
//!         id: 1234_5678_9999_0000u64,
//!         name: "name".to_string(),
//!         stocked_at: "2023/01/03".to_string(),
//!         author: "author".to_string()
//!     };
//! 
//!     // Auto generate
//!     let des: DestructBook = book.into_destruct();
//! 
//!     println!("{:?}", des.id);
//! }
//! ```
//! 
//! ### What is `destructure pattern`?
//! A structure with too many fields makes it hard to call constructors,
//! but it is also hard work to prepare a `Getter/Setter` for each one.
//! There are macros for this purpose, but even so, a large number of macros reduces readability.
//! This is especially true when using `From<T>` Trait.
//! 
//! So how can this be simplified? It is the technique of "converting all fields to public". 
//!   
//! This allows for a simplified representation, as in the following example
//! 
//! ```rust
//! pub struct Book {
//!     id: u64,
//!     name: String,
//!     stocked_at: String,
//!     author: String,
//!     // ... too many fields...
//! }
//! 
//! impl Book {
//!     pub fn into_destruct(self) -> DestructBook {
//!         DestructBook {
//!             id: self.id,
//!             name: self.name,
//!             stocked_at: self.stocked_at,
//!             author: self.author,
//!         }
//!     }
//! }
//! 
//! pub struct DestructBook {
//!     pub id: u64,
//!     pub name: String,
//!     pub stocked_at: String,
//!     pub author: String,
//!     // ... too many fields (All `public`.)...
//! }
//!
//! fn main() {
//!     let book = Book {
//!         id: 1234_5678_9999_0000u64,
//!         name: "name".to_string(),
//!         stocked_at: "2023/01/03".to_string(),
//!         author: "author".to_string()
//!     };
//!     
//!     let des = book.into_destruct();
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
//! ```rust
//! use destructure::Destructure;
//!
//! #[derive(Destructure)]
//! pub struct Book {
//!     id: u64,
//!     name: String,
//!     stocked_at: String,
//!     author: String,
//!     // ... too many fields...
//! }
//!
//! fn main() {
//!     let book = Book {
//!         id: 1234_5678_9999_0000u64,
//!         name: "name".to_string(),
//!         stocked_at: "2023/01/03".to_string(),
//!         author: "author".to_string()
//!     };
//!
//!     // Auto generate
//!     let des: DestructBook = book.into_destruct();
//!
//!     println!("{:?}", des.id);
//! }
//!```
//!
//! You can also perform safe value substitution by using `reconstruct()`,
//! which performs the same role as the following usage.
//! ```rust
//! use destructure::Destructure;
//!
//! #[derive(Debug, Eq, PartialEq, Clone, Destructure)]
//! pub struct Book {
//!     id: u64,
//!     name: String,
//!     stocked_at: String,
//!     author: String,
//!     // ... too many fields...
//! }
//!
//! fn main() {
//!    let before = Book {
//!        id: 1234_5678_9999_0000u64,
//!        name: "name".to_string(),
//!        stocked_at: "2023/01/03".to_string(),
//!        author: "author".to_string()
//!    };
//!
//!    let author = "After".to_string();
//!
//!    // Auto generate
//!    let after = before.clone().reconstruct(|before| {
//!        before.author = author;
//!    });
//!
//!    assert_ne!(before, after);
//! }
//```

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

/// Automatically implements `into_destruct()` and `freeze()` methods.
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

    let freeze = expanded.clone();

    let q = quote::quote! {
        /// Do not have an explicit implementation for this structure.
        pub struct #generate_ident {
            #(#destruction,)*
        }

        impl #name {
            /// Convert the field value to a fully disclosed Destruct structure.
            /// 
            /// If you wish to revert the Destruct structure back to the original structure, see `freeze()`.
            pub fn into_destruct(self) -> #generate_ident {
                #generate_ident { #(#expanded,)* }
            }

            /// It provides a mechanism for replacing the contents by [`into_destruct()`]
            /// and changing the actual value by [`freeze()`] using a limited closure.
            ///
            /// If you wish to use Result, see [`try_reconstruct()`].
            pub fn reconstruct(self, f: impl FnOnce(&mut #generate_ident)) -> #name {
                let mut dest = self.into_destruct();
                f(&mut dest);
                dest.freeze()
            }

            pub fn try_reconstruct<E>(self, f: impl FnOnce(&mut #generate_ident) -> Result<(), E>) -> Result<#name, E> {
                let mut dest = self.into_destruct();
                f(&mut dest)?;
                Ok(dest.freeze())
            }
        }

        impl #generate_ident {
            /// Restore the Destruct structure to its original structure again.
            pub fn freeze(self) -> #name {
                #name { #(#freeze,)* }
            }
        }
    };

    q.into()
}


#[proc_macro_derive(Mutation)]
pub fn derive_mutation(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let generate = format!("{}Mut", name);
    let generate_ident = Ident::new(&generate, name.span());

    let fields = if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { ref named, ..}), .. }) = ast.data {
        named
    } else {
        unimplemented!();
    };

    let destruction = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub #name: &'mutation mut #ty
        }
    });

    let expanded = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: &mut self.#name
        }
    });

    let q = quote::quote! {
        /// Do not have an explicit implementation for this structure.
        pub struct #generate_ident<'mutation> {
            #(#destruction,)*
        }

        impl #name {
            pub fn substitute(&mut self, mut f: impl FnMut(&mut #generate_ident)) {
                f(&mut #generate_ident {
                    #(#expanded,)*
                })
            }
        }
    };

    q.into()
}