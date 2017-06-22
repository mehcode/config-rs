use std::env;
use std::collections::HashMap;
use error::*;
use source::Source;
use value::{Value, ValueKind};

#[derive(Clone, Debug)]
pub struct Environment {
    /// Optional prefix that will limit access to the environment to only keys that
    /// begin with the defined prefix.
    ///
    /// A prefix, followed by `_` (the seperator),
    /// is tested to be present on each key before its considered
    /// to be part of the source environment.
    ///
    /// For example, the key `CONFIG_DEBUG` would become `DEBUG` with a prefix of `config`.
    prefix: Option<String>,

    /// The character sequence that separates each key segment in an environment key pattern.
    /// Consider a nested configuration such as `redis.password`, a separator of `_` would allow
    /// an environment key of `REDIS_PASSWORD` to match.
    ///
    /// The default separator is `_`.
    separator: String,
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }

    pub fn with_prefix(s: &str) -> Self {
        Environment {
            prefix: Some(s.into()),
            ..Environment::default()
        }
    }

    pub fn prefix(&mut self, s: String) -> &mut Self {
        self.prefix = s.into();
        self
    }

    pub fn separator(&mut self, s: String) -> &mut Self {
        self.separator = s;
        self
    }
}

impl Default for Environment {
    fn default() -> Environment {
        Environment {
            prefix: None,
            separator: "_".into(),
        }
    }
}

impl Source for Environment {
    fn clone_into_box(&self) -> Box<Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        let mut m = HashMap::new();
        let uri: String = "the environment".into();

        // Define a prefiux pattern to test and exclude from keys
        let prefix_pattern = match self.prefix {
            Some(ref prefix) => Some(prefix.clone() + &self.separator),
            _ => None,
        };

        for (key, value) in env::vars() {
            let mut key = key.to_string();

            // Check for prefix
            if let Some(ref prefix_pattern) = prefix_pattern {
                if key.to_lowercase().starts_with(prefix_pattern) {
                    // Remove this prefix from the key
                    key = key[prefix_pattern.len()..].to_string();
                } else {
                    // Skip this key
                    continue;
                }
            }

            // Replace `separator` with `.`
            key = key.replace(&self.separator, ".");

            m.insert(key.to_lowercase(),
                     Value::new(Some(&uri), ValueKind::String(value)));
        }

        Ok(m)
    }
}
