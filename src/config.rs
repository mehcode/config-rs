use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::str::FromStr;

use error::*;
use ser::ConfigSerializer;
use source::Source;

use path;
use value::{Table, Value, ValueKind};

#[derive(Clone, Debug)]
enum ConfigKind {
    // A mutable configuration. This is the default.
    Mutable {
        defaults: HashMap<path::Expression, Value>,
        overrides: HashMap<path::Expression, Value>,
        sources: Vec<Box<dyn Source + Send + Sync>>,
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
#[derive(Clone, Debug)]
pub struct Config {
    kind: ConfigKind,

    /// Root of the cached configuration.
    pub cache: Value,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            kind: ConfigKind::default(),
            cache: Value::new(None, Table::new()),
        }
    }
}

impl Config {
    /// Merge in a configuration property source.
    pub fn merge<T>(&mut self, source: T) -> Result<&mut Config>
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut sources, ..
            } => {
                sources.push(Box::new(source));
            }

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen);
            }
        }

        self.refresh()
    }

    /// Merge in a configuration property source.
    pub fn with_merged<T>(mut self, source: T) -> Result<Self>
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut sources, ..
            } => {
                sources.push(Box::new(source));
            }

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen);
            }
        }

        self.refresh()?;
        Ok(self)
    }

    /// Merge multiple sources without refresh()ing each time.
    ///
    /// This function can be used if the `Config::refresh()` call would be expensive (e.g. with a
    /// very large configuration).
    /// Because `Config::merge()` and `Config::with_merged()` call `Config::refresh()` before
    /// returning, and `Config::refresh()` might be expensive, this function exists.
    ///
    /// Merging multiple sources with this function calls `Config::refresh()` only once: after all
    /// config sources are merged in.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use config::ConfigError;
    /// # use config::Config;
    /// # fn run() -> Result<(), ConfigError> {
    /// let config = Config::builder()
    ///     .with_source(config::File::with_name("Settings"))
    ///     .with_source(config::File::with_name("MoreSettings"))
    ///     .with_source(config::File::with_name("EvenMoreSettings"))
    ///     .build()?;
    /// # Ok(())
    /// # }
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            defaults: HashMap::new(),
            overrides: HashMap::new(),
            sources: Vec::new(),
        }
    }

    /// Refresh the configuration cache with fresh
    /// data from added sources.
    ///
    /// Configuration is automatically refreshed after a mutation
    /// operation (`set`, `merge`, `set_default`, etc.).
    pub fn refresh(&mut self) -> Result<&mut Config> {
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
                sources.collect_to(&mut cache)?;

                // Add overrides
                for (key, val) in overrides {
                    key.set(&mut cache, val.clone());
                }

                cache
            }

            ConfigKind::Frozen => {
                return Err(ConfigError::Frozen);
            }
        };

        Ok(self)
    }

    /// Set a default `value` at `key`
    pub fn set_default<T>(&mut self, key: &str, value: T) -> Result<&mut Config>
    where
        T: Into<Value>,
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut defaults, ..
            } => {
                defaults.insert(key.parse()?, value.into());
            }

            ConfigKind::Frozen => return Err(ConfigError::Frozen),
        };

        self.refresh()
    }

    /// Set an overwrite
    ///
    /// This function sets an overwrite value.
    /// The overwrite `value` is written to the `key` location on every `refresh()`
    ///
    /// # Warning
    ///
    /// Errors if config is frozen
    pub fn set<T>(&mut self, key: &str, value: T) -> Result<&mut Config>
    where
        T: Into<Value>,
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut overrides, ..
            } => {
                overrides.insert(key.parse()?, value.into());
            }

            ConfigKind::Frozen => return Err(ConfigError::Frozen),
        };

        self.refresh()
    }

    pub fn set_once(&mut self, key: &str, value: Value) -> Result<()> {
        let expr: path::Expression = key.parse()?;

        // Traverse the cache using the path to (possibly) retrieve a value
        if let Some(ref mut val) = expr.get_mut(&mut self.cache) {
            **val = value;
        } else {
            expr.set(&mut self.cache, value);
        }
        Ok(())
    }

    pub fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T> {
        // Parse the key into a path expression
        let expr: path::Expression = key.parse()?;

        // Traverse the cache using the path to (possibly) retrieve a value
        let value = expr.get(&self.cache).cloned();

        match value {
            Some(value) => {
                // Deserialize the received value into the requested type
                T::deserialize(value).map_err(|e| e.extend_with_key(key))
            }

            None => Err(ConfigError::NotFound(key.into())),
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String> {
        self.get(key).and_then(Value::into_string)
    }

    pub fn get_int(&self, key: &str) -> Result<i64> {
        self.get(key).and_then(Value::into_int)
    }

    pub fn get_float(&self, key: &str) -> Result<f64> {
        self.get(key).and_then(Value::into_float)
    }

    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.get(key).and_then(Value::into_bool)
    }

    pub fn get_table(&self, key: &str) -> Result<HashMap<String, Value>> {
        self.get(key).and_then(Value::into_table)
    }

    pub fn get_array(&self, key: &str) -> Result<Vec<Value>> {
        self.get(key).and_then(Value::into_array)
    }

    /// Attempt to deserialize the entire configuration into the requested type.
    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T> {
        T::deserialize(self)
    }

    /// Attempt to serialize the entire configuration from the given type.
    pub fn try_from<T: Serialize>(from: &T) -> Result<Self> {
        let mut serializer = ConfigSerializer::default();
        from.serialize(&mut serializer)?;
        Ok(serializer.output)
    }

    #[deprecated(since = "0.7.0", note = "please use 'try_into' instead")]
    pub fn deserialize<'de, T: Deserialize<'de>>(self) -> Result<T> {
        self.try_into()
    }
}

impl Source for Config {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        self.cache.clone().into_table()
    }
}

pub struct ConfigBuilder {
    defaults: HashMap<path::Expression, Value>,
    overrides: HashMap<path::Expression, Value>,
    sources: Vec<Box<dyn Source + Send + Sync>>,
}

impl ConfigBuilder {
    /// Merge a configuration source onto the builder object
    pub fn with_source<T>(mut self, source: T) -> Self
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        self.sources.push(Box::new(source));
        self
    }

    /// Set a default value at a certain key
    pub fn with_default<T>(mut self, key: &str, value: T) -> Result<Self>
    where
        T: Into<Value>,
    {
        self.defaults.insert(key.parse()?, value.into());
        Ok(self)
    }

    /// Set an overwrite value at a certain key
    pub fn with_overwrite<T>(mut self, key: &str, value: T) -> Result<Self>
    where
        T: Into<Value>,
    {
        self.overrides.insert(key.parse()?, value.into());
        Ok(self)
    }

    /// Build the Config object
    pub fn build(mut self) -> Result<Config> {
        let mut c = Config::default();
        match c.kind {
            ConfigKind::Mutable {
                ref mut sources, ..
            } => {
                sources.append(&mut self.sources);
            },

            ConfigKind::Frozen => return Err(ConfigError::Frozen),
        }

        c.refresh()?;
        Ok(c)
    }
}
