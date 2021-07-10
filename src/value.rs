use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

use serde::de::{Deserialize, Deserializer, Visitor};

use crate::error::*;

/// Underlying kind of the configuration value.
///
/// Standard operations on a `Value` by users of this crate do not require
/// knowledge of `ValueKind`. Introspection of underlying kind is only required
/// when the configuration values are unstructured or do not have known types.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Nil,
    Boolean(bool),
    I64(i64),
    I128(i128),
    U64(u64),
    U128(u128),
    Float(f64),
    String(String),
    Table(Table),
    Array(Array),
}

pub type Array = Vec<Value>;
pub type Table = HashMap<String, Value>;

impl Default for ValueKind {
    fn default() -> Self {
        ValueKind::Nil
    }
}

impl<T> From<Option<T>> for ValueKind
where
    T: Into<ValueKind>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => ValueKind::Nil,
        }
    }
}

impl From<String> for ValueKind {
    fn from(value: String) -> Self {
        ValueKind::String(value)
    }
}

impl<'a> From<&'a str> for ValueKind {
    fn from(value: &'a str) -> Self {
        ValueKind::String(value.into())
    }
}

impl From<i8> for ValueKind {
    fn from(value: i8) -> Self {
        ValueKind::I64(value as i64)
    }
}

impl From<i16> for ValueKind {
    fn from(value: i16) -> Self {
        ValueKind::I64(value as i64)
    }
}

impl From<i32> for ValueKind {
    fn from(value: i32) -> Self {
        ValueKind::I64(value as i64)
    }
}

impl From<i64> for ValueKind {
    fn from(value: i64) -> Self {
        ValueKind::I64(value)
    }
}

impl From<i128> for ValueKind {
    fn from(value: i128) -> Self {
        ValueKind::I128(value)
    }
}

impl From<u8> for ValueKind {
    fn from(value: u8) -> Self {
        ValueKind::U64(value as u64)
    }
}

impl From<u16> for ValueKind {
    fn from(value: u16) -> Self {
        ValueKind::U64(value as u64)
    }
}

impl From<u32> for ValueKind {
    fn from(value: u32) -> Self {
        ValueKind::U64(value as u64)
    }
}

impl From<u64> for ValueKind {
    fn from(value: u64) -> Self {
        ValueKind::U64(value)
    }
}

impl From<u128> for ValueKind {
    fn from(value: u128) -> Self {
        ValueKind::U128(value)
    }
}

impl From<f64> for ValueKind {
    fn from(value: f64) -> Self {
        ValueKind::Float(value)
    }
}

impl From<bool> for ValueKind {
    fn from(value: bool) -> Self {
        ValueKind::Boolean(value)
    }
}

impl<T> From<HashMap<String, T>> for ValueKind
where
    T: Into<Value>,
{
    fn from(values: HashMap<String, T>) -> Self {
        let t = values.into_iter().map(|(k, v)| (k, v.into())).collect();
        ValueKind::Table(t)
    }
}

impl<T> From<Vec<T>> for ValueKind
where
    T: Into<Value>,
{
    fn from(values: Vec<T>) -> Self {
        ValueKind::Array(values.into_iter().map(T::into).collect())
    }
}

impl Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValueKind::String(ref value) => write!(f, "{}", value),
            ValueKind::Boolean(value) => write!(f, "{}", value),
            ValueKind::I64(value) => write!(f, "{}", value),
            ValueKind::I128(value) => write!(f, "{}", value),
            ValueKind::U64(value) => write!(f, "{}", value),
            ValueKind::U128(value) => write!(f, "{}", value),
            ValueKind::Float(value) => write!(f, "{}", value),
            ValueKind::Nil => write!(f, "nil"),
            ValueKind::Table(ref table) => write!(f, "{{ {} }}", {
                table
                    .iter()
                    .map(|(k, v)| format!("{} => {}, ", k, v))
                    .collect::<String>()
            }),
            ValueKind::Array(ref array) => write!(f, "{:?}", {
                array.iter().map(|e| format!("{}, ", e)).collect::<String>()
            }),
        }
    }
}

/// A configuration value.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Value {
    /// A description of the original location of the value.
    ///
    /// A Value originating from a File might contain:
    /// ```text
    /// Settings.toml
    /// ```
    ///
    /// A Value originating from the environment would contain:
    /// ```text
    /// the envrionment
    /// ```
    ///
    /// A Value originating from a remote source might contain:
    /// ```text
    /// etcd+http://127.0.0.1:2379
    /// ```
    origin: Option<String>,

    /// Underlying kind of the configuration value.
    pub kind: ValueKind,
}

