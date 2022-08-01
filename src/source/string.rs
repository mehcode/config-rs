use crate::ConfigSource;
use crate::description::ConfigSourceDescription;
use crate::element::AsConfigElement;
use crate::object::ConfigObject;
use crate::source::format::FormatParser;

use super::SourceError;

#[derive(Debug)]
pub struct StringSource<'source, P: FormatParser<'source> + std::fmt::Debug> {
    source: &'source str,
    _pd: std::marker::PhantomData<P>,
}

impl<'source, P: FormatParser<'source>> StringSource<'source, P> {
    pub fn new(source: &'source str) -> Result<Self, SourceError> {
        Ok(StringSource {
            source,
            _pd: std::marker::PhantomData
        })
    }
}

impl<'source, P> ConfigSource<'source> for StringSource<'source, P>
    where P: FormatParser<'source> + std::fmt::Debug + 'source,
        SourceError: From<<<P as FormatParser<'source>>::Output as AsConfigElement<'source>>::Error>
{
    fn load(&'source self) -> Result<ConfigObject<'source>, SourceError> {
        let element = P::parse(self.source)?;
        let element = element.as_config_element()?;

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

