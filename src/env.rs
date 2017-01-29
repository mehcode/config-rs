use std::env;
use std::error::Error;
use std::borrow::Cow;

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
    fn build(&self) -> Result<Box<source::Source>, Box<Error>> {
        Ok(Box::new(self.clone()))
    }
}

impl source::Source for Environment {
    fn get<'a>(&self, key: &str) -> Option<Cow<'a, Value>> {
        let mut env_key = String::new();

        // Apply prefix
        if let Some(ref prefix) = self.prefix {
            env_key.push_str(prefix);
            env_key.push('_');
        }

        env_key.push_str(&key.to_uppercase());

        // Attempt to retreive environment variable and coerce into a Value
        env::var(env_key.clone()).ok().map(Value::from).map(Cow::Owned)
    }
}
