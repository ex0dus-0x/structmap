//! Implements the main functions to enable conversion between a struct type a map container type
//! in Rust through the use of a procedural macros.
#![recursion_limit = "128"]

use syn::{Field, Ident, Result, Token, Expr, Member, Generics};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use quote::quote;

use proc_macro::TokenStream;
use std::collections::HashMap;


trait FromHashMap<T>: Default {
    fn from_hashmap(hashmap: HashMap<String, String>) -> T;
}

trait ToHashMap {
    fn to_hashmap(structure: Self) -> HashMap<String, String>;
}

struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: syn::token::Brace,
    fields: Punctuated<Field, Token![,]>,
    generics: Generics,
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: syn::braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
            generics: input.parse()?,
        })
    }
}

#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(input as ItemStruct);

	// parse out all the field names in the struct
	let idents: Vec<Ident> = ast.fields
		.iter()
		.filter_map(|field| field.ident)
		.collect::<Vec<Ident>>();

    let keys: Vec<String> = idents.clone()
        .iter()
        .map(|ident| String::from(ident.as_ref()))
        .collect::<Vec<String>>();

	// get the name identifier of the struct input AST
	let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

	// start codegen of the impl for the given struct using quasi-quoting
    let tokens = quote! {
        impl FromHashMap<#name> for #name #ty_generics #where_clause {
            fn parse_pair<T>(v: &str) -> T where T: ::std::str::FromStr {
                let res = v.parse::<T>();
                match res {
                    Ok(val) => val,
                    Err(_) => panic!(format!("Unable to convert inpu to type")),
                }
            }

            fn from_hashmap(mut hashmap: ::std::collections::HashMap<String, String>) -> #name {
                let mut settings = #name::default();
                #(
                    match hashmap.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(entry) => {
                            settings.#idents = #name::parse_pair(entry.get().as_str());
                        },
                        ::std::collections::hash_map::Entry::Vacant(_) => {},
                    }
                )*
                settings
            }
        }
    };
    TokenStream::from(tokens)
}


#[proc_macro_derive(ToHashMap)]
pub fn to_hashmap(hashmap: TokenStream) -> TokenStream {
	// turn source into a parsable string for AST conversion
    let source: String = hashmap.to_string();
    println!("{}", source);
    let ast: Expr = syn::parse_str::<Expr>(&source).unwrap();

    // given a struct, parse out fields intriniscally
    let fields: Vec<Ident> = match ast {
        Expr::Struct(st) => {
            st.fields
                .iter()
                .filter_map(|field| match &field.member {
                    Member::Named(v) => Some(v.clone()),
                    _ => None
                })
                .collect::<Vec<Ident>>()
        },
        Expr::Tuple(_) => {
            panic!("tuple are not (yet) supported");
        },
        _ => {
            panic!("does not support non-product abstract data types");
        }
    };

    todo!()
}
