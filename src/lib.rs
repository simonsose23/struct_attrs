extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Attributes)]
pub fn attrs(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);

    let ident = parsed_input.ident;

    let mut field_names: Vec<String> = Vec::new();
    let mut field_types: Vec<String> = Vec::new();

    match parsed_input.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(named) => {
                dbg!("we have named fields");
                for field in named.named.iter() {
                    field_names.push(field.to_owned().ident.unwrap().to_string());
                    field_types.push((&field.ty).into_token_stream().to_string());
                }

                ()
            }
            _ => {}
        },
        _ => {}
    }

    dbg!(&field_names);
    dbg!(&field_types);

    let expanded = quote! {
        impl #ident {
            pub fn attrs() -> ( Vec<String>, Vec<String> ) {
                return (vec![ #(#field_names.to_string(),)* ], vec![#(#field_types.to_string(),)*])
            }
        }
    };

    dbg!(&expanded.to_string());

    expanded.into()
}

