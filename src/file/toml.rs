use toml;
use source::Source;
use std::borrow::Cow;
use std::error::Error;
use value::Value;

pub struct Content {
    // Root table of the TOML document
    root: toml::Value,
}

impl Content {
    pub fn parse(text: &str) -> Result<Box<Source>, Box<Error>> {
        // Parse
        let mut parser = toml::Parser::new(text);
        // TODO: Get a solution to make this return an Error-able
        let root = parser.parse().unwrap();

        Ok(Box::new(Content { root: toml::Value::Table(root) }))
    }
}

fn from_toml_value<'a>(value: &toml::Value) -> Option<Cow<'a, Value>> {
    match *value {
        toml::Value::String(ref value) => Some(Cow::Owned(Value::String(Cow::Borrowed(value)))),
        toml::Value::Float(value) => Some(Cow::Owned(Value::Float(value))),
        toml::Value::Integer(value) => Some(Cow::Owned(Value::Integer(value))),
        toml::Value::Boolean(value) => Some(Cow::Owned(Value::Boolean(value))),

        _ => None,
    }
}

impl Source for Content {
    fn get<'a>(&self, key: &str) -> Option<Cow<'a, Value>> {
        // TODO: Key segment iteration is not something that should be here directly
        let key_delim = '.';
        let key_segments = key.split(key_delim);
        let mut toml_cursor = &self.root;
        for segment in key_segments {
            match *toml_cursor {
                toml::Value::Table(ref table) => {
                    if let Some(value) = table.get(segment) {
                        toml_cursor = value;
                    }
                }

                _ => {
                    // This is not a table or array
                    // Traversal is not possible
                    return None;
                }
            }
        }

        from_toml_value(toml_cursor)
    }
}
