use value::Value;
use source::{Source, SourceBuilder};

use std::error::Error;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {
    defaults: HashMap<String, Value>,
    overrides: HashMap<String, Value>,
    sources: Vec<Box<Source>>,
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }

    /// Merge in configuration values from the given source.
    pub fn merge<T>(&mut self, source: T) -> Result<(), Box<Error>>
        where T: SourceBuilder
    {
        self.sources.push(source.build()?);

        Ok(())
    }

    /// Sets the default value for this key. The default value is only used
    /// when no other value is provided.
    pub fn set_default<T>(&mut self, key: &str, value: T)
        where T: Into<Value>
    {
        self.defaults.insert(key.to_lowercase(), value.into());
    }

    /// Sets an override for this key.
    pub fn set<T>(&mut self, key: &str, value: T)
        where T: Into<Value>
    {
        self.overrides.insert(key.to_lowercase(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        // Check explicit override

        if let Some(value) = self.overrides.get(key) {
            return Some(value.clone());
        }

        // Check sources

        for source in &mut self.sources.iter().rev() {
            if let Some(value) = source.get(key) {
                return Some(value);
            }
        }

        // Check explicit defaults

        if let Some(value) = self.defaults.get(key) {
            return Some(value.clone());
        }

        None
    }

    pub fn get_str(&self, key: &str) -> Option<String> {
        self.get(key).and_then(Value::as_str)
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(Value::as_int)
    }

    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(Value::as_float)
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(Value::as_bool)
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

        c.set_default("key_1", false);
        c.set_default("key_2", false);

        assert!(!c.get_bool("key_1").unwrap());
        assert!(!c.get_bool("key_2").unwrap());

        c.set("key_2", true);

        assert!(!c.get_bool("key_1").unwrap());
        assert!(c.get_bool("key_2").unwrap());
    }

    // Storage and retrieval of String values
    #[test]
    fn test_str() {
        let mut c = Config::new();

        c.set("key", "value");

        assert_eq!(c.get_str("key").unwrap(), "value");
    }

    // Storage and retrieval of Boolean values
    #[test]
    fn test_bool() {
        let mut c = Config::new();

        c.set("key", true);

        assert_eq!(c.get_bool("key").unwrap(), true);
    }

    // Storage and retrieval of Float values
    #[test]
    fn test_float() {
        let mut c = Config::new();

        c.set("key", 3.14);

        assert_eq!(c.get_float("key").unwrap(), 3.14);
    }

    // Storage and retrieval of Integer values
    #[test]
    fn test_int() {
        let mut c = Config::new();

        c.set("key", 42);

        assert_eq!(c.get_int("key").unwrap(), 42);
    }

    // Storage of various values and retrieval as String
    #[test]
    fn test_retrieve_str() {
        let mut c = Config::new();

        c.set("key_1", 115);
        c.set("key_2", 1.23);
        c.set("key_3", false);

        assert_eq!(c.get_str("key_1").unwrap(), "115");
        assert_eq!(c.get_str("key_2").unwrap(), "1.23");
        assert_eq!(c.get_str("key_3").unwrap(), "false");
    }

    // Storage of various values and retrieval as Integer
    #[test]
    fn test_retrieve_int() {
        let mut c = Config::new();

        c.set("key_1", "121");
        c.set("key_2", 5.12);
        c.set("key_3", 5.72);
        c.set("key_4", false);
        c.set("key_5", true);
        c.set("key_6", "asga");

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

        c.set("key_1", "121");
        c.set("key_2", "121.512");
        c.set("key_3", 5);
        c.set("key_4", false);
        c.set("key_5", true);
        c.set("key_6", "asga");

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

        c.set("key_1", "121");
        c.set("key_2", "1");
        c.set("key_3", "0");
        c.set("key_4", "true");
        c.set("key_5", "");
        c.set("key_6", 51);
        c.set("key_7", 0);
        c.set("key_8", 12.12);
        c.set("key_9", 1.0);
        c.set("key_10", 0.0);
        c.set("key_11", "asga");

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
