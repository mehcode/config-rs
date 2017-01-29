use value::Value;
use source::{Source, SourceBuilder};

use std::error::Error;
use std::fmt;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct FrozenError { }

impl fmt::Display for FrozenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrozenError")
    }
}

impl Error for FrozenError {
    fn description(&self) -> &'static str {
        "configuration is frozen"
    }
}

// Underlying storage for the configuration
enum ConfigStore<'a> {
    Mutable {
        defaults: HashMap<String, Value<'a>>,
        overrides: HashMap<String, Value<'a>>,
        sources: Vec<Box<Source>>,
    },

    // TODO: Will be used for frozen configuratino soon
    #[allow(dead_code)]
    Frozen,
}

impl<'a> Default for ConfigStore<'a> {
    fn default() -> Self {
        ConfigStore::Mutable {
            defaults: HashMap::new(),
            overrides: HashMap::new(),
            sources: Vec::new(),
        }
    }
}

impl<'a> ConfigStore<'a> {
    fn merge<T>(&mut self, source: T) -> Result<(), Box<Error>>
        where T: SourceBuilder
    {
        if let ConfigStore::Mutable { ref mut sources, .. } = *self {
            sources.push(source.build()?);

            Ok(())
        } else {
            Err(FrozenError::default().into())
        }
    }

    fn set_default<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value<'a>>
    {
        if let ConfigStore::Mutable { ref mut defaults, .. } = *self {
            defaults.insert(key.to_lowercase(), value.into());

            Ok(())
        } else {
            Err(FrozenError::default().into())
        }
    }

    fn set<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value<'a>>
    {
        if let ConfigStore::Mutable { ref mut overrides, .. } = *self {
            overrides.insert(key.to_lowercase(), value.into());

            Ok(())
        } else {
            Err(FrozenError::default().into())
        }
    }

    fn get(&self, key: &str) -> Option<Value> {
        if let ConfigStore::Mutable { ref overrides, ref sources, ref defaults } = *self {
            // Check explicit override
            if let Some(value) = overrides.get(key) {
                return Some(value.clone());
            }

            // Check sources
            for source in &mut sources.iter().rev() {
                if let Some(value) = source.get(key) {
                    return Some(value);
                }
            }

            // Check explicit defaults
            if let Some(value) = defaults.get(key) {
                return Some(value.clone());
            }
        }

        None
    }
}

#[derive(Default)]
pub struct Config<'a> {
    store: ConfigStore<'a>,
}

// TODO(@rust): There must be a way to remove this function and use Value::as_str
#[allow(needless_lifetimes)]
fn value_into_str<'a>(value: Value<'a>) -> Option<Cow<'a, str>> {
    if let Value::String(value) = value {
        Some(value)
    } else if let Value::Integer(value) = value {
        Some(Cow::Owned(value.to_string()))
    } else if let Value::Float(value) = value {
        Some(Cow::Owned(value.to_string()))
    } else if let Value::Boolean(value) = value {
        Some(Cow::Owned(value.to_string()))
    } else {
        None
    }
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Default::default()
    }

    /// Merge in configuration values from the given source.
    pub fn merge<T>(&mut self, source: T) -> Result<(), Box<Error>>
        where T: SourceBuilder
    {
        self.store.merge(source)
    }

    /// Sets the default value for this key. The default value is only used
    /// when no other value is provided.
    pub fn set_default<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value<'a>>
    {
        self.store.set_default(key, value)
    }

    /// Sets an override for this key.
    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value<'a>>
    {
        self.store.set(key, value)
    }

    pub fn get(&self, key: &str) -> Option<Cow<'a, Value>> {
        self.store.get(key).map(Cow::Owned)
    }

    pub fn get_str(&'a self, key: &str) -> Option<Cow<'a, str>> {
        // TODO(@rust): This is a bit nasty looking; 3x match and requires this
        //              odd into_str method
        if let Some(value) = self.get(key) {
            match value {
                Cow::Borrowed(value) => {
                    match value.as_str() {
                        Some(value) => {
                            match value {
                                Cow::Borrowed(value) => Some(Cow::Borrowed(value)),
                                Cow::Owned(value) => Some(Cow::Owned(value)),
                            }
                        }

                        _ => None,
                    }
                }

                Cow::Owned(value) => value_into_str(value),
            }
        } else {
            None
        }
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        // TODO(@rust): Why doesn't .and_then(Value::as_int) work?
        self.get(key).and_then(|v| v.as_int())
    }

    pub fn get_float(&self, key: &str) -> Option<f64> {
        // TODO(@rust): Why doesn't .and_then(Value::as_float) work?
        self.get(key).and_then(|v| v.as_float())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        // TODO(@rust): Why doesn't .and_then(Value::as_bool) work?
        self.get(key).and_then(|v| v.as_bool())
    }
}

#[cfg(test)]
mod test {
    // use std::env;
    use super::Config;

