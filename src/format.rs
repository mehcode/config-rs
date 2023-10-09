use std::error::Error;

use crate::error::{ConfigError, Unexpected};
use crate::map::Map;
use crate::value::{Value, ValueKind};

/// Describes a format of configuration source data
///
/// Implementations of this trait can be used to convert [`File`](crate::File) sources to configuration data.
///
/// There can be various formats, some of them provided by this library, such as JSON, Yaml and other.
/// This trait enables users of the library to easily define their own, even proprietary formats without
/// the need to alter library sources.
///
/// What is more, it is recommended to use this trait with custom [`Source`](crate::Source)s and their async counterparts.
pub trait Format {
    /// Parses provided content into configuration values understood by the library.
    ///
    /// It also allows specifying optional URI of the source associated with format instance that can facilitate debugging.
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>>;
}

// Have a proper error fire if the root of a file is ever not a Table
pub fn extract_root_table(
    uri: Option<&String>,
    value: Value,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    match value.kind {
        ValueKind::Table(map) => Ok(map),
        ValueKind::Nil => Err(Unexpected::Unit),
        ValueKind::Array(_value) => Err(Unexpected::Seq),
        ValueKind::Boolean(value) => Err(Unexpected::Bool(value)),
        ValueKind::I64(value) => Err(Unexpected::I64(value)),
        ValueKind::I128(value) => Err(Unexpected::I128(value)),
        ValueKind::U64(value) => Err(Unexpected::U64(value)),
        ValueKind::U128(value) => Err(Unexpected::U128(value)),
        ValueKind::Float(value) => Err(Unexpected::Float(value)),
        ValueKind::String(value) => Err(Unexpected::Str(value)),
    }
    .map_err(|err| ConfigError::invalid_root(uri, err))
    .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)
}

// Equivalent to ValueKind, except Table + Array store the same enum
// Useful for serde to serialize values into, then convert to Value
#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum ParsedValue {
    Nil,
    Boolean(bool),
    I64(i64),
    I128(i128),
    U64(u64),
    U128(u128),
    Float(f64),
    String(String),
    Table(Map<String, Self>),
    Array(Vec<Self>),
}

// Value wrap ValueKind values, with optional uri (origin)
pub fn from_parsed_value(uri: Option<&String>, value: ParsedValue) -> Value {
    let vk = match value {
        ParsedValue::Nil => ValueKind::Nil,
        ParsedValue::String(v) => ValueKind::String(v),
        ParsedValue::I64(v) => ValueKind::I64(v),
        ParsedValue::I128(v) => ValueKind::I128(v),
        ParsedValue::U64(v) => ValueKind::U64(v),
        ParsedValue::U128(v) => ValueKind::U128(v),
        ParsedValue::Float(v) => ValueKind::Float(v),
        ParsedValue::Boolean(v) => ValueKind::Boolean(v),
        ParsedValue::Table(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| (k, from_parsed_value(uri, v)))
                .collect();

            ValueKind::Table(m)
        }

        ParsedValue::Array(array) => {
            let l = array
                .into_iter()
                .map(|v| from_parsed_value(uri, v))
                .collect();

            ValueKind::Array(l)
        }
    };

    Value::new(uri, vk)
}
