use value::Value;
use source::{Source, SourceBuilder};
use path;

use std::error::Error;
use std::fmt;
use std::str::FromStr;
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

fn merge_in_all(r: &mut HashMap<String, Value>, map: &HashMap<String, Value>) {
    for (key, value) in map {
        path_set_str(r, key, value);
    }
}

// Child ( Child ( Identifier( "x" ), "y" ), "z" )
fn path_get_mut<'a>(root: &'a mut HashMap<String, Value>, expr: path::Expression) -> Option<&'a mut Value> {
    match expr {
        path::Expression::Identifier(text) => Some(root.entry(text.clone()).or_insert(Value::Nil)),

        path::Expression::Child(expr, member) => {
            match path_get_mut(root, *expr) {
                Some(&mut Value::Table(ref mut table)) => Some(table.entry(member.clone()).or_insert(Value::Nil)),

                Some(v @ _) => {
                    *v = Value::Table(HashMap::new());
                    if let Value::Table(ref mut table) = *v {
                        Some(table.entry(member.clone()).or_insert(Value::Nil))
                    } else {
                        None
                    }
                }

                _ => None,
            }
        }

        path::Expression::Subscript(expr, mut index) => {
            match path_get_mut(root, *expr) {
                Some(&mut Value::Array(ref mut array)) => {
                    let len = array.len() as i32;

                    if index < 0 {
                        index = len + index;
                    }

                    if index < 0 {
                        None
                    } else {
                        // Ensure there is enough room
                        array.resize((index + 1) as usize, Value::Nil);

                        Some(&mut array[index as usize])
                    }
                }

                _ => None,
            }
        }
    }
}

