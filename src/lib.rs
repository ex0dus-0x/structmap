//! Implements the public traits that developers inherit from in order to properly utilize the
//! derive macro's functionality in code conversion and generation.
#![doc = include_str!("../README.md")]
//!
pub mod value;

use std::collections::BTreeMap;

use crate::value::Value;

// Alias for BTreeMap with String keys and values
pub type StringMap = BTreeMap<String, String>;

// Alias for BTreeMap with String keys and generic values
pub type GenericMap = BTreeMap<String, Value>;

pub trait FromMap: Default {
    /// Converts a `GenericMap` back into a structure.
    /// __Constraints__: assumes that value types conform to the original types of the struct.
    fn from_genericmap(hashmap: GenericMap) -> Self;
}

pub trait ToMap: Default {
    /// Generates a `StringMap` where value types are all casted to strings.
    /// __Constraints__: one-way, will need additional work to re-convert to struct.
    #[allow(clippy::wrong_self_convention)]
    fn to_stringmap(structure: Self) -> StringMap;

    /// Generates a `GenericMap` where value types are all encapsulated under a sum type.
    /// __Constraints__: currently only supports primitive types for genericized values.
    #[allow(clippy::wrong_self_convention)]
    fn to_genericmap(structure: Self) -> GenericMap;
}
