#![feature(try_from)]

mod value;

use value::Value;

use std::env;
use std::convert::TryFrom;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {
    defaults: HashMap<String, Value>,
    overrides: HashMap<String, Value>,
    environ: HashMap<String, Value>,
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }

    pub fn set_default<T>(&mut self, key: &str, value: T)
        where T: Into<Value>
    {
        self.defaults.insert(key.into(), value.into());
    }

    pub fn set<T>(&mut self, key: &str, value: T)
        where T: Into<Value>
    {
        self.overrides.insert(key.into(), value.into());
    }

    pub fn get<'a, T>(&'a mut self, key: &str) -> Option<T>
        where T: TryFrom<&'a mut Value>,
              T: Default
    {
        if let Some(value) = self.overrides.get_mut(key) {
            T::try_from(value).ok()
        } else if let Ok(value) = env::var(key.to_uppercase()) {
            // Store the environment variable into an environ
            // hash map; we want to return references

            // TODO: Key name needs to go through a transform
            self.environ.insert(key.to_lowercase().into(), value.into());

            T::try_from(self.environ.get_mut(key).unwrap()).ok()
        } else if let Some(value) = self.defaults.get_mut(key) {
            T::try_from(value).ok()
        } else {
            None
        }
    }

    pub fn get_str<'a>(&'a mut self, key: &str) -> Option<&'a str> {
        self.get(key)
    }

    pub fn get_int(&mut self, key: &str) -> Option<i64> {
        self.get(key)
    }

    pub fn get_float(&mut self, key: &str) -> Option<f64> {
        self.get(key)
    }

    pub fn get_bool(&mut self, key: &str) -> Option<bool> {
        self.get(key)
    }
}

#[cfg(test)]
mod test {
    use std::env;

    // Retrieval of a non-existent key
    #[test]
    fn test_not_found() {
        let mut c = super::Config::new();

        assert_eq!(c.get_int("key"), None);
    }

    // Environment override
    #[test]
    fn test_env_override() {
        let mut c = super::Config::new();

        c.set_default("key_1", false);

        env::set_var("KEY_1", "1");

        assert_eq!(c.get_bool("key_1"), Some(true));
    }

    // Explicit override
    #[test]
    fn test_default_override() {
        let mut c = super::Config::new();

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
        let mut c = super::Config::new();

        c.set("key", "value");

        assert_eq!(c.get_str("key").unwrap(), "value");
        assert!("value" == c.get::<&str>("key").unwrap());
    }

    // Storage and retrieval of Boolean values
    #[test]
    fn test_bool() {
        let mut c = super::Config::new();

        c.set("key", true);

        assert_eq!(c.get_bool("key").unwrap(), true);
        assert!(false != c.get("key").unwrap());
    }

    // Storage and retrieval of Float values
    #[test]
    fn test_float() {
        let mut c = super::Config::new();

        c.set("key", 3.14);

        assert_eq!(c.get_float("key").unwrap(), 3.14);
        assert!(3.14 >= c.get("key").unwrap());
    }

    // Storage and retrieval of Integer values
    #[test]
    fn test_int() {
        let mut c = super::Config::new();

        c.set("key", 42);

        assert_eq!(c.get_int("key").unwrap(), 42);
        assert!(42 == c.get::<i64>("key").unwrap());
    }

    // Storage of various values and retrieval as String
    #[test]
    fn test_retrieve_str() {
        let mut c = super::Config::new();

        c.set("key_1", 115);
        c.set("key_2", 1.23);
        c.set("key_3", false);

        assert_eq!(c.get_str("key_1"), Some("115"));
        assert_eq!(c.get_str("key_2"), Some("1.23"));
        assert_eq!(c.get_str("key_3"), Some("false"));
    }

    // Storage of various values and retrieval as Integer
    #[test]
    fn test_retrieve_int() {
        let mut c = super::Config::new();

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
        let mut c = super::Config::new();

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
        let mut c = super::Config::new();

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

        assert_eq!(c.get_bool("key_1"), Some(false));
        assert_eq!(c.get_bool("key_2"), Some(true));
        assert_eq!(c.get_bool("key_3"), Some(false));
        assert_eq!(c.get_bool("key_4"), Some(true));
        assert_eq!(c.get_bool("key_5"), Some(false));
        assert_eq!(c.get_bool("key_6"), Some(true));
        assert_eq!(c.get_bool("key_7"), Some(false));
        assert_eq!(c.get_bool("key_8"), Some(true));
        assert_eq!(c.get_bool("key_9"), Some(true));
        assert_eq!(c.get_bool("key_10"), Some(false));
        assert_eq!(c.get_bool("key_11"), Some(false));
    }
}
