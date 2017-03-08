use error::*;
use value::Value;

/// Describes a generic _source_ of configuration properties.
pub trait Source {
    /// Collect all configuration properties available from this source and return
    /// a top-level Value (which we expected to be a Table).
    fn collect(&self) -> Result<Value>;
}
