use std::error::Error;

use crate::file::{FileExtensions, FileSource};
use crate::Format;

/// Describes a file sourced from a string
#[derive(Clone, Debug)]
pub struct FileSourceString(String);

impl<'a> From<&'a str> for FileSourceString {
    fn from(s: &'a str) -> Self {
        FileSourceString(s.into())
    }
}

impl<F> FileSource<F> for FileSourceString
where
    F: Format + FileExtensions + 'static,
{
    fn resolve(
        &self,
        format_hint: Option<F>,
    ) -> Result<(Option<String>, String, Box<dyn Format>), Box<dyn Error + Send + Sync>> {
        Ok((
            None,
            self.0.clone(),
            Box::new(format_hint.expect("from_str requires a set file format")),
        ))
    }
}
