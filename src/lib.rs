//! Implements the public traits that developers inherit from in order to properly utilize the
//! derive macro's functionality in code conversion and generation.

use std::collections::{HashMap, BTreeMap};
//use std::fmt;
//use std::any::Any;

/*
/// Represents primitive types that are supported for conversion into a HashMap that can support
/// heterogeneous values. Inspired by `serde_json::Value`s. Exported and copied from derive macro
/// crate to make publicly available for users.
#[derive(Debug, Clone)]
pub enum Value<'a> {
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
*/


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
