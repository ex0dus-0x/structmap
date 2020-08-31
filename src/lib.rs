//! Implements the public traits that developers inherit from in order to properly utilize the
//! derive macro's functionality in code conversion and generation.

use std::collections::HashMap;

pub trait FromHashMap: Default {
    fn from_hashmap(hashmap: HashMap<String, String>) -> Self;
}

pub trait ToHashMap: Default {
    fn to_hashmap(structure: Self) -> HashMap<String, String>;
}
