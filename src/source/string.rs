use crate::ConfigSource;
use crate::description::ConfigSourceDescription;
use crate::element::AsConfigElement;
use crate::object::ConfigObject;
use crate::source::format::FormatParser;

use super::SourceError;

#[derive(Debug)]
pub struct StringSource<P: FormatParser + std::fmt::Debug> {
    data: P::Output,
}

impl<P: FormatParser> StringSource<P> {
    pub fn new(buffer: &str) -> Result<Self, SourceError> {
        Ok(StringSource {
            data: P::parse(buffer)?,
        })
    }
}

impl<P: FormatParser + std::fmt::Debug> ConfigSource for StringSource<P>
    where SourceError: From<<<P as FormatParser>::Output as AsConfigElement>::Error>
{
    fn load<'a>(&'a self) -> Result<ConfigObject<'a>, SourceError> {
        let element = self.data
            .as_config_element()?;

        let desc = ConfigSourceDescription::Custom("String".to_string());
        Ok(ConfigObject::new(element, desc))
    }
}

#[cfg(test)]
mod test_source_impl {
    #[cfg(feature = "json")]
    #[test]
    fn test_json_string_source() {
        use super::*;

        let source = "{}";

        let source = StringSource::<crate::source::JsonFormatParser>::new(source).unwrap();
        let _object = source.load().unwrap();
    }
}

