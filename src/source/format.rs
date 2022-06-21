use crate::element::AsConfigElement;

use super::SourceError;

pub trait FormatParser: std::fmt::Debug {
    type Output: AsConfigElement + std::fmt::Debug + Sized;

    fn parse(buffer: &str) -> Result<Self::Output, SourceError>;
}

#[cfg(feature = "json")]
#[derive(Debug)]
pub struct JsonFormatParser;

#[cfg(feature = "json")]
impl FormatParser for JsonFormatParser {
    type Output = serde_json::Value;

    fn parse(buffer: &str) -> Result<Self::Output, SourceError> {
        serde_json::from_str(buffer).map_err(SourceError::JsonParserError)
    }
}

