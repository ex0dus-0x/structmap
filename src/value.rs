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
    pub fn to_value<T: Any>(value: T) -> Value {
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
}
