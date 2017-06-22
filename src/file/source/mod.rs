pub mod file;
pub mod string;

use std::fmt::Debug;
use std::error::Error;

use source::Source;
use super::FileFormat;

/// Describes where the file is sourced
pub trait FileSource: Debug + Clone {
    fn resolve(&self,
               format_hint: Option<FileFormat>)
               -> Result<(Option<String>, String, FileFormat), Box<Error>>;
}
