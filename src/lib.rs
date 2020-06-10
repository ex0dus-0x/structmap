//! Implements the main functions to enable conversion between a struct type a map container type
//! in Rust through the use of a procedural macros.
#![recursion_limit = "128"]

use syn::{Field, Ident, Result, Token, Expr, Member};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use quote::quote;

use proc_macro::TokenStream;
use std::collections::HashMap;


trait FromHashMap<T, U> {
    fn from_hashmap(hashmap: HashMap<String, T>) -> Self;
}

trait ToHashMap<T> {
    fn to_hashmap(structure: ParseStream) -> HashMap<String, String>;
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

	// get the name identifier of the struct input AST
	let name: &Ident = &ast.ident;

	// start codegen of the impl for the given struct
	todo!()
}


#[proc_macro_derive(ToHashMap)]
pub fn to_hashmap(input: TokenStream) -> TokenStream {
	// turn source into a parsable string for AST conversion
    let source: String = input.to_string();
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
