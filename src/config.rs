use value::Value;
use source::{Source, SourceBuilder};

use std::error::Error;
use std::fmt;
use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};

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
enum ConfigStore {
    Mutable {
        defaults: HashMap<String, Value>,
        overrides: HashMap<String, Value>,

        // Ordered list of sources
        sources: Vec<Box<Source>>,
    },

    // TODO: Will be used for frozen configuratino soon
    #[allow(dead_code)]
    Frozen,
}

impl Default for ConfigStore {
    fn default() -> Self {
        ConfigStore::Mutable {
            defaults: HashMap::new(),
            overrides: HashMap::new(),
            sources: Vec::new(),
        }
    }
}

const KEY_DELIM: char = '.';

fn merge_in(r: &mut HashMap<String, Value>, key: &str, value: &Value) {
    let key_segments: VecDeque<&str> = key.splitn(2, KEY_DELIM).collect();

    if key_segments.len() > 1 {
        // Ensure there is at least an empty hash map
        let key = key_segments[0].to_string();
        if r.contains_key(&key) {
            // Coerce to table
            match *r.get(&key).unwrap() {
                Value::Table(_) => {
                    // Do nothing; already table
                }

                _ => {
                    // Override with empty table
                    r.insert(key.clone(), Value::Table(HashMap::new()));
                }
            }
        } else {
            // Insert table
            r.insert(key.clone(), Value::Table(HashMap::new()));
        }

        // Continue to merge
        if let Value::Table(ref mut table) = *r.get_mut(&key).unwrap() {
            merge_in(table, key_segments[1], value);
        }

        return;
    }

    // Check if we are setting a table (and if we should do a deep merge)
    if let Value::Table(ref table) = *value {
        let inner_v = r.get_mut(key);
        if let Some(&mut Value::Table(ref mut inner_table)) = inner_v {
            merge_in_all(inner_table, table);

            return;
        }
    }

    // Direct set/override whatever is here
    r.insert(key.into(), value.clone());
}

fn merge_in_all(r: &mut HashMap<String, Value>, map: &HashMap<String, Value>) {
    for (key, value) in map {
        merge_in(r, key, value);
    }
}

impl ConfigStore {
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
        where T: Into<Value>
    {
        if let ConfigStore::Mutable { ref mut defaults, .. } = *self {
            merge_in(defaults, &key.to_lowercase(), &value.into());

            Ok(())
        } else {
            Err(FrozenError::default().into())
        }
    }

    fn set<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value>
    {
        if let ConfigStore::Mutable { ref mut overrides, .. } = *self {
            merge_in(overrides, &key.to_lowercase(), &value.into());

            Ok(())
        } else {
            Err(FrozenError::default().into())
        }
    }

    fn collect(&self) -> Result<HashMap<String, Value>, Box<Error>> {
        if let ConfigStore::Mutable { ref overrides, ref sources, ref defaults } = *self {
            let mut r = HashMap::<String, Value>::new();

            merge_in_all(&mut r, defaults);

            for source in sources {
                merge_in_all(&mut r, &source.collect());
            }

            merge_in_all(&mut r, overrides);

            Ok(r)
        } else {
            Err(FrozenError::default().into())
        }
    }
}

#[derive(Default)]
pub struct Config {
    store: ConfigStore,

    /// Top-level table of the cached configuration
    ///
    /// As configuration sources are merged with `Config::merge`, this
    /// cache is updated.
    cache: HashMap<String, Value>,
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }

    /// Merge in configuration values from the given source.
    pub fn merge<T>(&mut self, source: T) -> Result<(), Box<Error>>
        where T: SourceBuilder
    {
        self.store.merge(source)?;
        self.refresh()?;

        Ok(())
    }

    /// Sets the default value for this key. The default value is only used
    /// when no other value is provided.
    pub fn set_default<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value>
    {
        self.store.set_default(key, value)?;
        self.refresh()?;

        Ok(())
    }

    /// Sets an override for this key.
    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value>
    {
        self.store.set(key, value)?;
        self.refresh()?;

        Ok(())
    }

    /// Refresh the configuration cache with fresh
    /// data from associated sources.
    ///
    /// Configuration is automatically refreshed after a mutation
    /// operation (`set`, `merge`, `set_default`, etc.).
    pub fn refresh(&mut self) -> Result<(), Box<Error>> {
        self.cache = self.store.collect()?;

        Ok(())
    }

    pub fn get<'a>(&'a self, key: &str) -> Option<&'a Value> {
        self.cache.get(key)
    }

    pub fn get_str<'a>(&'a self, key: &str) -> Option<Cow<'a, str>> {
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

    pub fn get_map<'a>(&'a self, key: &str) -> Option<&'a HashMap<String, Value>> {
        self.get(key).and_then(Value::as_map)
    }
    
    pub fn get_slice<'a>(&'a self, key: &str) -> Option<&'a [Value]> {
        self.get(key).and_then(Value::as_slice)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::{Value, Config};

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

    #[test]
    fn test_slice() {
        let mut c = Config::new();

        c.set("values", vec![
            Value::Integer(10),
            Value::Integer(325),
            Value::Integer(12),
        ]);

        let values = c.get_slice("values").unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[1].as_int(), Some(325));
    }

    #[test]
    fn test_slice_into() {
        let mut c = Config::new();

        c.set("values", vec![
            10,
            325,
            12,
        ]);

        let values = c.get_slice("values").unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[1].as_int(), Some(325));

    }

    #[test]
    fn test_map() {
        let mut c = Config::new();

        {
            let mut m = HashMap::new();
            m.insert("port".into(), Value::Integer(6379));
            m.insert("address".into(), Value::String("::1".into()));

            c.set("redis", m).unwrap();
        }

        {
            let m = c.get_map("redis").unwrap();

            assert_eq!(m.get("port").unwrap().as_int().unwrap(), 6379);
            assert_eq!(m.get("address").unwrap().as_str().unwrap(), "::1");
        }

        {
            let mut m = HashMap::new();
            m.insert("address".into(), Value::String("::0".into()));
            m.insert("db".into(), Value::Integer(1));

            c.set("redis", m).unwrap();
        }

        {
            let m = c.get_map("redis").unwrap();

            assert_eq!(m.get("port").unwrap().as_int().unwrap(), 6379);
            assert_eq!(m.get("address").unwrap().as_str().unwrap(), "::0");
            assert_eq!(m.get("db").unwrap().as_str().unwrap(), "1");
        }
    }

    #[test]
    fn test_map_into() {
        let mut c = Config::new();

        {
            let mut m = HashMap::new();
            m.insert("port".into(), 6379);
            m.insert("db".into(), 2);

            c.set("redis", m).unwrap();
        }

        {
            let m = c.get_map("redis").unwrap();

            assert_eq!(m.get("port").unwrap().as_int().unwrap(), 6379);
            assert_eq!(m.get("db").unwrap().as_int().unwrap(), 2);
        }
    }
}
