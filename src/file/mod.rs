use std::env;
use std::error::Error;
use std::io::{self, Read};
use std::fs;
use std::path::{PathBuf, Path};
use std::collections::HashMap;

use source::{Source, SourceBuilder};

mod nil;

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "yaml")]
mod yaml;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum FileFormat {
    /// TOML (parsed with toml)
    #[cfg(feature = "toml")]
    Toml,

    /// JSON (parsed with serde_json)
    #[cfg(feature = "json")]
    Json,

    /// YAML (parsed with yaml_rust)
    #[cfg(feature = "yaml")]
    Yaml,
}



lazy_static! {
    static ref ALL_EXTENSIONS: HashMap<FileFormat, Vec<&'static str>> = {
        let mut formats: HashMap<FileFormat, Vec<_>> = HashMap::new();

        #[cfg(feature = "toml")]
        formats.insert(FileFormat::Toml, vec!["toml"]);

        #[cfg(feature = "json")]
        formats.insert(FileFormat::Json, vec!["json"]);

        #[cfg(feature = "yaml")]
        formats.insert(FileFormat::Yaml, vec!["yaml", "yml"]);
        formats
    };
}


impl FileFormat {

    fn extensions(&self) -> &'static Vec<&'static str> {
        ALL_EXTENSIONS.get(self).unwrap()
    }

    #[allow(unused_variables)]
    fn parse(&self, text: &str, namespace: Option<&String>) -> Result<Box<Source + Send + Sync>, Box<Error>> {
        match *self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => toml::Content::parse(text, namespace),

            #[cfg(feature = "json")]
            FileFormat::Json => json::Content::parse(text, namespace),

            #[cfg(feature = "yaml")]
            FileFormat::Yaml => yaml::Content::parse(text, namespace),
        }
    }
}

pub trait FileSource {
    fn try_build(&self,
                 format: FileFormat,
                 namespace: Option<&String>)
                 -> Result<Box<Source + Send + Sync>, Box<Error>>;
}

pub struct FileSourceString(String);

impl FileSource for FileSourceString {
    fn try_build(&self,
                 format: FileFormat,
                 namespace: Option<&String>)
                 -> Result<Box<Source + Send + Sync>, Box<Error>> {
        format.parse(&self.0, namespace)
    }
}

pub struct FileSourceFile {
    /// Basename of configuration file
    name: String,

    /// Directory where configuration file is found
    /// When not specified, the current working directory (CWD) is considered
    path: Option<String>,
}

impl FileSourceFile {
    // Find configuration file
    // Use algorithm similar to .git detection by git
    fn find_file(&self, format: FileFormat) -> Result<PathBuf, Box<Error>> {
        // Build expected configuration file
        let mut basename = PathBuf::new();
        let extensions = format.extensions();

        if let Some(ref path) = self.path {
            basename.push(path.clone());
        }

        basename.push(self.name.clone());

        // Find configuration file (algorithm similar to .git detection by git)
        let mut dir = env::current_dir()?;

        loop {
            let mut filename = dir.as_path().join(basename.clone());
            for ext in extensions {
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
}

impl FileSource for FileSourceFile {
    fn try_build(&self,
                 format: FileFormat,
                 namespace: Option<&String>)
                 -> Result<Box<Source + Send + Sync>, Box<Error>> {
        // Find file
        let filename = self.find_file(format)?;

        // Read contents from file
        let mut file = fs::File::open(filename)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        // Parse the file
        format.parse(&text, namespace)
    }
}

pub struct File<T: FileSource> {
    /// Source of the file
    source: T,

    /// Namespace to restrict configuration from the file
    namespace: Option<String>,

    /// Format of file (which dictates what driver to use); Defauts to TOML.
    format: FileFormat,

    /// A required File will error if it cannot be found
    required: bool,
}

impl File<FileSourceString> {
    pub fn from_str(s: &str, format: FileFormat) -> File<FileSourceString> {
        File {
            format: format,
            required: true,
            namespace: None,
            source: FileSourceString(s.into()),
        }
    }
}

impl File<FileSourceFile> {
    // Given the basename of a file, tries reading it in all the registered formats. #21
    pub fn with_name(name: &str) -> File<FileSourceFile> {
        let possible_format = ALL_EXTENSIONS.iter().find( |&(_, extension_names)|
            extension_names.iter().any( |&extension_name|
               Path::new(&format!("{}.{}",name, extension_name)).exists()
            )
        );
        match possible_format {
            Some((format, _)) => File::new(name, *format),
            None => panic!("The format of the file {} is not understood", name),
        }
    }

    pub fn new(name: &str, format: FileFormat) -> File<FileSourceFile> {
        File {
            format: format,
            required: true,
            namespace: None,
            source: FileSourceFile {
                name: name.into(),
                path: None,
            },
        }
    }
}

impl<T: FileSource> File<T> {
    pub fn required(self, required: bool) -> File<T> {
        File { required: required, ..self }
    }

    pub fn namespace(self, namespace: &str) -> Self {
        File { namespace: Some(namespace.into()), ..self }
    }

    // Build normally and return error on failure
    fn try_build(&self) -> Result<Box<Source + Send + Sync>, Box<Error>> {
        self.source.try_build(self.format, self.namespace.as_ref())
    }
}

impl File<FileSourceFile> {
    pub fn path(self, path: &str) -> Self {
        File { source: FileSourceFile { path: Some(path.into()), ..self.source }, ..self }
    }
}

impl<T: FileSource> SourceBuilder for File<T> {
    // Use try_build but only pass an error through if this source
    // is required
    fn build(&self) -> Result<Box<Source + Send + Sync>, Box<Error>> {
        if self.required {
            self.try_build()
        } else {
            self.try_build().or_else(|_| Ok(Box::new(nil::Nil {})))
        }
    }
}
