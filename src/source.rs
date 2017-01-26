use std::error::Error;
use std::collections::HashMap;

use value::Value;

pub trait Source {
    fn build(&mut self) -> Result<HashMap<String, Value>, Box<Error>>;
}
