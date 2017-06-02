use source::Source;
use value::Value;
use std::error::Error;

#[cfg(feature = "toml")]
mod toml;

// #[cfg(feature = "json")]
// mod json;

// #[cfg(feature = "yaml")]
// mod yaml;

#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    /// TOML (parsed with toml)
    #[cfg(feature = "toml")]
    Toml,

    // /// JSON (parsed with serde_json)
    // #[cfg(feature = "json")]
    // Json,

    // /// YAML (parsed with yaml_rust)
    // #[cfg(feature = "yaml")]
    // Yaml,
}

impl FileFormat {
    // TODO: pub(crate)
    #[doc(hidden)]
    pub fn extensions(&self) -> Vec<&'static str> {
        match *self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => vec!["toml"],

            // #[cfg(feature = "json")]
            // FileFormat::Json => vec!["json"],

            // #[cfg(feature = "yaml")]
            // FileFormat::Yaml => vec!["yaml", "yml"],
        }
    }

    // TODO: pub(crate)
    #[doc(hidden)]
    #[allow(unused_variables)]
    pub fn parse(&self, uri: Option<&String>, text: &str, namespace: Option<&String>) -> Result<Value, Box<Error>> {
        match *self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => toml::parse(uri, text, namespace),

            // #[cfg(feature = "json")]
            // FileFormat::Json => json::Content::parse(text, namespace),

            // #[cfg(feature = "yaml")]
            // FileFormat::Yaml => yaml::Content::parse(text, namespace),
        }
    }
}
