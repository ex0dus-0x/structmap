//! Implements the main functions to enable conversion between a struct type a map container type
//! in Rust through the use of a procedural macros.
#![recursion_limit = "128"]

use syn::{Field, Ident, Result, Token, Expr, Member};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use quote::quote;

use proc_macro::TokenStream;
use std::collections::HashMap;


trait FromHashMap {
    fn from_hashmap(structure: ParseStream, hashmap: HashMap<String, String>) -> Self;
}

trait ToHashMap {
    fn to_hashmap(structure: ParseStream, expression: ParseStream) -> HashMap<String, String>;
}

struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: syn::token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: syn::braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}

#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(input as ItemStruct);

	// parse out all the field names in the struct
	let idents: Vec<&Ident> = ast.fields
		.iter()
		.filter_map(|field| field.ident.as_ref())
		.collect::<Vec<&Ident>>();

    for item in idents {
        println!("{:?}", item);
    }

	// get the name identifier of the struct input AST
	let name: &Ident = &ast.ident;

    todo!()
    /*
	// start codegen of the impl for the given struct using quasi-quoting
    let tokens = quote! {
        impl FromHashMap<#name> for #name where {
            fn from_hashmap(mut hashmap: ::std::collections::HashMap<String, #name>) -> #name {
                let mut settings = #name::default();
                settings
            }
        }
    };
    TokenStream::from(tokens)
    */
}


#[proc_macro_derive(ToHashMap)]
pub fn to_hashmap(hashmap: TokenStream) -> TokenStream {
	// turn source into a parsable string for AST conversion
    let source: String = hashmap.to_string();
    println!("{:?}", source);
    let ast: Expr = syn::parse_str::<Expr>(&source).unwrap();

    // given a struct, parse out fields intriniscally and functionally
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
