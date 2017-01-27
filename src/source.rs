use std::error::Error;

use value::Value;

pub trait Source {
    fn get(&self, key: &str) -> Option<Value>;
}

pub trait SourceBuilder {
    fn build(&self) -> Result<Box<Source>, Box<Error>>;
}
