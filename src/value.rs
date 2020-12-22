use std::any::Any;
use std::fmt;

/// Represents primitive types that are supported for conversion into a HashMap that can support
/// heterogeneous values. Inspired by `serde_json::Value`s. Exported and copied from derive macro
/// crate to make publicly available for users.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i32),
    UInt(u32),
    String(&'static str),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    pub fn to_value<T: Any>(value: T) -> Value {
        let any_val = &value as &dyn Any;
        if let Some(val) = any_val.downcast_ref::<bool>() {
            Value::Bool(*val)
        } else if let Some(val) = any_val.downcast_ref::<i32>() {
            Value::Int(*val)
        } else if let Some(val) = any_val.downcast_ref::<u32>() {
            Value::UInt(*val)
        } else if let Some(val) = any_val.downcast_ref::<&'static str>() {
            Value::String(val)
        } else {
            Value::Null
        }
    }

    pub fn from_value<T: Any>(value: Value) -> T {
        todo!()
    }
}
