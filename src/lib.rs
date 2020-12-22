//! Implements the public traits that developers inherit from in order to properly utilize the
//! derive macro's functionality in code conversion and generation.
pub mod value;

use std::collections::{BTreeMap, HashMap};

pub trait FromHashMap: Default {
    fn from_hashmap(hashmap: HashMap<String, String>) -> Self;
}

pub trait ToHashMap: Default {
    fn to_hashmap(structure: Self) -> HashMap<String, String>;
}

pub trait FromBTreeMap: Default {
    fn from_btreemap(btreemap: BTreeMap<String, String>) -> Self;
}

pub trait ToBTreeMap: Default {
    fn to_btreemap(structure: Self) -> BTreeMap<String, String>;
}
