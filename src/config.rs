use std::collections::HashMap;
use serde::de::Deserialize;

use error::*;
use source::Source;
use value::Value;
use path;

enum ConfigKind {
    // A mutable configuration. This is the default.
    Mutable {
        defaults: HashMap<String, Value>,
        overrides: HashMap<String, Value>,
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
            } => sources[0].collect()?,

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen);
            }
        };

        Ok(())
    }

    pub fn deserialize<T: Deserialize>(&self) -> Result<T> {
        return T::deserialize(self.cache.clone());
    }

    pub fn get<T: Deserialize>(&self, key: &str) -> Result<T> {
        // Parse the key into a path expression
        let expr: path::Expression = key.to_lowercase().parse()?;

        // Traverse the cache using the path to (possibly) retrieve a value
        let value = expr.get(&self.cache).cloned();

        match value {
            Some(value) => {
                // Deserialize the received value into the requested type
                T::deserialize(value)
            }

            None => Err(ConfigError::NotFound(key.into())),
        }
    }
}
