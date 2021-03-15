//! Implements the functionality to enable conversion between a struct type a map container type
//! in Rust through the use of a procedural macros.
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Type};

use std::collections::HashMap;

/// Implements the functionality for converting entries in a HashMap into attributes and values of a
/// struct. It will consume a tokenized version of the initial struct declaration, and use code
/// generation to implement the `FromMap` trait for instantiating the contents of the struct.
#[proc_macro_derive(FromMap)]
pub fn from_map(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    // parse out all the field names in the struct as `Ident`s
    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => panic!("Implementation must be a struct"),
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

    // parse out all the primitive types in the struct as Idents
    let typecalls: Vec<Ident> = fields
        .iter()
        .map(|field| match field.ty.clone() {
            Type::Path(typepath) => {
                // TODO: options and results
                // TODO: vecs
                // TODO: genericized numerics

                // get the type of the specified field, lowercase
                let typename: String = quote! {#typepath}.to_string().to_lowercase();

                // TODO: exit on unsupported types

                // initialize new Ident for codegen
                Ident::new(&typename, Span::mixed_site())
            }
            _ => unimplemented!(),
        })
        .collect::<Vec<Ident>>();

    // get the name identifier of the struct input AST
    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // start codegen of a generic or non-generic impl for the given struct using quasi-quoting
    let tokens = quote! {
        use structmap::value::Value;
        use structmap::{StringMap, GenericMap};

        impl #impl_generics FromMap for #name #ty_generics #where_clause {

            fn from_stringmap(mut hashmap: StringMap) -> #name {
                let mut settings = #name::default();
                /* TODO
                #(
                    match hashmap.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(entry) => {
                            let value = match entry.get() {
                                Some(val) => val.parse::<#typecalls>().unwrap(),
                                None => unreachable!()
                            };
                            settings.#idents = value;
                        },
                        _ => unreachable!()
                    }
                )*
                */
                settings
            }

            fn from_genericmap(mut hashmap: GenericMap) -> #name {
                let mut settings = #name::default();
                #(
                    match hashmap.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(entry) => {
                            // parse out primitive value from generic type using typed call
                            let value = match entry.get().#typecalls() {
                                Some(val) => val,
                                None => unreachable!()
                            };
                            settings.#idents = value;
                        },
                        _ => unreachable!()
                    }
                )*
                settings
            }
        }
    };
    TokenStream::from(tokens)
}

/// Converts a given input struct into a HashMap where the keys are the attribute names assigned to
/// the values of the entries.
#[proc_macro_derive(ToMap, attributes(rename))]
pub fn to_map(input_struct: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input_struct as DeriveInput);

    // check for struct type and parse out fields
    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => panic!("Implementation must be a struct"),
    };

    // before unrolling out more, get mapping of any renaming needed to be done
    let rename_map = parse_rename_attrs(&fields);

    // parse out all the field names in the struct as `Ident`s
    let idents: Vec<&Ident> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    // convert all the field names into strings
    let keys: Vec<String> = idents
        .clone()
        .iter()
        .map(|ident| ident.to_string())
        .map(|name| match rename_map.contains_key(&name) {
            true => rename_map.get(&name).unwrap().clone(),
            false => name,
        })
        .collect::<Vec<String>>();

    // get the name identifier of the struct input AST
    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // start codegen for to_hashmap functionality that converts a struct into a hashmap
    let tokens = quote! {

        impl #impl_generics ToMap for #name #ty_generics #where_clause {

            fn to_stringmap(mut input_struct: #name) -> StringMap {
                let mut map = StringMap::new();
                #(
                    map.insert(#keys.to_string(), input_struct.#idents.to_string());
                )*
                map
            }

            fn to_genericmap(mut input_struct: #name) -> GenericMap {
                let mut map = GenericMap::new();
                #(
                    map.insert(#keys.to_string(), Value::new(input_struct.#idents));
                )*
                map
            }
        }
    };
    TokenStream::from(tokens)
}

/// Helper method used to parse out any `rename` attribute definitions in a struct
/// marked with the ToMap trait, returning a mapping between the original field name
/// and the one being changed for later use when doing codegen.
fn parse_rename_attrs(fields: &Fields) -> HashMap<String, String> {
    let mut rename: HashMap<String, String> = HashMap::new();
    match fields {
        Fields::Named(_) => {
            // iterate over fields available and attributes
            for field in fields.iter() {
                for attr in field.attrs.iter() {
                    // parse original struct field name
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    if rename.contains_key(&field_name) {
                        panic!("Cannot redefine field name multiple times");
                    }

                    // parse out name value pairs in attributes
                    // first get `lst` in #[rename(lst)]
                    match attr.parse_meta() {
                        Ok(syn::Meta::List(lst)) => {
                            // then parse key-value name
                            match lst.nested.first() {
                                Some(syn::NestedMeta::Meta(syn::Meta::NameValue(nm))) => {
                                    // check path to be = `name`
                                    let path = nm.path.get_ident().unwrap().to_string();
                                    if path != "name" {
                                        panic!("Must be `#[rename(name = 'VALUE')]`");
                                    }

                                    let lit = match &nm.lit {
                                        syn::Lit::Str(val) => val.value(),
                                        _ => {
                                            panic!("Must be `#[rename(name = 'VALUE')]`");
                                        }
                                    };
                                    rename.insert(field_name, lit);
                                }
                                _ => {
                                    panic!("Must be `#[rename(name = 'VALUE')]`");
                                }
                            }
                        }
                        _ => {
                            panic!("Must be `#[rename(name = 'VALUE')]`");
                        }
                    }
                }
            }
        }
        _ => {
            panic!("Must have named fields");
        }
    }
    rename
}
