use crate::config::Config;
use crate::object::ConfigObject;
#[cfg(feature = "async")]
use crate::source::AsyncConfigSource;
use crate::source::ConfigSource;
use crate::source::SourceError;

use super::ConfigError;

#[derive(Debug)]
pub struct ConfigBuilder<'source> {
    layers_builders: Vec<Box<dyn ConfigSource<'source>>>,
    defaults_builders: Vec<Box<dyn ConfigSource<'source>>>,
    overwrites_builders: Vec<Box<dyn ConfigSource<'source>>>,
}

impl<'source> ConfigBuilder<'source> {
    pub(crate) fn new() -> Self {
        ConfigBuilder {
            layers_builders: Vec::new(),
            defaults_builders: Vec::new(),
            overwrites_builders: Vec::new(),
        }
    }

    pub fn load(mut self, source: Box<dyn ConfigSource<'source>>) -> Self {
        self.layers_builders.push(source);
        self
    }

    pub fn load_default(mut self, source: Box<dyn ConfigSource<'source>>) -> Self {
        self.defaults_builders.push(source);
        self
    }

    pub fn load_overwrite(mut self, source: Box<dyn ConfigSource<'source>>) -> Self {
        self.overwrites_builders.push(source);
        self
    }

    pub fn build(self) -> Result<Config<'source>, ConfigError> {
        Config::build_from_builder(self)
    }

    pub(crate) fn reload(&'source self) -> Result<Vec<ConfigObject<'source>>, SourceError> {
        self.overwrites_builders
            .iter()
            .map(|cs| cs.load())
            .chain(self.layers_builders.iter().map(|cs| cs.load()))
            .chain(self.defaults_builders.iter().map(|cs| cs.load()))
            .collect()
    }
}

#[cfg(feature = "async")]
#[derive(Debug)]
pub struct AsyncConfigBuilder<'source> {
    layers_builders: Vec<Box<dyn AsyncConfigSource<'source>>>,
    defaults_builders: Vec<Box<dyn AsyncConfigSource<'source>>>,
    overwrites_builders: Vec<Box<dyn AsyncConfigSource<'source>>>,
}

#[cfg(feature = "async")]
impl<'source> AsyncConfigBuilder<'source> {
    pub(crate) fn new() -> Self {
        Self {
            layers_builders: Vec::new(),
            defaults_builders: Vec::new(),
            overwrites_builders: Vec::new(),
        }
    }

    /// Register a AsyncConfigSource with the builder, but don't poll it
    pub fn load(mut self, source: Box<dyn AsyncConfigSource<'source>>) -> Self {
        self.layers_builders.push(source);
        self
    }

    /// Register a AsyncConfigSource with the builder, but don't poll it
    pub fn load_default(mut self, source: Box<dyn AsyncConfigSource<'source>>) -> Self {
        self.defaults_builders.push(source);
        self
    }

    /// Register a AsyncConfigSource with the builder, but don't poll it
    pub fn load_overwrite(mut self, source: Box<dyn AsyncConfigSource<'source>>) -> Self {
        self.overwrites_builders.push(source);
        self
    }

    pub async fn build(self) -> Result<Config<'source>, ConfigError> {
        Config::build_from_async_builder(self).await
    }

    pub(crate) async fn reload(&'source self) -> Result<Vec<ConfigObject<'source>>, SourceError> {
        async fn do_load<'source>(builders: &'source Vec<Box<dyn AsyncConfigSource<'source>>>) -> Result<Vec<ConfigObject<'source>>, SourceError> {
            let mut v = Vec::with_capacity(builders.len());
            for cs in builders.iter() {
                v.push(cs.load().await?);
            }
            Ok(v)
        }

        let overwrites = do_load(&self.overwrites_builders);
        let layers = do_load(&self.layers_builders);
        let defaults = do_load(&self.defaults_builders);

        let (mut overwrites, mut layers, mut defaults) =
            futures::try_join!(overwrites, layers, defaults)?;

        let mut v = Vec::with_capacity({
            self.layers_builders.len()
                + self.defaults_builders.len()
                + self.overwrites_builders.len()
        });

        v.append(&mut overwrites);
        v.append(&mut layers);
        v.append(&mut defaults);

        Ok(v)
    }
}
