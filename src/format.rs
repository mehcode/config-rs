use std::error::Error;

use crate::{map::Map, value::Value};

/// Describes a format of configuration source data
///
/// Implementations of this trait can be used to convert [`File`](crate::File) sources to configuration data.
///
/// There can be various formats, some of them provided by this library, such as JSON, Yaml and other.
/// This trait enables users of the library to easily define their own, even proprietary formats without
/// the need to alter library sources.
///
/// What is more, it is recommended to use this trait with custom [`Source`](crate::Source)s and their async counterparts.
pub trait Format {
    /// Parses provided content into configuration values understood by the library.
    ///
    /// It also allows specifying optional URI of the source associated with format instance that can facilitate debugging.
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>>;
}
