use crate::description::ConfigSourceDescription;
use crate::element::IntoConfigElement;
use crate::object::ConfigObject;
use crate::source::format::FormatParser;
use crate::ConfigSource;

use super::SourceError;

#[derive(Debug)]
pub struct StringSource<P: FormatParser + std::fmt::Debug> {
    source: String,
    _pd: std::marker::PhantomData<P>,
}

impl<P: FormatParser> StringSource<P> {
    pub fn new(source: String) -> Result<Self, SourceError> {
        Ok(StringSource {
            source,
            _pd: std::marker::PhantomData,
        })
    }
}

impl<P> ConfigSource for StringSource<P>
where
    P: FormatParser + std::fmt::Debug,
    SourceError: From<<<P as FormatParser>::Output as IntoConfigElement>::Error>,
{
    fn load(&self) -> Result<ConfigObject, SourceError> {
        let element = P::parse(&self.source)?;
        let element = element.into_config_element()?;

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

        let source =
            StringSource::<crate::source::JsonFormatParser>::new(source.to_string()).unwrap();
        let _object = source.load().unwrap();
    }
}
