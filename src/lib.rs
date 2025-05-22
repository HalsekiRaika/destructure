#![allow(clippy::needless_doctest_main)]
//! # Automation of Destructure Pattern
//! `destructure` is an automation library for `destructure pattern`.
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
//! ```

use proc_macro::TokenStream;
use darling::FromField;
use darling::util::Flag;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident,
    Lifetime, LifetimeParam,
};

#[derive(darling::FromField)]
#[darling(attributes(destructure))]
struct Attributes {
    skip: Flag,
}

/// Automatically implements `into_destruct()` and `freeze()` methods.
//noinspection DuplicatedCode
#[proc_macro_derive(Destructure, attributes(destructure))]
pub fn derive_destructure(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;

    let generate = format!("Destruct{}", name);
    let generate_ident = Ident::new(&generate, name.span());

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        return quote_spanned! { name.span() => compile_error!("Only structures with named fields are supported.") }.into();
    };

    let destruction = fields.iter().map(|field| {
        let Ok(attr) = Attributes::from_field(field) else {
            return quote_spanned! { field.span() => compile_error!("unrecognized attribute.") }.into();
        };
        let name = &field.ident;
        let ty = &field.ty;
        
        if attr.skip.is_present() {
            quote! {
                #name: #ty
            }
        } else {
            quote! {
                pub #name: #ty
            }
        }
    });

    let constructor = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: self.#name
        }
    });

    let freeze = constructor.clone();

    let q = quote::quote! {
        /// Do not have an explicit implementation for this structure.
        pub struct #generate_ident #generics {
            #(#destruction,)*
        }

        impl #generics #name #generics {
            /// Convert the field value to a fully disclosed Destruct structure.
            ///
            /// If you wish to revert the Destruct structure back to the original structure, see `freeze()`.
            pub fn into_destruct(self) -> #generate_ident #generics {
                #generate_ident { #(#constructor,)* }
            }

            /// It provides a mechanism for replacing the contents by [`into_destruct()`]
            /// and changing the actual value by [`freeze()`] using a limited closure.
            ///
            /// If you wish to use Result, see [`try_reconstruct()`].
            pub fn reconstruct(self, f: impl FnOnce(&mut #generate_ident #generics)) -> Self {
                let mut dest = self.into_destruct();
                f(&mut dest);
                dest.freeze()
            }

            pub fn try_reconstruct<E>(self, f: impl FnOnce(&mut #generate_ident #generics) -> Result<(), E>) -> Result<Self, E> {
                let mut dest = self.into_destruct();
                f(&mut dest)?;
                Ok(dest.freeze())
            }
        }

        impl #generics #generate_ident #generics {
            /// Restore the Destruct structure to its original structure again.
            pub fn freeze(self) -> #name #generics {
                #name { #(#freeze,)* }
            }
        }
    };

    q.into()
}

/// Automatically implements `as_destruct()` method.
//noinspection DuplicatedCode
#[proc_macro_derive(DestructureRef)]
pub fn derive_destructure_ref(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;
    let mut destructure_generics = ast.generics.clone();

    let generate = format!("Destruct{}Ref", name);
    let generate_ident = Ident::new(&generate, name.span());

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        return quote_spanned! { name.span() => compile_error!("Only structures with named fields are supported.") }.into();
    };

    let origin_lifetime: Lifetime =
        syn::parse_str("'__origin_destruct_lifetime").expect("cannot parse lifetime");

    destructure_generics
        .params
        .push(syn::GenericParam::Lifetime(LifetimeParam {
            lifetime: origin_lifetime.clone(),
            attrs: Default::default(),
            colon_token: Default::default(),
            bounds: Default::default(),
        }));

    let destruction = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub #name: & #origin_lifetime #ty
        }
    });

    let expanded = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: & self.#name
        }
    });

    let q = quote::quote! {
        /// Do not have an explicit implementation for this structure.
        pub struct #generate_ident #destructure_generics {
            #(#destruction,)*
        }

        impl #generics #name #generics {
            /// Makes the field value to a fully disclosed Destruct structure with access by reference.
            pub fn as_destruct<#origin_lifetime>(& #origin_lifetime self) -> #generate_ident #destructure_generics {
                #generate_ident { #(#expanded,)* }
            }
        }
    };

    q.into()
}

/// Automatically implements `substitute()` methods.
///
/// When performing loop processing, and so on,
/// it is more efficient than using [`reconstruct()`].
/// ## Usage
/// ```rust
/// use destructure::Mutation;
///
/// #[derive(Debug, Mutation)]
/// pub struct Book {
///     id: String,
///     name: String,
/// }
///
/// # fn main() {
/// # let mut book = Book { id: "123456789-abc".to_string(), name: "name".to_string() };
/// book.substitute(|book| {
///     *book.name = "new name".to_string();
/// });
///
/// book.try_substitute(|book| -> Result<(), std::io::Error> {
///    *book.name = "new name".to_string();
///    Ok(())
/// }).expect("Error");
/// # }
//noinspection DuplicatedCode
#[proc_macro_derive(Mutation)]
pub fn derive_mutation(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;

    let generate = format!("{}Mut", name);
    let generate_ident = Ident::new(&generate, name.span());

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        return quote_spanned! { name.span() => compile_error!("Only structures with named fields are supported.") }.into();
    };

    let lifetime = Lifetime::new("'mutation", generics.span());
    let generics_gn = generics.params.iter();
    let generics_with_lt = quote! {
        <#lifetime, #(#generics_gn,)*>
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

    let expanded_cloned = expanded.clone();

    let q = quote::quote! {
        /// Do not have an explicit implementation for this structure.
        pub struct #generate_ident #generics_with_lt {
            #(#destruction,)*
        }

        impl #generics #name #generics {
            pub fn substitute(&mut self, mut f: impl FnOnce(&mut #generate_ident #generics)) {
                f(&mut #generate_ident {
                    #(#expanded,)*
                })
            }

            pub fn try_substitute<E>(&mut self, mut f: impl FnOnce(&mut #generate_ident #generics) -> Result<(), E>) -> Result<(), E> {
                f(&mut #generate_ident {
                    #(#expanded_cloned,)*
                })
            }
        }
    };

    q.into()
}
