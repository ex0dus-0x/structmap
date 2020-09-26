//! Implements the functionality to enable conversion between a struct type a map container type
//! in Rust through the use of a procedural macros.
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

use std::fmt;
use std::any::Any;

/// Represents primitive types that are supported for conversion into a HashMap that can support
/// heterogeneous values. Inspired by `serde_json::Value`s.
#[derive(Debug, Clone)]
enum Value<'a> {
    Null,
    Bool(bool),
    Int(i32),
    UInt(u32),
    String(&'a str),
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a> Value<'a> {
    pub fn to_value<T: Any>(value: T) -> Value<'a> {
        let any_val = &value as &dyn Any;
        if let Some(val) = any_val.downcast_ref::<bool>() {
            Value::Bool(*val)
        }
        else if let Some(val) = any_val.downcast_ref::<i32>() {
            Value::Int(*val)
        }
        else if let Some(val) = any_val.downcast_ref::<u32>() {
            Value::UInt(*val)
        }
        else if let Some(val) = any_val.downcast_ref::<&'static str>() {
            Value::String(val)
        } else {
            Value::Null
        }
    }

    pub fn from_value<T: Any>(value: Value<'a>) -> T {
        todo!()
    }
}


/// Implements the functionality for converting entries in a HashMap into attributes and values of a
/// struct. It will consume a tokenized version of the initial struct declaration, and use code
/// generation to implement the `FromHashMap` trait for instantiating the contents of the struct.
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
                Err(_) => panic!(format!("Unable to convert input to type")),
            }
        }
    };
    TokenStream::from(tokens)
}


/// Converts a given input struct into a HashMap where the keys are the attribute names assigned to
/// the values of the entries.
#[proc_macro_derive(ToHashMap)]
pub fn to_hashmap(input_struct: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input_struct as DeriveInput);

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

    // start codegen for to_hashmap functionality that converts a struct into a hashmap
    let tokens = quote! {
        impl #impl_generics ToHashMap for #name #ty_generics #where_clause {
            fn to_hashmap(mut input_struct: #name) -> ::std::collections::HashMap<String, String> {
                let mut hm: ::std::collections::HashMap<String, String> = ::std::collections::HashMap::new();
                #(
                    hm.insert(#keys.to_string(), input_struct.#idents);
                )*
                hm
            }
        }
    };
    TokenStream::from(tokens)
}

#[proc_macro_attribute]
pub fn rename(attr: TokenStream, item: TokenStream) -> TokenStream {
    // TODO: parse out the attribute name and internally rewrite it for HashMap
    todo!()
}
