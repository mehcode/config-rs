use std::convert::From;
use std::borrow::Cow;

/// A configuration value.
///
/// Has an underlying or native type that comes from the configuration source
/// but will be coerced into the requested type.
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    /// Gets the underyling value as a string, performing a conversion only if neccessary.
    #[allow(needless_lifetimes)]
    pub fn as_str<'a>(&'a self) -> Option<Cow<'a, str>> {
        if let Value::String(ref value) = *self {
            Some(Cow::Borrowed(value))
        } else if let Value::Integer(value) = *self {
            Some(Cow::Owned(value.to_string()))
        } else if let Value::Float(value) = *self {
            Some(Cow::Owned(value.to_string()))
        } else if let Value::Boolean(value) = *self {
            Some(Cow::Owned(value.to_string()))
        } else {
            None
        }
    }

    /// Gets the underlying type as a boolean, performing a conversion only if neccessary.
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(value) = *self {
            Some(value)
        } else if let Value::String(ref value) = *self {
            match value.to_lowercase().as_ref() {
                "1" | "true" | "on" | "yes" => Some(true),
                "0" | "false" | "off" | "no" => Some(false),
                _ => None,
            }
        } else if let Value::Integer(value) = *self {
            Some(value != 0)
        } else if let Value::Float(value) = *self {
            Some(value != 0.0)
        } else {
            None
        }
    }

    /// Gets the underlying type as an integer, performing a conversion only if neccessary.
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Integer(value) = *self {
            Some(value)
        } else if let Value::String(ref value) = *self {
            value.parse().ok()
        } else if let Value::Boolean(value) = *self {
            Some(if value { 1 } else { 0 })
        } else if let Value::Float(value) = *self {
            Some(value.round() as i64)
        } else {
            None
        }
    }

    /// Gets the underlying type as a floating-point, performing a conversion only if neccessary.
    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(value) = *self {
            Some(value)
        } else if let Value::String(ref value) = *self {
            value.parse().ok()
        } else if let Value::Integer(value) = *self {
            Some(value as f64)
        } else if let Value::Boolean(value) = *self {
            Some(if value { 1.0 } else { 0.0 })
        } else {
            None
        }
    }
}

// Generalized construction from type into variant is needed
// for setting configuration values

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
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
