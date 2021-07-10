use std::collections::HashMap;
use std::env;

use crate::error::*;
use crate::source::Source;
use crate::value::{Value, ValueKind};

#[derive(Clone, Debug)]
pub struct Environment {
    /// Optional prefix that will limit access to the environment to only keys that
    /// begin with the defined prefix.
    ///
    /// A prefix with a separator of `_` is tested to be present on each key before its considered
    /// to be part of the source environment.
    ///
    /// For example, the key `CONFIG_DEBUG` would become `DEBUG` with a prefix of `config`.
    prefix: Option<String>,

    /// Optional character sequence that separates each key segment in an environment key pattern.
    /// Consider a nested configuration such as `redis.password`, a separator of `_` would allow
    /// an environment key of `REDIS_PASSWORD` to match.
    separator: Option<String>,

    /// Ignore empty env values (treat as unset).
    ignore_empty: bool,

    /// Parses booleans, integers and floats if they're detected (can be safely parsed).
    try_parsing: bool,
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

    pub fn prefix(mut self, s: &str) -> Self {
        self.prefix = Some(s.into());
        self
    }

    pub fn separator(mut self, s: &str) -> Self {
        self.separator = Some(s.into());
        self
    }

    pub fn ignore_empty(mut self, ignore: bool) -> Self {
        self.ignore_empty = ignore;
        self
    }

    /// Note: enabling `try_parsing` can reduce performance it will try and parse
    /// each environment variable 3 times (bool, i64, f64)
    pub fn try_parsing(mut self, try_parsing: bool) -> Self {
        self.try_parsing = try_parsing;
        self
    }
}

impl Default for Environment {
    fn default() -> Environment {
        Environment {
            prefix: None,
            separator: None,
            ignore_empty: false,
            try_parsing: false,
        }
    }
}

impl Source for Environment {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        let mut m = HashMap::new();
        let uri: String = "the environment".into();

        let separator = self.separator.as_deref().unwrap_or("");
        let group_separator = self.separator.as_deref().unwrap_or("_");

        // Define a prefix pattern to test and exclude from keys
        let prefix_pattern = self
            .prefix
            .as_ref()
            .map(|prefix| format!("{}{}", prefix, group_separator).to_lowercase());

        for (key, value) in env::vars() {
            // Treat empty environment variables as unset
            if self.ignore_empty && value.is_empty() {
                continue;
            }

            let mut key = key.to_lowercase();

            // Check for prefix
            if let Some(ref prefix_pattern) = prefix_pattern {
                if key.starts_with(prefix_pattern) {
                    // Remove this prefix from the key
                    key = key[prefix_pattern.len()..].to_string();
                } else {
                    // Skip this key
                    continue;
                }
            }

            // If separator is given replace with `.`
            if !separator.is_empty() {
                key = key.replace(separator, ".");
            }

            let value = if self.try_parsing {
                // convert to lowercase because bool parsing expects all lowercase
                if let Ok(parsed) = value.to_lowercase().parse::<bool>() {
                    ValueKind::Boolean(parsed)
                } else if let Ok(parsed) = value.parse::<i64>() {
                    ValueKind::I64(parsed)
                } else if let Ok(parsed) = value.parse::<f64>() {
                    ValueKind::Float(parsed)
                } else {
                    ValueKind::String(value)
                }
            } else {
                ValueKind::String(value)
            };

            m.insert(key, Value::new(Some(&uri), value));
        }

        Ok(m)
    }
}
