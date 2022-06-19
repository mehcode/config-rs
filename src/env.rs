use std::env;

use crate::error::Result;
use crate::map::Map;
use crate::source::Source;
use crate::value::{Value, ValueKind};

#[must_use]
#[derive(Clone, Debug, Default)]
pub struct Environment {
    /// Optional prefix that will limit access to the environment to only keys that
    /// begin with the defined prefix.
    ///
    /// A prefix with a separator of `_` is tested to be present on each key before its considered
    /// to be part of the source environment.
    ///
    /// For example, the key `CONFIG_DEBUG` would become `DEBUG` with a prefix of `config`.
    prefix: Option<String>,

    /// Optional character sequence that separates the prefix from the rest of the key
    prefix_separator: Option<String>,

    /// Optional character sequence that separates each key segment in an environment key pattern.
    /// Consider a nested configuration such as `redis.password`, a separator of `_` would allow
    /// an environment key of `REDIS_PASSWORD` to match.
    separator: Option<String>,

    /// Optional character sequence that separates each env value into a vector. only works when try_parsing is set to true
    /// Once set, you cannot have type String on the same environment, unless you set list_parse_keys.
    list_separator: Option<String>,
    /// A list of keys which should always be parsed as a list. If not set you can have only Vec<String> or String (not both) in one environment.
    list_parse_keys: Option<Vec<String>>,

    /// Ignore empty env values (treat as unset).
    ignore_empty: bool,

    /// Parses booleans, integers and floats if they're detected (can be safely parsed).
    try_parsing: bool,

    // Preserve the prefix while parsing
    keep_prefix: bool,

    /// Alternate source for the environment. This can be used when you want to test your own code
    /// using this source, without the need to change the actual system environment variables.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use config::{Environment, Config};
    /// # use serde::Deserialize;
    /// # use std::collections::HashMap;
    /// # use std::convert::TryInto;
    /// #
    /// #[test]
    /// fn test_config() -> Result<(), config::ConfigError> {
    ///   #[derive(Clone, Debug, Deserialize)]
    ///   struct MyConfig {
    ///     pub my_string: String,
    ///   }
    ///
    ///   let source = Environment::default()
    ///     .source(Some({
    ///       let mut env = HashMap::new();
    ///       env.insert("MY_STRING".into(), "my-value".into());
    ///       env
    ///   }));
    ///
    ///   let config: MyConfig = Config::builder()
    ///     .add_source(source)
    ///     .build()?
    ///     .try_into()?;
    ///   assert_eq!(config.my_string, "my-value");
    ///
    ///   Ok(())
    /// }
    /// ```
    source: Option<Map<String, String>>,
}

impl Environment {
    pub fn with_prefix(s: &str) -> Self {
        Self {
            prefix: Some(s.into()),
            ..Self::default()
        }
    }

    pub fn prefix(mut self, s: &str) -> Self {
        self.prefix = Some(s.into());
        self
    }

    pub fn prefix_separator(mut self, s: &str) -> Self {
        self.prefix_separator = Some(s.into());
        self
    }

    pub fn separator(mut self, s: &str) -> Self {
        self.separator = Some(s.into());
        self
    }

    /// When set and try_parsing is true, then all environment variables will be parsed as [`Vec<String>`] instead of [`String`].
    /// See [`with_list_parse_key`] when you want to use [`Vec<String>`] in combination with [`String`].
    pub fn list_separator(mut self, s: &str) -> Self {
        self.list_separator = Some(s.into());
        self
    }

    /// Add a key which should be parsed as a list when collecting [`Value`]s from the environment.
    /// Once list_separator is set, the type for string is [`Vec<String>`].
    /// To switch the default type back to type Strings you need to provide the keys which should be [`Vec<String>`] using this function.
    pub fn with_list_parse_key(mut self, key: &str) -> Self {
        if self.list_parse_keys == None {
            self.list_parse_keys = Some(vec![key.into()])
        } else {
            self.list_parse_keys = self.list_parse_keys.map(|mut keys| {
                keys.push(key.into());
                keys
            });
        }
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

    pub fn keep_prefix(mut self, keep: bool) -> Self {
        self.keep_prefix = keep;
        self
    }

    pub fn source(mut self, source: Option<Map<String, String>>) -> Self {
        self.source = source;
        self
    }
}

impl Source for Environment {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>> {
        let mut m = Map::new();
        let uri: String = "the environment".into();

        let separator = self.separator.as_deref().unwrap_or("");
        let prefix_separator = match (self.prefix_separator.as_deref(), self.separator.as_deref()) {
            (Some(pre), _) => pre,
            (None, Some(sep)) => sep,
            (None, None) => "_",
        };

        // Define a prefix pattern to test and exclude from keys
        let prefix_pattern = self
            .prefix
            .as_ref()
            .map(|prefix| format!("{}{}", prefix, prefix_separator).to_lowercase());

        let collector = |(key, value): (String, String)| {
            // Treat empty environment variables as unset
            if self.ignore_empty && value.is_empty() {
                return;
            }

            let mut key = key.to_lowercase();

            // Check for prefix
            if let Some(ref prefix_pattern) = prefix_pattern {
                if key.starts_with(prefix_pattern) {
                    if !self.keep_prefix {
                        // Remove this prefix from the key
                        key = key[prefix_pattern.len()..].to_string();
                    }
                } else {
                    // Skip this key
                    return;
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
                } else if let Some(separator) = &self.list_separator {
                    if let Some(keys) = &self.list_parse_keys {
                        if keys.contains(&key) {
                            let v: Vec<Value> = value
                                .split(separator)
                                .map(|s| Value::new(Some(&uri), ValueKind::String(s.to_string())))
                                .collect();
                            ValueKind::Array(v)
                        } else {
                            ValueKind::String(value)
                        }
                    } else {
                        let v: Vec<Value> = value
                            .split(separator)
                            .map(|s| Value::new(Some(&uri), ValueKind::String(s.to_string())))
                            .collect();
                        ValueKind::Array(v)
                    }
                } else {
                    ValueKind::String(value)
                }
            } else {
                ValueKind::String(value)
            };

            m.insert(key, Value::new(Some(&uri), value));
        };

        match &self.source {
            Some(source) => source.clone().into_iter().for_each(collector),
            None => env::vars().for_each(collector),
        }

        Ok(m)
    }
}
