use std::collections::HashMap;
use serde::de::Deserialize;

use error::*;
use source::Source;

use value::{Value, ValueWithKey};
use path;

enum ConfigKind {
    // A mutable configuration. This is the default.
    Mutable {
        defaults: HashMap<path::Expression, Value>,
        overrides: HashMap<path::Expression, Value>,
        sources: Vec<Box<Source + Send + Sync>>,
    },

    // A frozen configuration.
    // Configuration can no longer be mutated.
    Frozen,
}

impl Default for ConfigKind {
    fn default() -> Self {
        ConfigKind::Mutable {
            defaults: HashMap::new(),
            overrides: HashMap::new(),
            sources: Vec::new(),
        }
    }
}

/// A prioritized configuration repository. It maintains a set of
/// configuration sources, fetches values to populate those, and provides
/// them according to the source's priority.
#[derive(Default)]
pub struct Config {
    kind: ConfigKind,

    /// Root of the cached configuration.
    pub cache: Value,
}

impl Config {
    /// Merge in a configuration property source.
    pub fn merge<T>(&mut self, source: T) -> Result<()>
        where T: 'static,
              T: Source + Send + Sync
    {
        match self.kind {
            ConfigKind::Mutable { ref mut sources, .. } => {
                sources.push(Box::new(source));
            }

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen);
            }
        }

        self.refresh()
    }

    /// Refresh the configuration cache with fresh
    /// data from added sources.
    ///
    /// Configuration is automatically refreshed after a mutation
    /// operation (`set`, `merge`, `set_default`, etc.).
    pub fn refresh(&mut self) -> Result<()> {
        self.cache = match self.kind {
            // TODO: We need to actually merge in all the stuff
            ConfigKind::Mutable {
                ref overrides,
                ref sources,
                ref defaults,
            } => {
                let mut cache: Value = HashMap::<String, Value>::new().into();

                // Add defaults
                for (key, val) in defaults {
                    key.set(&mut cache, val.clone());
                }

                // Add sources
                for source in sources {
                    let props = source.collect()?;
                    for (key, val) in &props {
                        path::Expression::Identifier(key.clone()).set(&mut cache, val.clone());
                    }
                }

                // Add overrides
                for (key, val) in overrides {
                    key.set(&mut cache, val.clone());
                }

                cache
            },

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen);
            }
        };

        Ok(())
    }

    pub fn deserialize<'de, T: Deserialize<'de>>(&self) -> Result<T> {
        T::deserialize(self.cache.clone())
    }

    pub fn get<'de, T: Deserialize<'de>>(&self, key: &'de str) -> Result<T> {
        // Parse the key into a path expression
        let expr: path::Expression = key.to_lowercase().parse()?;

        // Traverse the cache using the path to (possibly) retrieve a value
        let value = expr.get(&self.cache).cloned();

        match value {
            Some(value) => {
                // Deserialize the received value into the requested type
                T::deserialize(ValueWithKey::new(value, key))
            }

            None => Err(ConfigError::NotFound(key.into())),
        }
    }

    pub fn set_default<T>(&mut self, key: &str, value: T) -> Result<()>
        where T: Into<Value>
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut defaults,
                ..
            } => {
                defaults.insert(key.parse()?, value.into());
            }

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen)
            }
        };

        self.refresh()
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<()>
        where T: Into<Value>
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut overrides,
                ..
            } => {
                overrides.insert(key.parse()?, value.into());
            }

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen)
            }
        };

        self.refresh()
    }
}
