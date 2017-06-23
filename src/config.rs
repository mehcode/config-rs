use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::fmt::Debug;
use serde::de::Deserialize;

use error::*;
use source::Source;

use value::{Value, ValueWithKey};
use path;

#[derive(Clone, Debug)]
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
#[derive(Default, Clone, Debug)]
pub struct Config {
    kind: ConfigKind,

    /// Root of the cached configuration.
    pub cache: Value,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    /// Merge in a configuration property source.
    pub fn merge<T>(&mut self, source: T) -> ConfigResult
        where T: 'static,
              T: Source + Send + Sync
    {
        match self.kind {
            ConfigKind::Mutable { ref mut sources, .. } => {
                sources.push(Box::new(source));
            }

            ConfigKind::Frozen => {
                return ConfigResult::Err(ConfigError::Frozen);
            }
        }

        self.refresh()
    }

    /// Refresh the configuration cache with fresh
    /// data from added sources.
    ///
    /// Configuration is automatically refreshed after a mutation
    /// operation (`set`, `merge`, `set_default`, etc.).
    pub fn refresh(&mut self) -> ConfigResult {
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
                if let Err(error) = sources.collect_to(&mut cache) {
                    return ConfigResult::Err(error);
                }

                // Add overrides
                for (key, val) in overrides {
                    key.set(&mut cache, val.clone());
                }

                cache
            }

