use std::env;
use std::error::Error;
use std::collections::HashMap;

use source;
use value::Value;

#[derive(Clone)]
pub struct Environment {
    /// Optional prefix that would restrict environment consideration
    /// to only variables which begin with that prefix.
    prefix: Option<String>,
}

impl Environment {
    pub fn new<'a, T>(prefix: T) -> Environment
        where T: Into<Option<&'a str>>
    {
        Environment { prefix: prefix.into().map(String::from) }
    }
}

impl source::SourceBuilder for Environment {
    fn build(&self) -> Result<Box<source::Source + Send + Sync>, Box<Error>> {
        Ok(Box::new(self.clone()))
    }
}

impl source::Source for Environment {
    fn collect(&self) -> HashMap<String, Value> {
        // Iterate through environment variables
        let mut r = HashMap::new();

        // Make prefix pattern
        let prefix_pat = if let Some(ref prefix) = self.prefix {
            Some(prefix.clone() + "_".into())
        } else {
            None
        };

        for (key, value) in env::vars() {
            let mut key = key.to_string();

            // Check if key matches prefix
            if let Some(ref prefix_pat) = prefix_pat {
                if key.starts_with(prefix_pat) {
                    // Remove the prefix from the key
                    key = key[prefix_pat.len()..].to_string();
                } else {
                    // Skip this key
                    continue;
                }
            }

            r.insert(key, Value::String(value));
        }

        r
    }
}
