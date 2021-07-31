pub mod file;
pub mod string;

use std::error::Error;
use std::fmt::Debug;

use crate::{file::FileExtensions, Format};

/// Describes where the file is sourced
pub trait FileSource<T>: Debug + Clone
where
    T: Format + FileExtensions,
{
    fn resolve(
        &self,
        format_hint: Option<T>,
    ) -> Result<(Option<String>, String, Box<dyn Format>), Box<dyn Error + Send + Sync>>;
}
