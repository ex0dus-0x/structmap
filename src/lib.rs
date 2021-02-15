//! Implements the public traits that developers inherit from in order to properly utilize the
//! derive macro's functionality in code conversion and generation.

pub mod value;

use crate::value::Value;
use std::collections::{BTreeMap, HashMap};

// Alias for HashMap with String keys and values
pub type StringMap = HashMap<String, String>;

// Alias for HashMap with String keys and generic values
pub type GenericMap = HashMap<String, Value>;

pub trait MapTrait<K, V> {}
impl<K: Eq + Hash MapTrait for HashMap<K, V> {}
impl MapTrait for BTreeMap<K, V> {}

pub trait FromMap: Default {
    fn from_map<T: MapTrait>(hashmap: T) -> Self;
    fn from_stringmap(hashmap: StringMap) -> Self;
    fn from_genericmap(hashmap: GenericMap) -> Self;
    //fn from_btreemap(btreemap: BTreeMap<String, Value>) -> Self;
}

#[allow(clippy::wrong_self_convention)]
pub trait ToMap: Default {
    fn to_stringmap(structure: Self) -> StringMap;
    fn to_genericmap(structure: Self) -> GenericMap;
    //fn to_btreemap(structure: Self) -> BTreeMap<String, Value>;
}
