use error::*;
use value::Value;
use std::collections::HashMap;

/// Describes a generic _source_ of configuration properties.
pub trait Source {
    /// Collect all configuration properties available from this source and return
    /// a HashMap.
    fn collect(&self) -> Result<HashMap<String, Value>>;
}
