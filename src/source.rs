use std::error::Error;
use std::borrow::Cow;

use value::Value;

pub trait Source {
    fn get<'a>(&self, key: &str) -> Option<Cow<'a, Value>>;
}

pub trait SourceBuilder {
    fn build(&self) -> Result<Box<Source>, Box<Error>>;
}
