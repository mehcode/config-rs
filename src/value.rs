use std::convert::From;
use std::collections::HashMap;
use std::borrow::Cow;

/// A configuration value.
///
/// Has an underlying or native type that comes from the configuration source
/// but will be coerced into the requested type.
#[derive(Debug, Clone)]
pub enum Value<'a> {
    String(Cow<'a, str>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Table(HashMap<String, Value<'a>>),
    Array(Vec<Value<'a>>),
}

impl<'a> Value<'a> {
    /// Gets the underlying value as a string, performing a conversion only if neccessary.
    pub fn as_str(&'a self) -> Option<Cow<'a, str>> {
        match *self {
            Value::String(ref value) => Some(Cow::Borrowed(&*value)),
            Value::Integer(value) => Some(value.to_string().into()),
            Value::Float(value) => Some(value.to_string().into()),
            Value::Boolean(value) => Some(value.to_string().into()),

            _ => unimplemented!(),
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

            _ => unimplemented!(),
        }
    }

    /// Gets the underlying type as an integer, performing a conversion only if neccessary.
    pub fn as_int(&self) -> Option<i64> {
        match *self {
            Value::Integer(value) => Some(value),
            Value::String(ref value) => value.parse().ok(),
            Value::Boolean(value) => Some(if value { 1 } else { 0 }),
            Value::Float(value) => Some(value.round() as i64),

            _ => unimplemented!(),
        }
    }

    /// Gets the underlying type as a floating-point, performing a conversion only if neccessary.
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(value) => Some(value),
            Value::String(ref value) => value.parse().ok(),
            Value::Integer(value) => Some(value as f64),
            Value::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),

            _ => unimplemented!(),
        }
    }
}

// Generalized construction from type into variant is needed
// for setting configuration values

impl<'a> From<String> for Value<'a> {
    fn from(value: String) -> Value<'a> {
        Value::String(value.into())
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Value<'a> {
        Value::String(value.into())
    }
}

impl<'a> From<i64> for Value<'a> {
    fn from(value: i64) -> Value<'a> {
        Value::Integer(value)
    }
}

impl<'a> From<f64> for Value<'a> {
    fn from(value: f64) -> Value<'a> {
        Value::Float(value)
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Value<'a> {
        Value::Boolean(value)
    }
}
