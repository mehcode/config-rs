use crate::element::AsConfigElement;

use super::SourceError;

pub trait FormatParser<'source>: std::fmt::Debug {
    type Output: AsConfigElement<'source> + std::fmt::Debug + Sized;

    fn parse(buffer: &'source str) -> Result<Self::Output, SourceError>;
}

#[cfg(feature = "json")]
#[derive(Debug)]
pub struct JsonFormatParser;

#[cfg(feature = "json")]
impl<'source> FormatParser<'source> for JsonFormatParser {
    type Output = serde_json::Value;

    fn parse(buffer: &'source str) -> Result<Self::Output, SourceError> {
        serde_json::from_str(buffer).map_err(SourceError::JsonParserError)
    }
}


#[cfg(feature = "toml")]
#[derive(Debug)]
pub struct TomlFormatParser;

#[cfg(feature = "toml")]
impl<'source> FormatParser<'source> for TomlFormatParser {
    type Output = toml::Value;

    fn parse(buffer: &'source str) -> Result<Self::Output, SourceError> {
        toml::from_str(buffer).map_err(SourceError::TomlParserError)
    }
}
