use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a TOML input from the provided text
    let value = from_toml_value(uri, toml::from_str(text)?);
    format::extract_root_table(uri, value)
}

fn from_toml_value(uri: Option<&String>, value: toml::Value) -> Value {
    let vk = match value {
        toml::Value::Datetime(v) => ValueKind::String(v.to_string()),
        toml::Value::String(v)   => ValueKind::String(v),
        toml::Value::Float(v)    => ValueKind::Float(v),
        toml::Value::Integer(v)  => ValueKind::I64(v),
        toml::Value::Boolean(v)  => ValueKind::Boolean(v),

        toml::Value::Table(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| (k, from_toml_value(uri, v)))
                .collect();

            ValueKind::Table(m)
        }

        toml::Value::Array(array) => {
            let l = array
                .into_iter()
                .map(|v| from_toml_value(uri, v))
                .collect();

            ValueKind::Array(l)
        }
    };

    Value::new(uri, vk)
}
