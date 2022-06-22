use crate::config::Config;
use crate::config::ConfigBuilderError;
use crate::object::ConfigObject;
use crate::source::ConfigSource;

#[derive(Debug)]
pub struct ConfigBuilder<'a> {
    layers: Vec<ConfigObject<'a>>,
    defaults: Vec<ConfigObject<'a>>,
    overwrites: Vec<ConfigObject<'a>>,
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
