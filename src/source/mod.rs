use crate::object::ConfigObject;

mod format;
mod string;

pub use crate::source::string::StringSource;
pub use crate::source::format::FormatParser;
pub use crate::source::format::JsonFormatParser;

pub trait ConfigSource: std::fmt::Debug {
    fn load(&self) -> Result<ConfigObject, SourceError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "json")]
    #[error("JSON Parser error")]
    JsonParserError(#[from] serde_json::Error),

    #[cfg(feature = "json")]
    #[error("JSON load error")]
    JsonLoadError(#[from] crate::element::json::JsonIntoConfigElementError),

    #[cfg(feature = "toml")]
    #[error("TOML Parser error")]
    TomlParserError(#[from] toml::de::Error),

    #[cfg(feature = "toml")]
    #[error("TOML load error")]
    TomlLoadError(#[from] crate::element::toml::TomlIntoConfigElementError),
}

#[cfg(test)]
pub(crate) mod test_source {
    use crate::source::ConfigSource;
    use crate::object::ConfigObject;
    use crate::element::ConfigElement;
    use crate::description::ConfigSourceDescription;

    use super::SourceError;

    #[derive(Debug)]
    pub(crate) struct TestSource(pub(crate) ConfigElement);

    impl ConfigSource for TestSource {
        fn load(&self) -> Result<ConfigObject, SourceError> {
            Ok(ConfigObject::new(self.0.clone(), ConfigSourceDescription::Unknown))
        }
    }
}