impl Value {
    /// Create a new value instance that will remember its source uri.
    pub fn new<V>(origin: Option<&String>, kind: V) -> Self
    where
        V: Into<ValueKind>,
    {
        Value {
            origin: origin.cloned(),
            kind: kind.into(),
        }
    }

    /// Attempt to deserialize this value into the requested type.
    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T> {
        T::deserialize(self)
    }

    /// Returns `self` as a bool, if possible.
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_bool(self) -> Result<bool> {
        match self.kind {
            ValueKind::Boolean(value) => Ok(value),
            ValueKind::I64(value) => Ok(value != 0),
            ValueKind::I128(value) => Ok(value != 0),
            ValueKind::U64(value) => Ok(value != 0),
            ValueKind::U128(value) => Ok(value != 0),
            ValueKind::Float(value) => Ok(value != 0.0),

            ValueKind::String(ref value) => {
                match value.to_lowercase().as_ref() {
                    "1" | "true" | "on" | "yes" => Ok(true),
                    "0" | "false" | "off" | "no" => Ok(false),

                    // Unexpected string value
                    s => Err(ConfigError::invalid_type(
                        self.origin.clone(),
                        Unexpected::Str(s.into()),
                        "a boolean",
                    )),
                }
            }

            // Unexpected type
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "a boolean",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "a boolean",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "a boolean",
            )),
        }
    }

    /// Returns `self` into an i64, if possible.
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_int(self) -> Result<i64> {
        match self.kind {
            ValueKind::I64(value) => Ok(value),
            ValueKind::I128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I128(value),
                "an signed 64 bit or less integer",
            )),
            ValueKind::U64(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U64(value),
                "an signed 64 bit or less integer",
            )),
            ValueKind::U128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U128(value),
                "an signed 64 bit or less integer",
            )),

            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    _ => {
                        s.parse().map_err(|_| {
                            // Unexpected string
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.clone()),
                                "an integer",
                            )
                        })
                    }
                }
            }

            ValueKind::Boolean(value) => Ok(if value { 1 } else { 0 }),
            ValueKind::Float(value) => Ok(value.round() as i64),

            // Unexpected type
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer",
            )),
        }
    }

    /// Returns `self` into an i128, if possible.
    pub fn into_int128(self) -> Result<i128> {
        match self.kind {
            ValueKind::I64(value) => Ok(value as i128),
            ValueKind::I128(value) => Ok(value),
            ValueKind::U64(value) => Ok(value as i128),
            ValueKind::U128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U128(value),
                "an signed 128 bit integer",
            )),

            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    _ => {
                        s.parse().map_err(|_| {
                            // Unexpected string
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.clone()),
                                "an integer",
                            )
                        })
                    }
                }
            }

            ValueKind::Boolean(value) => Ok(if value { 1 } else { 0 }),
            ValueKind::Float(value) => Ok(value.round() as i128),

            // Unexpected type
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer",
            )),
        }
    }

    /// Returns `self` into an u64, if possible.
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_uint(self) -> Result<u64> {
        match self.kind {
            ValueKind::U64(value) => Ok(value),
            ValueKind::U128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U128(value),
                "an unsigned 64 bit or less integer",
            )),
            ValueKind::I64(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I64(value),
                "an unsigned 64 bit or less integer",
            )),
            ValueKind::I128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I128(value),
                "an unsigned 64 bit or less integer",
            )),

            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    _ => {
                        s.parse().map_err(|_| {
                            // Unexpected string
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.clone()),
                                "an integer",
                            )
                        })
                    }
                }
            }

            ValueKind::Boolean(value) => Ok(if value { 1 } else { 0 }),
            ValueKind::Float(value) => Ok(value.round() as u64),

            // Unexpected type
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer",
            )),
        }
    }

    /// Returns `self` into an u128, if possible.
    pub fn into_uint128(self) -> Result<u128> {
        match self.kind {
            ValueKind::U64(value) => Ok(value as u128),
            ValueKind::U128(value) => Ok(value),
            ValueKind::I64(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I64(value),
                "an unsigned 128 bit or less integer",
            )),
            ValueKind::I128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I128(value),
                "an unsigned 128 bit or less integer",
            )),

            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    _ => {
                        s.parse().map_err(|_| {
                            // Unexpected string
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.clone()),
                                "an integer",
                            )
                        })
                    }
                }
            }

            ValueKind::Boolean(value) => Ok(if value { 1 } else { 0 }),
            ValueKind::Float(value) => Ok(value.round() as u128),

            // Unexpected type
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer",
            )),
        }
    }

    /// Returns `self` into a f64, if possible.
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_float(self) -> Result<f64> {
        match self.kind {
            ValueKind::Float(value) => Ok(value),

            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1.0),
                    "false" | "off" | "no" => Ok(0.0),
                    _ => {
                        s.parse().map_err(|_| {
                            // Unexpected string
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.clone()),
                                "a floating point",
                            )
                        })
                    }
                }
            }

            ValueKind::I64(value) => Ok(value as f64),
            ValueKind::I128(value) => Ok(value as f64),
            ValueKind::U64(value) => Ok(value as f64),
            ValueKind::U128(value) => Ok(value as f64),
            ValueKind::Boolean(value) => Ok(if value { 1.0 } else { 0.0 }),

            // Unexpected type
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "a floating point",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "a floating point",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "a floating point",
            )),
        }
    }

    /// Returns `self` into a string, if possible.
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_string(self) -> Result<String> {
        match self.kind {
            ValueKind::String(value) => Ok(value),

            ValueKind::Boolean(value) => Ok(value.to_string()),
            ValueKind::I64(value) => Ok(value.to_string()),
            ValueKind::I128(value) => Ok(value.to_string()),
            ValueKind::U64(value) => Ok(value.to_string()),
            ValueKind::U128(value) => Ok(value.to_string()),
            ValueKind::Float(value) => Ok(value.to_string()),

            // Cannot convert
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "a string",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "a string",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "a string",
            )),
        }
    }

    /// Returns `self` into an array, if possible
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_array(self) -> Result<Vec<Value>> {
        match self.kind {
            ValueKind::Array(value) => Ok(value),

            // Cannot convert
            ValueKind::Float(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Float(value),
                "an array",
            )),
            ValueKind::String(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Str(value),
                "an array",
            )),
            ValueKind::I64(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I64(value),
                "an array",
            )),
            ValueKind::I128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I128(value),
                "an array",
            )),
            ValueKind::U64(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U64(value),
                "an array",
            )),
            ValueKind::U128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U128(value),
                "an array",
            )),
            ValueKind::Boolean(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Bool(value),
                "an array",
            )),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an array",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an array",
            )),
        }
    }

    /// If the `Value` is a Table, returns the associated Map.
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_table(self) -> Result<HashMap<String, Value>> {
        match self.kind {
            ValueKind::Table(value) => Ok(value),

            // Cannot convert
            ValueKind::Float(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Float(value),
                "a map",
            )),
            ValueKind::String(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Str(value),
                "a map",
            )),
            ValueKind::I64(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I64(value),
                "a map",
            )),
            ValueKind::I128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I128(value),
                "a map",
            )),
            ValueKind::U64(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U64(value),
                "a map",
            )),
            ValueKind::U128(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U128(value),
                "a map",
            )),
            ValueKind::Boolean(value) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Bool(value),
                "a map",
            )),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "a map",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "a map",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid configuration value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> ::std::result::Result<Value, E> {
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> ::std::result::Result<Value, E> {
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> ::std::result::Result<Value, E> {
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i128<E>(self, value: i128) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> ::std::result::Result<Value, E> {
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> ::std::result::Result<Value, E> {
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> ::std::result::Result<Value, E> {
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> ::std::result::Result<Value, E> {
                // FIXME: This is bad
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_u128<E>(self, value: u128) -> ::std::result::Result<Value, E> {
                // FIXME: This is bad
                Ok((value as i128).into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Value, E>
            where
                E: ::serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_none<E>(self) -> ::std::result::Result<Value, E> {
                Ok(Value::new(None, ValueKind::Nil))
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> ::std::result::Result<Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> ::std::result::Result<Value, E> {
                Ok(Value::new(None, ValueKind::Nil))
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> ::std::result::Result<Value, V::Error>
            where
                V: ::serde::de::SeqAccess<'de>,
            {
                let mut vec = Array::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(vec.into())
            }

            fn visit_map<V>(self, mut visitor: V) -> ::std::result::Result<Value, V::Error>
            where
                V: ::serde::de::MapAccess<'de>,
            {
                let mut values = Table::new();

                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }

                Ok(values.into())
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl<T> From<T> for Value
where
    T: Into<ValueKind>,
{
    fn from(value: T) -> Self {
        Value {
            origin: None,
            kind: value.into(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use super::ValueKind;
    use crate::Config;
    use crate::File;
    use crate::FileFormat;

    #[test]
    fn test_i64() {
        let mut c = Config::default();
        c.merge(File::new("tests/types/i64.toml", FileFormat::Toml))
            .unwrap();

        assert!(std::matches!(c.cache.kind, ValueKind::Table(_)));
        let v = match c.cache.kind {
            ValueKind::Table(t) => t,
            _ => unreachable!(),
        };

        let value = v.get("value").unwrap();
        assert!(
            std::matches!(value.kind, ValueKind::I64(120)),
            "Is not a i64(120): {:?}",
            value.kind
        );
    }
}
