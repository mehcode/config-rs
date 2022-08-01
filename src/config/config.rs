use crate::accessor::Accessor;
use crate::accessor::ParsableAccessor;
use crate::config::ConfigBuilder;
use crate::config::ConfigError;
use crate::element::ConfigElement;
use crate::object::ConfigObject;

#[derive(Debug)]
pub struct Config {
    layers: Vec<ConfigObject>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub(super) fn build_from_builder(builder: &ConfigBuilder) -> Result<Self, ConfigError> {
        let config = Config {
            layers: builder.reload()?,
        };

        Ok(config)
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
    pub fn get<A>(&self, accessor: A) -> Result<Option<&ConfigElement>, ConfigError>
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
    ) -> Result<Option<&ConfigElement>, ConfigError> {
        for layer in self.layers.iter() {
            if let Some(value) = layer.get(&mut accessor)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}
