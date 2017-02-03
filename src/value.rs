use std::convert::From;
use std::collections::HashMap;
use std::borrow::Cow;

/// A configuration value.
///
/// Has an underlying or native type that comes from the configuration source
/// but will be coerced into the requested type.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Table(HashMap<String, Value>),
    Array(Vec<Value>),
}

impl Value {
    /// Gets the underlying value as a string, performing a conversion only if neccessary.
    #[allow(needless_lifetimes)]
    pub fn as_str<'a>(&'a self) -> Option<Cow<'a, str>> {
        match *self {
            Value::String(ref value) => Some(Cow::Borrowed(value)),
            Value::Integer(value) => Some(Cow::Owned(value.to_string())),
            Value::Float(value) => Some(Cow::Owned(value.to_string())),
            Value::Boolean(value) => Some(Cow::Owned(value.to_string())),

            _ => None,
        }
    }

    /// Gets the underlying type as a boolean, performing a conversion only if neccessary.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
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

    /// Gets the underlying type as an integer, performing a conversion only if neccessary.
    pub fn as_int(&self) -> Option<i64> {
        match *self {
            Value::Integer(value) => Some(value),
            Value::String(ref value) => value.parse().ok(),
            Value::Boolean(value) => Some(if value { 1 } else { 0 }),
            Value::Float(value) => Some(value.round() as i64),

            _ => None,
        }
    }

    /// Gets the underlying type as a floating-point, performing a conversion only if neccessary.
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(value) => Some(value),
            Value::String(ref value) => value.parse().ok(),
            Value::Integer(value) => Some(value as f64),
            Value::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),

            _ => None,
        }
    }

    /// Gets the underlying type as a map; only works if the type is actually a map.
    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        match *self {
            Value::Table(ref value) => Some(value),
            _ => None
        }
    }
    /// Gets the underlying type as a slice; only works if the type is actually a slice.
    pub fn as_slice(&self) -> Option<&[Value]> {
        match *self {
            Value::Array(ref value) => Some(value),
            _ => None
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

// impl From<HashMap<String, Value>> for Value {
//     fn from(value: HashMap<String, Value>) -> Value {
//         Value::Table(value)
//     }
// }

impl<T> From<HashMap<String, T>> for Value where T: Into<Value> {
    fn from(values: HashMap<String, T>) -> Value {
        let mut r = HashMap::new();

        for (k, v) in values {
            r.insert(k.clone(), v.into());
        }

        Value::Table(r)
    }
}

impl<T> From<Vec<T>> for Value where T: Into<Value> {
    fn from(values: Vec<T>) -> Value {
        let mut l = Vec::new();

        for v in values {
            l.push(v.into());
        }

        Value::Array(l)
    }
}
