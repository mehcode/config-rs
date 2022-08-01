use std::sync::RwLock;

use crate::accessor::Accessor;
use crate::accessor::ParsableAccessor;
#[cfg(feature = "async")]
use crate::config::AsyncConfigBuilder;
use crate::config::ConfigBuilder;
use crate::config::ConfigError;
use crate::element::ConfigElement;
use crate::object::ConfigObject;

#[derive(Debug)]
pub struct Config<'source> {
    builder: Builder<'source>,
    layers: RwLock<Option<Vec<ConfigObject<'source>>>>,
}

#[derive(Debug)]
enum Builder<'source> {
    Sync(ConfigBuilder<'source>),

    #[cfg(feature = "async")]
    Async(AsyncConfigBuilder<'source>),
}

impl<'source> Config<'source> {
    pub fn builder() -> ConfigBuilder<'source> {
        ConfigBuilder::new()
    }

    pub(super) fn build_from_builder(builder: ConfigBuilder<'source>) -> Result<Self, ConfigError> {
        let config = Config {
            layers: RwLock::new(None),
            builder: Builder::Sync(builder),
        };

        {
            let mut layers = config.layers.write().unwrap();
            #[allow(irrefutable_let_patterns)]
            if let Builder::Sync(builder) = &config.builder {
                *layers = Some(builder.reload()?);
            } else {
                unreachable!()
            }
        }

        Ok(config)
    }

    #[cfg(feature = "async")]
    pub(super) async fn build_from_async_builder(
        builder: AsyncConfigBuilder<'source>,
    ) -> Result<Config<'source>, ConfigError> {
        let config = Config {
            layers: RwLock::new(None),
            builder: Builder::Async(builder),
        };

        {
            let l = match config.builder {
                Builder::Sync(ref builder) => builder.reload()?,
                Builder::Async(ref builder) => builder.reload().await?,
            };

            let mut layers = config.layers.write().unwrap();
            *layers = Some(l);
        }

        Ok(config)
    }

    #[cfg(feature = "async")]
    pub fn async_builder() -> AsyncConfigBuilder<'source> {
        AsyncConfigBuilder::new()
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
    pub fn get<A>(&self, accessor: A) -> Result<Option<&ConfigElement<'source>>, ConfigError>
    where
        A: ParsableAccessor,
    {
        let accessor = accessor.parse()?;
        self.get_with_accessor(accessor)
    }

    /// Access the configuration at a specific position
    ///
    /// See [`Config::get`]
    pub fn get_with_accessor(
        &self,
        mut accessor: Accessor,
    ) -> Result<Option<&ConfigElement<'source>>, ConfigError> {
        let layers = self
            .layers
            .read()
            .map_err(|_| ConfigError::InternalRwLockPoisioned)?
            .ok_or_else(|| ConfigError::NotLoaded)?;

        for layer in layers.iter() {
            if let Some(value) = layer.get(&mut accessor)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}
