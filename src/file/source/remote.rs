use std::str::FromStr;
use std::result;
use std::error::Error;

use remote::Remote;
use source::Source;
use super::{FileFormat, FileSource};

/// Describes a file sourced from a string
#[derive(Clone, Debug)]
pub struct FileSourceRemote {
    remote: Box<Remote + Sync + Send>,
    path: String,
}

impl FileSourceRemote {
    pub(crate) fn new<R: Remote + Sync + Send>(remote: R, path: &str) -> Self
    where
        R: 'static,
    {
        Self {
            remote: Box::new(remote),
            path: path.into(),
        }
    }
}

impl FileSource for FileSourceRemote {
    fn resolve(
        &self,
        format_hint: Option<FileFormat>,
    ) -> Result<(Option<String>, String, FileFormat), Box<Error + Send + Sync>> {
        Ok((
            Some(self.remote.uri()),
            self.remote.get(&self.path)?,
            format_hint.expect("from_remote requires a set file format"),
        ))
    }
}