fn require_table(r: &mut HashMap<String, Value>, key: &String) {
    if r.contains_key(key) {
        // Coerce to table
        match *r.get(key).unwrap() {
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
}

fn path_set(root: &mut HashMap<String, Value>, expr: path::Expression, value: &Value) {
    match expr {
        path::Expression::Identifier(text) => {
            match *value {
                Value::Table(ref table_v) => {
                    require_table(root, &text);
                    if let Value::Table(ref mut target) = *root.get_mut(&text).unwrap() {
                        merge_in_all(target, table_v);
                    }
                }

                _ => {
                    root.insert(text, value.clone());
                }
            }
        }

        path::Expression::Child(expr, member) => {
            if let Some(parent) = path_get_mut(root, *expr) {
                match *parent {
                    Value::Table(ref mut table) => {
                        path_set(table, path::Expression::Identifier(member), value);
                    }

                    _ => {
                        // Coerce to a table and do the insert anyway
                        *parent = Value::Table(HashMap::new());
                        if let Value::Table(ref mut table) = *parent {
                            path_set(table, path::Expression::Identifier(member), value);
                        }
                    }
                }
            }
        }

        path::Expression::Subscript(inner_expr, mut index) => {
            if let Some(parent) = path_get_mut(root, *inner_expr) {
                match *parent {
                    Value::Array(ref mut array) => {
                        let len = array.len() as i32;

                        if index < 0 {
                            index = len + index;
                        }

                        if index >= 0 {
                            array[index as usize] = value.clone();
                        }
                    }

                    Value::Nil => {
                        // Add an array and do this again
                        *parent = Value::Array(Vec::new());
                        if let Value::Array(ref mut array) = *parent {
                            let len = array.len() as i32;

                            if index < 0 {
                                index = len + index;
                            }

                            if index >= 0 {
                                array.resize((index + 1) as usize, Value::Nil);
                                array[index as usize] = value.clone();
                            }
                        }
                    }

                    _ => {
                        // Do nothing
                    }
                }
            }
        }
    }
}

fn path_set_str(root: &mut HashMap<String, Value>, key: &str, value: &Value) {
    match path::Expression::from_str(key) {
        Ok(expr) => {
            path_set(root, expr, value);
        },

        Err(_) => {
            // TODO: Log warning here
        }
    };
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
            path_set_str(defaults, &key.to_lowercase(), &value.into());

            Ok(())
        } else {
            Err(FrozenError::default().into())
        }
    }

    fn set<T>(&mut self, key: &str, value: T) -> Result<(), Box<Error>>
        where T: Into<Value>
    {
        if let ConfigStore::Mutable { ref mut overrides, .. } = *self {
            path_set_str(overrides, &key.to_lowercase(), &value.into());

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

    // Child ( Child ( Identifier( "x" ), "y" ), "z" )
    fn path_get<'a>(&'a self, expr: path::Expression) -> Option<&'a Value> {
        match expr {
            path::Expression::Identifier(text) => self.cache.get(&text),

            path::Expression::Child(expr, member) => {
                match self.path_get(*expr) {
                    Some(&Value::Table(ref table)) => table.get(&member),

                    _ => None,
                }
            }

            path::Expression::Subscript(expr, mut index) => {
                match self.path_get(*expr) {
                    Some(&Value::Array(ref array)) => {
                        let len = array.len() as i32;

                        if index < 0 {
                            index = len + index;
                        }

                        if index < 0 || index >= len {
                            None
                        } else {
                            Some(&array[index as usize])
                        }
                    }

                    _ => None,
                }
            }
        }
    }

    pub fn get(&self, key_path: &str) -> Option<Value> {
        let key_expr: path::Expression = match key_path.to_lowercase().parse() {
            Ok(expr) => expr,
            Err(_) => {
                // TODO: Log warning here
                return None;
            }
        };

        self.path_get(key_expr).cloned()
    }

    pub fn get_str(&self, key: &str) -> Option<String> {
        self.get(key).and_then(Value::into_str)
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(Value::into_int)
    }

    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(Value::into_float)
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(Value::into_bool)
    }

    pub fn get_table(&self, key: &str) -> Option<HashMap<String, Value>> {
        self.get(key).and_then(Value::into_table)
    }

    pub fn get_array(self, key: &str) -> Option<Vec<Value>> {
        self.get(key).and_then(Value::into_array)
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

        c.set("values",
                 vec![Value::Integer(10), Value::Integer(325), Value::Integer(12)])
            .unwrap();

        let values = c.get_array("values").unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[1].clone().into_int(), Some(325));
    }

    #[test]
    fn test_slice_into() {
        let mut c = Config::new();

        c.set("values", vec![10, 325, 12])
            .unwrap();

        let values = c.get_array("values").unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[1].clone().into_int(), Some(325));

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
            let m = c.get_table("redis").unwrap();

            assert_eq!(m.get("port").cloned().unwrap().into_int().unwrap(), 6379);
            assert_eq!(m.get("address").cloned().unwrap().into_str().unwrap(), "::1");
        }

        {
            let mut m = HashMap::new();
            m.insert("address".into(), Value::String("::0".into()));
            m.insert("db".into(), Value::Integer(1));

            c.set("redis", m).unwrap();
        }

        {
            let m = c.get_table("redis").unwrap();

            assert_eq!(m.get("port").cloned().unwrap().into_int().unwrap(), 6379);
            assert_eq!(m.get("address").cloned().unwrap().into_str().unwrap(), "::0");
            assert_eq!(m.get("db").cloned().unwrap().into_str().unwrap(), "1");
        }
    }

    #[test]
    fn test_path() {
        use file::{File, FileFormat};

        let mut c = Config::new();

        c.merge(File::from_str(r#"
            [redis]
            address = "localhost:6379"

            [[databases]]
            name = "test_db"
            options = { trace = true }
        "#,
                                  FileFormat::Toml))
            .unwrap();

        assert_eq!(c.get_str("redis.address").unwrap(), "localhost:6379");
        assert_eq!(c.get_str("databases[0].name").unwrap(), "test_db");
        assert_eq!(c.get_str("databases[0].options.trace").unwrap(), "true");
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
            let m = c.get_table("redis").unwrap();

            assert_eq!(m.get("port").cloned().unwrap().into_int().unwrap(), 6379);
            assert_eq!(m.get("db").cloned().unwrap().into_int().unwrap(), 2);
        }
    }

    #[test]
    fn test_map_set_coerce() {
        let mut c = Config::new();

        // Coerce value to table
        c.set("redis", 10).unwrap();
        c.set("redis.port", 6379).unwrap();

        assert_eq!(c.get_int("redis.port"), Some(6379));

        // Coerce nil to table
        c.set("server.port", 80).unwrap();

        assert_eq!(c.get_int("server.port"), Some(80));
    }

    #[test]
    fn test_slice_set_coerce() {
        let mut c = Config::new();

        // Coerce nil to slice
        c.set("values[2]", 45).unwrap();

        assert_eq!(c.get_int("values[2]"), Some(45));
    }

    #[test]
    fn test_file_namespace() {
        use file::{File, FileFormat};

        let mut c = Config::new();
        let text = r#"
            [development]
            port = 8080

            [production]
            port = 80
        "#;

        c.merge(File::from_str(text, FileFormat::Toml).namespace("development")).unwrap();

        assert_eq!(c.get_int("port"), Some(8080));

        c.merge(File::from_str(text, FileFormat::Toml).namespace("production")).unwrap();

        assert_eq!(c.get_int("port"), Some(80));
    }
}