            ConfigKind::Frozen => {
                return ConfigResult::Err(ConfigError::Frozen);
            }
        };

        ConfigResult::Ok(self)
    }

    /// Deserialize the entire configuration.
    pub fn deserialize<'de, T: Deserialize<'de>>(&self) -> Result<T> {
        T::deserialize(self.cache.clone())
    }

    pub fn set_default<T>(&mut self, key: &str, value: T) -> ConfigResult
        where T: Into<Value>
    {
        match self.kind {
            ConfigKind::Mutable { ref mut defaults, .. } => {
                defaults.insert(match key.to_lowercase().parse() {
                                    Ok(expr) => expr,
                                    Err(error) => {
                                        return ConfigResult::Err(error);
                                    }
                                },
                                value.into());
            }

            ConfigKind::Frozen => return ConfigResult::Err(ConfigError::Frozen),
        };

        self.refresh()
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> ConfigResult
        where T: Into<Value>
    {
        match self.kind {
            ConfigKind::Mutable { ref mut overrides, .. } => {
                overrides.insert(match key.to_lowercase().parse() {
                                     Ok(expr) => expr,
                                     Err(error) => {
                                         return ConfigResult::Err(error);
                                     }
                                 },
                                 value.into());
            }

            ConfigKind::Frozen => return ConfigResult::Err(ConfigError::Frozen),
        };

        self.refresh()
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

    pub fn get_str(&self, key: &str) -> Result<String> {
        self.get(key).and_then(Value::into_str)
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
}

/// Holds the result of configuration alteration functions.
/// A manual alias of Result to enable a chained API and error forwarding.
pub enum ConfigResult<'a> {
    Ok(&'a mut Config),
    Err(ConfigError),
}

#[inline]
fn unwrap_failed<E: Debug>(msg: &str, error: E) -> ! {
    panic!("{}: {:?}", msg, error)
}

impl<'a> ConfigResult<'a> {
    /// Forwards `Config::merge`
    pub fn merge<T>(self, source: T) -> ConfigResult<'a>
        where T: 'static,
              T: Source + Send + Sync
    {
        match self {
            // If OK, Proceed to nested method
            ConfigResult::Ok(instance) => instance.merge(source),

            // Else, Forward the error
            error => error,
        }
    }

    /// Forwards `Config::set_default`
    pub fn set_default<T>(self, key: &str, value: T) -> ConfigResult<'a>
        where T: Into<Value>,
              T: 'static
    {
        match self {
            // If OK, Proceed to nested method
            ConfigResult::Ok(instance) => instance.set_default(key, value),

            // Else, Forward the error
            error => error,
        }
    }

    /// Forwards `Config::set`
    pub fn set<T>(self, key: &str, value: T) -> ConfigResult<'a>
        where T: Into<Value>,
              T: 'static
    {
        match self {
            // If OK, Proceed to nested method
            ConfigResult::Ok(instance) => instance.set(key, value),

            // Else, Forward the error
            error => error,
        }
    }

    /// Deserialize the entire configuration.
    pub fn deserialize<'de, T: Deserialize<'de>>(self) -> Result<T> {
        self.and_then(|instance| instance.deserialize())
    }

    /// Creates a `Result` out of this `ConfigResult`
    #[inline]
    pub fn to_result(self) -> Result<Config> {
        match self {
            ConfigResult::Ok(value) => Ok(value.clone()),
            ConfigResult::Err(error) => Err(error),
        }
    }

    /// Forwards `Result::is_ok`
    #[inline]
    pub fn is_ok(&self) -> bool {
        match *self {
            ConfigResult::Ok(_) => true,
            ConfigResult::Err(_) => false,
        }
    }

    /// Forwards `Result::is_err`
    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Forwards `Result::ok`
    #[inline]
    pub fn ok(self) -> Option<Config> {
        match self {
            ConfigResult::Ok(x) => Some(x.clone()),
            ConfigResult::Err(_) => None,
        }
    }

    /// Forwards `Result::err`
    #[inline]
    pub fn err(self) -> Option<ConfigError> {
        match self {
            ConfigResult::Ok(_) => None,
            ConfigResult::Err(x) => Some(x),
        }
    }

    /// Forwards `Result::map`
    #[inline]
    pub fn map<U, F>(self, op: F) -> Result<U>
        where
            F: FnOnce(Config) -> U,
    {
        match self {
            ConfigResult::Ok(x) => Ok(op(x.clone())),
            ConfigResult::Err(error) => Err(error),
        }
    }

    /// Forwards `Result::map_err`
    #[inline]
    pub fn map_err<F, O>(self, op: O) -> ::std::result::Result<Config, F>
        where
            O: FnOnce(ConfigError) -> F,
    {
        match self {
            ConfigResult::Err(error) => Err(op(error)),
            ConfigResult::Ok(value) => Ok(value.clone()),
        }
    }

    /// Forwards `Result::and`
    #[inline]
    pub fn and<U>(self, res: Result<U>) -> Result<U>
    {
        match self {
            ConfigResult::Ok(_) => res,
            ConfigResult::Err(error) => Err(error),
        }
    }

    /// Forwards `Result::and_then`
    #[inline]
    pub fn and_then<U, F>(self, op: F) -> Result<U>
        where
            F: FnOnce(Config) -> Result<U>,
    {
        match self {
            ConfigResult::Ok(value) => op(value.clone()),
            ConfigResult::Err(error) => Err(error),
        }
    }

    /// Forwards `Result::or`
    #[inline]
    pub fn or<F>(self, res: ::std::result::Result<Config, F>) -> ::std::result::Result<Config, F>
    {
        match self {
            ConfigResult::Ok(value) => Ok(value.clone()),
            ConfigResult::Err(_) => res,
        }
    }

    /// Forwards `Result::or_else`
    #[inline]
    pub fn or_else<F, O>(self, op: O) -> ::std::result::Result<Config, F>
    where
        O: FnOnce(ConfigError) -> ::std::result::Result<Config, F>,
    {
        match self {
            ConfigResult::Ok(value) => Ok(value.clone()),
            ConfigResult::Err(error) => op(error),
        }
    }

    /// Forwards `Result::unwrap`
    #[inline]
    pub fn unwrap(self) -> Config {
        match self {
            ConfigResult::Ok(instance) => instance.clone(),
            ConfigResult::Err(error) => unwrap_failed("called `ConfigResult::unwrap()` on an `Err` value", error),
        }
    }

    /// Forwards `Result::expect`
    #[inline]
    pub fn expect(self, msg: &str) -> Config {
        match self {
            ConfigResult::Ok(instance) => instance.clone(),
            ConfigResult::Err(error) => unwrap_failed(msg, error),
        }
    }

    /// Forwards `Result::unwrap_err`
    #[inline]
    pub fn unwrap_err(self) -> ConfigError {
        match self {
            ConfigResult::Ok(t) => unwrap_failed("called `ConfigResult::unwrap_err()` on an `Ok` value", t),
            ConfigResult::Err(e) => e,
        }
    }

    /// Forwards `Result::expect_err`
    #[inline]
    pub fn expect_err(self, msg: &str) -> ConfigError {
        match self {
            ConfigResult::Ok(t) => unwrap_failed(msg, t),
            ConfigResult::Err(e) => e,
        }
    }
}
