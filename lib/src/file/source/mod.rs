pub mod file;
pub mod string;

use std::error::Error;

use source::Source;
use super::FileFormat;

/// Describes where the file is sourced
pub trait FileSource {
    fn resolve(&self, format_hint: Option<FileFormat>) -> Result<(Option<String>, String), Box<Error>>;
}
