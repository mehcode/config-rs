use std::error::Error;

use crate::format;
use crate::map::Map;
use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a YAML input from the provided text
    let value = from_yaml_value(uri, serde_yaml::from_str(text)?);
    format::extract_root_table(uri, value)
}

pub fn from_yaml_value(uri: Option<&String>, value: serde_yaml::Value) -> Value {
    let vk = match value {
        serde_yaml::Value::Tagged(_) | serde_yaml::Value::Null => ValueKind::Nil,
        serde_yaml::Value::Bool(v) => ValueKind::Boolean(v),
        serde_yaml::Value::String(v) => ValueKind::String(v),

        serde_yaml::Value::Number(v) => {
            if v.is_i64() {
                ValueKind::I64(v.as_i64().expect("i64"))
            } else if v.is_u64() {
                ValueKind::U64(v.as_u64().expect("u64"))
            } else if v.is_f64() {
                ValueKind::Float(v.as_f64().expect("f64"))
            } else {
                ValueKind::Nil
            }
        }

        serde_yaml::Value::Mapping(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| {
                    let key = match k {
                        serde_yaml::Value::Number(v) => v.to_string(),
                        serde_yaml::Value::String(v) => v,

                        _ => unreachable!(),
                    };
                    let value = from_yaml_value(uri, v);
                    (key, value)
                })
                .collect();

            ValueKind::Table(m)
        }

        serde_yaml::Value::Sequence(array) => {
            let l = array.into_iter().map(|v| from_yaml_value(uri, v)).collect();

            ValueKind::Array(l)
        }
    };

    Value::new(uri, vk)
}