    // Retrieval of a non-existent key
    #[test]
    fn test_not_found() {
        let c = Config::new();

        assert_eq!(c.get_int("key"), None);
    }

    // Explicit override
    #[test]
    fn test_default_override() {
        let mut c = Config::new();

        c.set_default("key_1", false).unwrap();
        c.set_default("key_2", false).unwrap();

        assert!(!c.get_bool("key_1").unwrap());
        assert!(!c.get_bool("key_2").unwrap());

        c.set("key_2", true).unwrap();

        assert!(!c.get_bool("key_1").unwrap());
        assert!(c.get_bool("key_2").unwrap());
    }

    // Storage and retrieval of String values
    #[test]
    fn test_str() {
        let mut c = Config::new();

        c.set("key", "value").unwrap();

        assert_eq!(c.get_str("key").unwrap(), "value");
    }

    // Storage and retrieval of Boolean values
    #[test]
    fn test_bool() {
        let mut c = Config::new();

        c.set("key", true).unwrap();

        assert_eq!(c.get_bool("key").unwrap(), true);
    }

    // Storage and retrieval of Float values
    #[test]
    fn test_float() {
        let mut c = Config::new();

        c.set("key", 3.14).unwrap();

        assert_eq!(c.get_float("key").unwrap(), 3.14);
    }

    // Storage and retrieval of Integer values
    #[test]
    fn test_int() {
        let mut c = Config::new();

        c.set("key", 42).unwrap();

        assert_eq!(c.get_int("key").unwrap(), 42);
    }

    // Storage of various values and retrieval as String
    #[test]
    fn test_retrieve_str() {
        let mut c = Config::new();

        c.set("key_1", 115).unwrap();
        c.set("key_2", 1.23).unwrap();
        c.set("key_3", false).unwrap();

        assert_eq!(c.get_str("key_1").unwrap(), "115");
        assert_eq!(c.get_str("key_2").unwrap(), "1.23");
        assert_eq!(c.get_str("key_3").unwrap(), "false");
    }

    // Storage of various values and retrieval as Integer
    #[test]
    fn test_retrieve_int() {
        let mut c = Config::new();

        c.set("key_1", "121").unwrap();
        c.set("key_2", 5.12).unwrap();
        c.set("key_3", 5.72).unwrap();
        c.set("key_4", false).unwrap();
        c.set("key_5", true).unwrap();
        c.set("key_6", "asga").unwrap();

        assert_eq!(c.get_int("key_1"), Some(121));
        assert_eq!(c.get_int("key_2"), Some(5));
        assert_eq!(c.get_int("key_3"), Some(6));
        assert_eq!(c.get_int("key_4"), Some(0));
        assert_eq!(c.get_int("key_5"), Some(1));
        assert_eq!(c.get_int("key_6"), None);
    }

    // Storage of various values and retrieval as Float
    #[test]
    fn test_retrieve_float() {
        let mut c = Config::new();

        c.set("key_1", "121").unwrap();
        c.set("key_2", "121.512").unwrap();
        c.set("key_3", 5).unwrap();
        c.set("key_4", false).unwrap();
        c.set("key_5", true).unwrap();
        c.set("key_6", "asga").unwrap();

        assert_eq!(c.get_float("key_1"), Some(121.0));
        assert_eq!(c.get_float("key_2"), Some(121.512));
        assert_eq!(c.get_float("key_3"), Some(5.0));
        assert_eq!(c.get_float("key_4"), Some(0.0));
        assert_eq!(c.get_float("key_5"), Some(1.0));
        assert_eq!(c.get_float("key_6"), None);
    }

    // Storage of various values and retrieval as Boolean
    #[test]
    fn test_retrieve_bool() {
        let mut c = Config::new();

        c.set("key_1", "121").unwrap();
        c.set("key_2", "1").unwrap();
        c.set("key_3", "0").unwrap();
        c.set("key_4", "true").unwrap();
        c.set("key_5", "").unwrap();
        c.set("key_6", 51).unwrap();
        c.set("key_7", 0).unwrap();
        c.set("key_8", 12.12).unwrap();
        c.set("key_9", 1.0).unwrap();
        c.set("key_10", 0.0).unwrap();
        c.set("key_11", "asga").unwrap();

        assert_eq!(c.get_bool("key_1"), None);
        assert_eq!(c.get_bool("key_2"), Some(true));
        assert_eq!(c.get_bool("key_3"), Some(false));
        assert_eq!(c.get_bool("key_4"), Some(true));
        assert_eq!(c.get_bool("key_5"), None);
        assert_eq!(c.get_bool("key_6"), Some(true));
        assert_eq!(c.get_bool("key_7"), Some(false));
        assert_eq!(c.get_bool("key_8"), Some(true));
        assert_eq!(c.get_bool("key_9"), Some(true));
        assert_eq!(c.get_bool("key_10"), Some(false));
        assert_eq!(c.get_bool("key_11"), None);
    }
}
