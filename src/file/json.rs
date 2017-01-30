use serde_json;

use source::Source;
use std::error::Error;
use std::borrow::Cow;
use value::Value;

pub struct Content {
    // Root table of the TOML document
    root: serde_json::Value,
}

impl Content {
    pub fn parse(text: &str) -> Result<Box<Source>, Box<Error>> {
        // Parse
        let root = serde_json::from_str(text)?;

        Ok(Box::new(Content { root: root }))
    }
}

fn from_json_value<'a>(value: &serde_json::Value) -> Option<Cow<'a, Value>> {
    match *value {
        serde_json::Value::String(ref value) => {
            Some(Cow::Owned(Value::String(Cow::Borrowed(value))))
        }

        serde_json::Value::Number(ref value) => {
            if let Some(value) = value.as_i64() {
                Some(Cow::Owned(Value::Integer(value)))
            } else if let Some(value) = value.as_f64() {
                Some(Cow::Owned(Value::Float(value)))
            } else {
                None
            }
        }

        serde_json::Value::Bool(value) => Some(Cow::Owned(Value::Boolean(value))),

        _ => None,
    }
}

impl Source for Content {
    fn get<'a>(&self, key: &str) -> Option<Cow<'a, Value>> {
        // TODO: Key segment iteration is not something that should be here directly
        let key_delim = '.';
        let key_segments = key.split(key_delim);
        let mut json_cursor = &self.root;
        for segment in key_segments {
            match *json_cursor {
                serde_json::Value::Object(ref table) => {
                    if let Some(value) = table.get(segment) {
                        json_cursor = value;
                    }
                }

                _ => {
                    // This is not a table or array
                    // Traversal is not possible
                    return None;
                }
            }
        }

        from_json_value(json_cursor)
    }
}
