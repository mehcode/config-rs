use crate::element::ConfigElement;
use crate::object::ConfigObject;
use crate::source::ConfigSource;
use crate::accessor::Accessor;
use crate::accessor::ParsableAccessor;

#[derive(Debug)]
pub struct Config<'a> {
    layers: Vec<ConfigObject<'a>>,
}

impl<'a> Config<'a> {
    pub fn builder() -> ConfigBuilder<'a> {
        ConfigBuilder::new()
    }

    /// Access the configuration at a specific position
    ///
    /// Use an object of a type implementing the `ParsableAccessor` trait for accessing the
    /// configuration at a certain position.
    /// As `ParsableAccessor` is implemented by [`&str`] and [`String`], passing those directly
    /// works.
    ///
    /// # Note
    ///
    /// Each time, [`Config::get`] is called, the `ParsableAccessor::parse()` function is called.
    /// If that is a unbearable overhead (especially in cases where the accessor is hard-coded),
    /// [`Config::get_with_accessor`] can be used to prevent that overhead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::config::Config;
    /// let config: Config = { //...
    /// # unimplemented!()
    /// };
    ///
    /// config.get("foo")
    ///     // ...
    /// # ;
    /// ```
    pub fn get<A>(&self, accessor: A) -> Result<Option<&ConfigElement<'a>>, ConfigError>
        where A: ParsableAccessor
    {
        let accessor = accessor.parse()?;
        self.get_with_accessor(accessor)
    }

    /// Access the configuration at a specific position
    ///
    /// See [`Config::get`]
    pub fn get_with_accessor(&self, accessor: Accessor) -> Result<Option<&ConfigElement<'a>>, ConfigError> {
        for layer in self.layers.iter() {
            if let Some(value) = layer.get(&accessor)? {
                return Ok(Some(value))
            }
        }

        Ok(None)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Accessor parser error")]
    AccessorParseError(#[from] crate::accessor::AccessorParseError),

    #[error("Config object access error")]
    ConfigObjectAccessError(#[from] crate::object::ConfigObjectAccessError),
}

#[derive(Debug)]
pub struct ConfigBuilder<'a> {
    layers: Vec<ConfigObject<'a>>,
    defaults: Vec<ConfigObject<'a>>,
    overwrites: Vec<ConfigObject<'a>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigBuilderError<E> {
    Wrapped(E),
}

impl<'a> ConfigBuilder<'a> {
    pub(crate) fn new() -> Self {
        ConfigBuilder {
            layers: Vec::new(),
            defaults: Vec::new(),
            overwrites: Vec::new(),
        }
    }

    pub fn load<CS, E>(mut self, source: &'a CS) -> Result<Self, ConfigBuilderError<E>>
        where CS: ConfigSource<Error = E>,
              E: std::error::Error,
    {
        let object = source.load().map_err(ConfigBuilderError::Wrapped)?;
        self.layers.push(object);
        Ok(self)
    }

    pub fn load_default<CS, E>(mut self, source: &'a CS) -> Result<Self, ConfigBuilderError<E>>
        where CS: ConfigSource<Error = E>,
              E: std::error::Error,
    {
        let object = source.load().map_err(ConfigBuilderError::Wrapped)?;
        self.defaults.push(object);
        Ok(self)
    }

    pub fn load_overwrite<CS, E>(mut self, source: &'a CS) -> Result<Self, ConfigBuilderError<E>>
        where CS: ConfigSource<Error = E>,
              E: std::error::Error,
    {
        let object = source.load().map_err(ConfigBuilderError::Wrapped)?;
        self.overwrites.push(object);
        Ok(self)
    }

    pub fn build(mut self) -> Config<'a> {
        let mut layers = self.overwrites;
        layers.append(&mut self.layers);
        layers.append(&mut self.defaults);

        Config {
            layers
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::AsConfigElement;

    #[test]
    fn test_compile_loading() {
        let _c = Config::builder()
            .load(&crate::source::test_source::TestSource(|| ConfigElement::Null))
            .unwrap()
            .build();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key": "value" }
        "#).unwrap();
        let json = std::sync::Arc::new(json);

        let _c = Config::builder()
            .load(&crate::source::test_source::TestSource(|| json.as_config_element().unwrap()))
            .unwrap()
            .build();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json_get_value() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key": "value" }
        "#).unwrap();
        let json = std::sync::Arc::new(json);

        let source = crate::source::test_source::TestSource(|| json.as_config_element().unwrap());

        let c = Config::builder()
            .load(&source)
            .unwrap()
            .build();

        let r = c.get("key");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value")));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_layered_json_config() {
        let json1: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value1" }
        "#).unwrap();
        let json1 = std::sync::Arc::new(json1);

        let json2: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value2", "key2": "value3" }
        "#).unwrap();
        let json2 = std::sync::Arc::new(json2);

        let source1 = crate::source::test_source::TestSource(|| json1.as_config_element().unwrap());
        let source2 = crate::source::test_source::TestSource(|| json2.as_config_element().unwrap());

        let c = Config::builder()
            .load(&source1)
            .unwrap()
            .load(&source2)
            .unwrap()
            .build();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value1")));

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value3")));
    }

    #[test]
    #[cfg(all(feature = "json", feature = "toml"))]
    fn test_layered_json_toml_config() {
        let json: serde_json::Value = serde_json::from_str(r#"
            { "key1": "value1" }
        "#).unwrap();
        let json = std::sync::Arc::new(json);

        let toml: toml::Value = toml::from_str(r#"
            key1 = "value2"
            key2 = "value3"
        "#).unwrap();
        let toml = std::sync::Arc::new(toml);

        let source1 = crate::source::test_source::TestSource(|| json.as_config_element().unwrap());
        let source2 = crate::source::test_source::TestSource(|| toml.as_config_element().unwrap());

        let c = Config::builder()
            .load(&source1)
            .unwrap()
            .load(&source2)
            .unwrap()
            .build();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value1")));

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(std::matches!(r, ConfigElement::Str("value3")));
    }
}
