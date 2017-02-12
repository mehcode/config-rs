use std::error::Error;
use std::collections::HashMap;

use value::Value;

pub trait Source {
    fn collect(&self) -> HashMap<String, Value>;
}

pub trait SourceBuilder {
    fn build(&self) -> Result<Box<Source + Send + Sync>, Box<Error>>;
}
