mod format;
pub mod source;

use std::fmt::Debug;
use std::path::{Path, PathBuf};

use crate::error::{ConfigError, Result};
use crate::map::Map;
use crate::source::Source;
use crate::value::Value;
use crate::Format;

pub use self::format::FileFormat;
use self::source::FileSource;

pub use self::source::file::FileSourceFile;
pub use self::source::string::FileSourceString;

/// A configuration source backed up by a file.
///
/// It supports optional automatic file format discovery.
#[derive(Clone, Debug)]
#[must_use]
pub struct File<T, F> {
    source: T,

    /// Format of file (which dictates what driver to use).
    format: Option<F>,

    /// A required File will error if it cannot be found
    required: bool,
}

/// An extension of [`Format`](crate::Format) trait.
///
/// Associates format with file extensions, therefore linking storage-agnostic notion of format to a file system.
pub trait FileStoredFormat: Format {
    /// Returns a vector of file extensions, for instance `[yml, yaml]`.
    fn file_extensions(&self) -> &'static [&'static str];
}

impl<F> File<source::string::FileSourceString, F>
where
    F: FileStoredFormat + 'static,
{
    pub fn from_str(s: &str, format: F) -> Self {
        Self {
            format: Some(format),
            required: true,
            source: s.into(),
        }
    }
}

impl<F> File<source::file::FileSourceFile, F>
where
    F: FileStoredFormat + 'static,
{
    pub fn new(name: &str, format: F) -> Self {
        Self {
            format: Some(format),
            required: true,
            source: source::file::FileSourceFile::new(name.into()),
        }
    }
}

impl File<source::file::FileSourceFile, FileFormat> {
    /// Given the basename of a file, will attempt to locate a file by setting its
    /// extension to a registered format.
    pub fn with_name(name: &str) -> Self {
        Self {
            format: None,
            required: true,
            source: source::file::FileSourceFile::new(name.into()),
        }
    }
}

impl<'a> From<&'a Path> for File<source::file::FileSourceFile, FileFormat> {
    fn from(path: &'a Path) -> Self {
        Self {
            format: None,
            required: true,
            source: source::file::FileSourceFile::new(path.to_path_buf()),
        }
    }
}

impl From<PathBuf> for File<source::file::FileSourceFile, FileFormat> {
    fn from(path: PathBuf) -> Self {
        Self {
            format: None,
            required: true,
            source: source::file::FileSourceFile::new(path),
        }
    }
}

impl<T, F> File<T, F>
where
    F: FileStoredFormat + 'static,
    T: FileSource<F>,
{
    pub fn format(mut self, format: F) -> Self {
        self.format = Some(format);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

impl<T, F> Source for File<T, F>
where
    F: FileStoredFormat + Debug + Clone + Send + Sync + 'static,
    T: Sync + Send + FileSource<F> + 'static,
{
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>> {
        // Coerce the file contents to a string
        let (uri, contents, format) = match self
            .source
            .resolve(self.format.clone())
            .map_err(|err| ConfigError::Foreign(err))
        {
            Ok(result) => (result.uri, result.content, result.format),

            Err(error) => {
                if !self.required {
                    return Ok(Map::new());
                }

                return Err(error);
            }
        };

        // Parse the string using the given format
        format
            .parse(uri.as_ref(), &contents)
            .map_err(|cause| ConfigError::FileParse { uri, cause })
    }
}
