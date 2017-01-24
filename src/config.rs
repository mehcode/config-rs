use value::Value;
use source::Source;

use std::env;
use std::error::Error;
use std::convert::TryFrom;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {
    env_prefix: Option<String>,

    defaults: HashMap<String, Value>,
    overrides: HashMap<String, Value>,
    environ: HashMap<String, Value>,
    sources: Vec<HashMap<String, Value>>,
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }

    /// Merge in configuration values from the given source.
    pub fn merge<T>(&mut self, mut source: T) -> Result<(), Box<Error>>
        where T: Source
    {
        self.sources.push(source.build()?);

        Ok(())
    }

    /// Defines a prefix that environment variables
    /// must start with to be considered.
    ///
    /// By default all environment variables are considered. This can lead to unexpected values
    /// in configuration (eg. `PATH`).
    pub fn set_env_prefix(&mut self, prefix: &str) {
        self.env_prefix = Some(prefix.to_uppercase());
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

    pub fn get<'a, T>(&'a mut self, key: &str) -> Option<T>
        where T: TryFrom<&'a mut Value>,
              T: Default
    {
        // Check explicit override

        if let Some(value) = self.overrides.get_mut(key) {
            return T::try_from(value).ok();
        }

        // Check environment

        // Transform key into an env_key which is uppercased
        // and has the optional prefix applied
        let mut env_key = String::new();

        if let Some(ref env_prefix) = self.env_prefix {
            env_key.push_str(env_prefix);
            env_key.push('_');
        }

        env_key.push_str(&key.to_uppercase());

        if let Ok(value) = env::var(env_key.clone()) {
            // Store the environment variable into an environ
            // hash map; we want to return references
            self.environ.insert(key.to_lowercase().into(), value.into());

            return T::try_from(self.environ.get_mut(key).unwrap()).ok();
        }

        // Check sources

        for source in &mut self.sources.iter_mut().rev() {
            if let Some(value) = source.get_mut(key) {
                return T::try_from(value).ok();
            }
        }

        // Check explicit defaults

        if let Some(value) = self.defaults.get_mut(key) {
            return T::try_from(value).ok();
        }

        None
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
    // use std::env;
    use super::Config;

    // Retrieval of a non-existent key
    #[test]
    fn test_not_found() {
        let mut c = Config::new();

        assert_eq!(c.get_int("key"), None);
    }

    // // Environment override
    // #[test]
    // fn test_env_override() {
    //     let mut c = Config::new();

    //     c.set_default("key_1", false);

    //     env::set_var("KEY_1", "1");

    //     assert_eq!(c.get_bool("key_1"), Some(true));

    //     // TODO(@rust): Is there a way to easily kill this at the end of a test?
    //     env::remove_var("KEY_1");
    // }

    // // Environment prefix
    // #[test]
    // fn test_env_prefix() {
    //     let mut c = Config::new();

    //     env::set_var("KEY_1", "1");
    //     env::set_var("CFG_KEY_2", "false");

    //     c.set_env_prefix("CFG");

    //     assert_eq!(c.get_bool("key_1"), None);
    //     assert_eq!(c.get_bool("key_2"), Some(false));

    //     // TODO(@rust): Is there a way to easily kill this at the end of a test?
    //     env::remove_var("KEY_1");
    //     env::remove_var("CFG_KEY_2");
    // }

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
        assert!("value" == c.get::<&str>("key").unwrap());
    }

    // Storage and retrieval of Boolean values
    #[test]
    fn test_bool() {
        let mut c = Config::new();

        c.set("key", true);

        assert_eq!(c.get_bool("key").unwrap(), true);
        assert!(false != c.get("key").unwrap());
    }

    // Storage and retrieval of Float values
    #[test]
    fn test_float() {
        let mut c = Config::new();

        c.set("key", 3.14);

        assert_eq!(c.get_float("key").unwrap(), 3.14);
        assert!(3.14 >= c.get("key").unwrap());
    }

    // Storage and retrieval of Integer values
    #[test]
    fn test_int() {
        let mut c = Config::new();

        c.set("key", 42);

        assert_eq!(c.get_int("key").unwrap(), 42);
        assert!(42 == c.get::<i64>("key").unwrap());
    }

    // Storage of various values and retrieval as String
    #[test]
    fn test_retrieve_str() {
        let mut c = Config::new();

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
