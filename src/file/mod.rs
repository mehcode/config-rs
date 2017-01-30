use std::env;
use std::error::Error;
use std::io::{self, Read};
use std::fs;
use std::path::PathBuf;

use source::{Source, SourceBuilder};

mod nil;

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "json")]
mod json;

pub enum FileFormat {
    /// TOML (parsed with toml)
    #[cfg(feature = "toml")]
    Toml,

    /// JSON (parsed with serde_json)
    #[cfg(feature = "json")]
    Json,
}

impl FileFormat {
    fn extensions(&self) -> Vec<&'static str> {
        match *self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => vec!["toml"],

            #[cfg(feature = "json")]
            FileFormat::Json => vec!["json"],
        }
    }

    #[allow(unused_variables)]
    fn parse(&self, text: &str) -> Result<Box<Source>, Box<Error>> {
        match *self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => toml::Content::parse(text),

            #[cfg(feature = "json")]
            FileFormat::Json => json::Content::parse(text),
        }
    }
}

pub struct File {
    /// Basename of configuration file
    name: String,

    /// Directory where configuration file is found
    /// When not specified, the current working directory (CWD) is considered
    path: Option<String>,

    /// Namespace to restrict configuration from the file
    namespace: Option<String>,

    /// Format of file (which dictates what driver to use); Defauts to TOML.
    format: FileFormat,

    /// A required File will error if it cannot be found
    required: bool,
}

impl File {
    pub fn new(name: &str, format: FileFormat) -> File {
        File {
            name: name.into(),
            format: format,
            required: true,
            path: None,
            namespace: None,
        }
    }

    pub fn path(self, path: &str) -> File {
        File { path: Some(path.into()), ..self }
    }

    pub fn namespace(self, namespace: &str) -> File {
        File { namespace: Some(namespace.into()), ..self }
    }

    pub fn required(self, required: bool) -> File {
        File { required: required, ..self }
    }

    // Find configuration file
    // Use algorithm similar to .git detection by git
    fn find_file(&self) -> Result<PathBuf, Box<Error>> {
        // Build expected configuration file
        let mut basename = PathBuf::new();
        let extensions = self.format.extensions();

        if let Some(ref path) = self.path {
            basename.push(path.clone());
        }

        basename.push(self.name.clone());

        // Find configuration file (algorithm similar to .git detection by git)
        let mut dir = env::current_dir()?;

        loop {
            let mut filename = dir.as_path().join(basename.clone());
            for ext in &extensions {
                filename.set_extension(ext);

                if filename.is_file() {
                    // File exists and is a file
                    return Ok(filename);
                }
            }

            // Not found.. travse up via the dir
            if !dir.pop() {
                // Failed to find the configuration file
                return Err(io::Error::new(io::ErrorKind::NotFound,
                                          format!("configuration file \"{}\" not found",
                                                  basename.to_string_lossy()))
                    .into());
            }
        }
    }

    // Build normally and return error on failure
    fn try_build(&self) -> Result<Box<Source>, Box<Error>> {
        // Find file
        let filename = self.find_file()?;

        // Read contents from file
        let mut file = fs::File::open(filename)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        // Parse the file
        self.format.parse(&text)
    }
}

impl SourceBuilder for File {
    // Use try_build but only pass an error through if this source
    // is required
    fn build(&self) -> Result<Box<Source>, Box<Error>> {
        if self.required {
            self.try_build()
        } else {
            self.try_build().or_else(|_| Ok(Box::new(nil::Nil {})))
        }
    }
}
