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