//! Implements the main functions to enable conversion between a struct type a map container type
//! in Rust through the use of a procedural macros.
#![recursion_limit = "128"]

use syn::{Expr, Ident, Member};
use syn::parse::ParseStream;
use proc_macro::TokenStream;

use std::collections::HashMap;


trait FromHashMap<T, U> {
    fn from_hashmap(hashmap: HashMap<String, T>) -> Self;
}

trait ToHashMap<T> {
    fn to_hashmap(structure: ParseStream) -> HashMap<String, String>;
}


#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap(input: TokenStream) -> TokenStream {

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


    // dynamic code generation time, given the set of fields parsed out
    let struct_name


    todo!()
}


#[proc_macro_derive(ToHashMap)]
pub fn to_hashmap(input: TokenStream) -> TokenStream {
    todo!()
}
