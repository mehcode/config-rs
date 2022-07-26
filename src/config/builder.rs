use crate::config::Config;
use crate::config::ConfigBuilderError;
use crate::object::ConfigObject;
#[cfg(feature = "async")]
use crate::source::AsyncConfigSource;
use crate::source::ConfigSource;
use crate::source::SourceError;

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

    pub fn load<CS>(mut self, source: &'a CS) -> Result<Self, ConfigBuilderError<SourceError>>
    where
        CS: ConfigSource,
    {
        let object = source.load().map_err(ConfigBuilderError::Wrapped)?;
        self.layers.push(object);
        Ok(self)
    }

    pub fn load_default<CS>(mut self, source: &'a CS) -> Result<Self, ConfigBuilderError<SourceError>>
    where
        CS: ConfigSource,
    {
        let object = source.load().map_err(ConfigBuilderError::Wrapped)?;
        self.defaults.push(object);
        Ok(self)
    }

    pub fn load_overwrite<CS>(mut self, source: &'a CS) -> Result<Self, ConfigBuilderError<SourceError>>
    where
        CS: ConfigSource,
    {
        let object = source.load().map_err(ConfigBuilderError::Wrapped)?;
        self.overwrites.push(object);
        Ok(self)
    }

    pub fn build(&self) -> Config<'a> {
        let mut layers = self.overwrites.clone();
        layers.append(&mut self.layers.clone());
        layers.append(&mut self.defaults.clone());

        Config { layers }
    }
}

#[cfg(feature = "async")]
#[derive(Debug)]
pub struct AsyncConfigBuilder {
    sources: Vec<Box<dyn AsyncConfigSource>>,
}

#[cfg(feature = "async")]
impl AsyncConfigBuilder {
    pub(crate) fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Register a AsyncConfigSource with the builder, but don't poll it
    pub fn load(mut self, source: Box<dyn AsyncConfigSource>) -> Self {
        self.sources.push(source);
        self
    }

    pub async fn build<'a>(&'a self) -> Result<Config<'a>, Vec<SourceError>> {
        use futures::stream::FuturesUnordered;
        use futures::stream::StreamExt;
        use itertools::Itertools;

        let (layers, errs) = self
            .sources
            .iter()
            .map(|cs| async move { cs.load().await })
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<Result<ConfigObject<'a>, SourceError>>>()
            .await
            .into_iter()
            .partition_result::<Vec<ConfigObject<'a>>, Vec<SourceError>, ConfigObject<'a>, SourceError>();

        if errs.is_empty() {
            Ok(Config { layers })
        } else {
            Err(errs)
        }
    }
}
