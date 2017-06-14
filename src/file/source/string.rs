use std::str::FromStr;
use std::result;
use std::error::Error;

use source::Source;
use super::{FileSource, FileFormat};

/// Describes a file sourced from a string
pub struct FileSourceString(String);

impl<'a> From<&'a str> for FileSourceString {
    fn from(s: &'a str) -> Self {
        FileSourceString(s.into())
    }
}

impl FileSource for FileSourceString {
    fn resolve(&self, _: Option<FileFormat>) -> Result<(Option<String>, String), Box<Error>> {
        Ok((None, self.0.clone()))
    }
}
