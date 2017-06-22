use error::*;
use std::fmt::Debug;
use value::Value;
use std::collections::HashMap;

/// Describes a generic _source_ of configuration properties.
pub trait Source: Debug {
    fn clone_into_box(&self) -> Box<Source + Send + Sync>;

    /// Collect all configuration properties available from this source and return
    /// a HashMap.
    fn collect(&self) -> Result<HashMap<String, Value>>;
}

impl Clone for Box<Source + Send + Sync> {
    fn clone(&self) -> Box<Source + Send + Sync> {
        self.clone_into_box()
    }
}
