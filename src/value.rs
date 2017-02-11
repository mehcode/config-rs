use std::convert::From;
use std::collections::HashMap;

/// A configuration value.
///
/// Has an underlying or native type that comes from the configuration source
/// but will be coerced into the requested type.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Table(HashMap<String, Value>),
    Array(Vec<Value>),
}

impl Value {
    /// Converts `self` into a string, if possible.
    /// Returns None otherwise.
    pub fn into_str(self) -> Option<String> {
        match self {
            Value::String(value) => Some(value),
            Value::Integer(value) => Some(value.to_string()),
            Value::Float(value) => Some(value.to_string()),
            Value::Boolean(value) => Some(value.to_string()),

            _ => None,
        }
    }

    /// Converts `self` into a bool, if possible.
    /// Returns None otherwise.
    pub fn into_bool(self) -> Option<bool> {
        match self {
            Value::Boolean(value) => Some(value),
            Value::Integer(value) => Some(value != 0),
            Value::Float(value) => Some(value != 0.0),

            Value::String(ref value) => {
                match value.to_lowercase().as_ref() {
                    "1" | "true" | "on" | "yes" => Some(true),
                    "0" | "false" | "off" | "no" => Some(false),
                    _ => None,
                }
            }

            _ => None,
        }
    }

    /// Converts `self` into an i64, if possible.
    /// Returns None otherwise.
    pub fn into_int(self) -> Option<i64> {
        match self {
            Value::Integer(value) => Some(value),
            Value::String(ref value) => value.parse().ok(),
            Value::Boolean(value) => Some(if value { 1 } else { 0 }),
            Value::Float(value) => Some(value.round() as i64),

            _ => None,
        }
    }

    /// Converts `self` into a f64, if possible.
    /// Returns None otherwise.
    pub fn into_float(self) -> Option<f64> {
        match self {
            Value::Float(value) => Some(value),
            Value::String(ref value) => value.parse().ok(),
            Value::Integer(value) => Some(value as f64),
            Value::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),

            _ => None,
        }
    }

    /// If the `Value` is a Table, returns the associated Map.
    /// Returns None otherwise.
    pub fn into_table(self) -> Option<HashMap<String, Value>> {
        match self {
            Value::Table(value) => Some(value),
            _ => None,
        }
    }

    /// If the `Value` is an Array, returns the associated Vector.
    /// Returns None otherwise.
    pub fn into_array(self) -> Option<Vec<Value>> {
        match self {
            Value::Array(value) => Some(value),
            _ => None,
        }
    }
}

// Generalized construction from type into variant is needed
// for setting configuration values

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value.into())
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Value {
        Value::String(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Value {
        Value::Integer(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Value::Boolean(value)
    }
}

impl<T> From<HashMap<String, T>> for Value
    where T: Into<Value>
{
    fn from(values: HashMap<String, T>) -> Value {
        let mut r = HashMap::new();

        for (k, v) in values {
            r.insert(k.clone(), v.into());
        }

        Value::Table(r)
    }
}

impl<T> From<Vec<T>> for Value
    where T: Into<Value>
{
    fn from(values: Vec<T>) -> Value {
        let mut l = Vec::new();

        for v in values {
            l.push(v.into());
        }

        Value::Array(l)
    }
}
