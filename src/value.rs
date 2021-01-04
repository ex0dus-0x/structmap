use std::any::Any;
use std::fmt;

/// Represents primitive types that are supported for conversion into a HashMap that can support
/// heterogeneous values. Inspired by `serde_json::Value`s.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i32),
    UInt(u32),
    String(String),
    Array(Vec<Value>),
    // TODO: Map
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    /// Given a genericized input type, encapsulate it as a Value that can be used in a map
    /// container type when converting to and from a struct.
    pub fn new<T: Any>(value: T) -> Value {
        let any_val = &value as &dyn Any;
        if let Some(val) = any_val.downcast_ref::<bool>() {
            Value::Bool(*val)
        } else if let Some(val) = any_val.downcast_ref::<i32>() {
            Value::Int(*val)
        } else if let Some(val) = any_val.downcast_ref::<u32>() {
            Value::UInt(*val)
        } else if let Some(val) = any_val.downcast_ref::<&'static str>() {
            Value::String(val.to_string())
        } else if let Some(val) = any_val.downcast_ref::<String>() {
            Value::String(val.to_string())
        } else if let Some(val) = any_val.downcast_ref::<Vec<Value>>() {
            Value::Array(val.to_vec())
        } else {
            Value::Null
        }
    }

    /*
    pub fn to_value(self) -> Box<Any> {
        match self {
            Value::Bool(val) => Box::new(val),
            Value::Int(val) => Box::new(val),
            Value::UInt(val) => Box::new(val),
            Value::String(val) => Box::new(val),
            _ => unimplemented!(),
        }
    }
    */

    /// Helper called by procedural macro to parse out bool primitive type
    pub fn to_bool(&self) -> Option<bool> {
        if let Value::Bool(val) = self {
            Some(*val)
        } else {
            None
        }
    }

    /// Helper called by procedural macro to parse out i32 primitive type
    pub fn to_i32(&self) -> Option<i32> {
        if let Value::Int(val) = self {
            Some(*val)
        } else {
            None
        }
    }

    /// Helper called by procedural macro to parse out i32 primitive type
    pub fn to_u32(&self) -> Option<u32> {
        if let Value::UInt(val) = self {
            Some(*val)
        } else {
            None
        }
    }

    /// Helper called by procedural macro to parse out String type
    pub fn to_string(&self) -> Option<String> {
        if let Value::String(string) = self {
            Some(string.to_string())
        } else {
            None
        }
    }
}
