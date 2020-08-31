//! Implements the functionality to enable conversion between a struct type a map container type
//! in Rust through the use of a procedural macros.
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Ident, Member};


/// Converts a given `HashMap` into an input struct type
#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    // parse out all the field names in the struct as `Ident`s
    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => unimplemented!(),
    };
    let idents: Vec<&Ident> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    // convert all the field names into strings
    let keys: Vec<String> = idents
        .clone()
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    // get the name identifier of the struct input AST
    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // start codegen of a generic or non-generic impl for the given struct using quasi-quoting
    let tokens = quote! {
        impl #impl_generics FromHashMap for #name #ty_generics #where_clause {
            fn from_hashmap(mut hashmap: ::std::collections::HashMap<String, String>) -> #name {
                let mut settings = #name::default();
                #(
                    match hashmap.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(entry) => {
                            settings.#idents = parse_pair(entry.get().as_str());
                        },
                        ::std::collections::hash_map::Entry::Vacant(_) => {},
                    }
                )*
                settings
            }
        }

        fn parse_pair<T>(v: &str) -> T where T: ::std::str::FromStr {
            let res = v.parse::<T>();
            match res {
                Ok(val) => val,
                Err(_) => panic!(format!("Unable to convert inpu to type")),
            }
        }
    };
    TokenStream::from(tokens)
}


#[proc_macro_derive(ToHashMap)]
pub fn to_hashmap(hashmap: TokenStream) -> TokenStream {
    // turn source into a parsable string for AST conversion
    let source: String = hashmap.to_string();
    let ast: Expr = syn::parse_str::<Expr>(&source).unwrap();

    // given a struct, parse out fields intriniscally
    let _fields: Vec<Ident> = match ast {
        Expr::Struct(st) => st
            .fields
            .iter()
            .filter_map(|field| match &field.member {
                Member::Named(v) => Some(v.clone()),
                _ => None,
            })
            .collect::<Vec<Ident>>(),
        Expr::Tuple(_) => {
            panic!("tuple are not (yet) supported");
        }
        _ => {
            panic!("does not support non-product abstract data types");
        }
    };
    todo!()
}
