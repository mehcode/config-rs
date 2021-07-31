// If no features are used, there is an "unused mut" warning in `ALL_EXTENSIONS`
// BUG: ? For some reason this doesn't do anything if I try and function scope this
#![allow(unused_mut)]

use std::collections::HashMap;
use std::error::Error;

use crate::map::Map;
use crate::{value::Value, Format, file::extension::FileExtensions};

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "ini")]
mod ini;

#[cfg(feature = "ron")]
mod ron;

#[cfg(feature = "json5")]
mod json5;

/// File formats provided by the library.
///
/// Although it is possible to define custom formats using [`Format`] trait it is recommended to use FileFormat if possible.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    /// INI (parsed with rust_ini)
    #[cfg(feature = "ini")]
    Ini,

    /// RON (parsed with ron)
    #[cfg(feature = "ron")]
    Ron,

    /// JSON5 (parsed with json5)
    #[cfg(feature = "json5")]
    Json5,
}

lazy_static! {
    #[doc(hidden)]
    // #[allow(unused_mut)] ?
    pub static ref ALL_EXTENSIONS: HashMap<FileFormat, Vec<&'static str>> = {
        let mut formats: HashMap<FileFormat, Vec<_>> = HashMap::new();

        #[cfg(feature = "toml")]
        formats.insert(FileFormat::Toml, vec!["toml"]);

        #[cfg(feature = "json")]
        formats.insert(FileFormat::Json, vec!["json"]);

        #[cfg(feature = "yaml")]
        formats.insert(FileFormat::Yaml, vec!["yaml", "yml"]);

        #[cfg(feature = "ini")]
        formats.insert(FileFormat::Ini, vec!["ini"]);

        #[cfg(feature = "ron")]
        formats.insert(FileFormat::Ron, vec!["ron"]);

        #[cfg(feature = "json5")]
        formats.insert(FileFormat::Json5, vec!["json5"]);

        formats
    };
}

impl FileFormat {
    // TODO: pub(crate)
    #[doc(hidden)]
    pub fn extensions(&self) -> &'static Vec<&'static str> {
        // It should not be possible for this to fail
        // A FileFormat would need to be declared without being added to the
        // ALL_EXTENSIONS map.
        ALL_EXTENSIONS.get(self).unwrap()
    }

    // TODO: pub(crate)
    #[doc(hidden)]
    #[allow(unused_variables)]
    pub fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        match self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => toml::parse(uri, text),

            #[cfg(feature = "json")]
            FileFormat::Json => json::parse(uri, text),

            #[cfg(feature = "yaml")]
            FileFormat::Yaml => yaml::parse(uri, text),

            #[cfg(feature = "ini")]
            FileFormat::Ini => ini::parse(uri, text),

            #[cfg(feature = "ron")]
            FileFormat::Ron => ron::parse(uri, text),

            #[cfg(feature = "json5")]
            FileFormat::Json5 => json5::parse(uri, text),
        }
    }
}

impl Format for FileFormat {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        self.parse(uri, text)
    }
}

impl FileExtensions for FileFormat {
    fn extensions(&self) -> &Vec<&'static str> {
        self.extensions()
    }
}
