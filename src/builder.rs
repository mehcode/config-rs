use std::str::FromStr;
use std::{collections::HashMap, iter::IntoIterator};

use crate::error::Result;
use crate::{config::Config, path::Expression, source::Source, value::Value};

/// A configuration builder
///
/// It registers ordered sources of configuration to later build consistent [`Config`] from them.
/// Configuration sources it defines are defaults, [`Source`]s and overrides.
///
/// Defaults are alaways loaded first and can be overwritten by any of two other sources.
/// Overrides are always loaded last, thus cannot be overridden.
/// Both can be only set explicitly key by key in code
/// using [`set_default`](Self::set_default) or [`set_override`](Self::set_override).
///
/// An intermediate category, [`Source`], set groups of keys at once implicitly using data coming from external sources
/// like files, environment variables or others that one implements. Defining a [`Source`] is as simple as implementing
/// a trait for a struct.
///
/// Adding sources, setting defaults and overrides does not invoke any I/O nor builds a config.
/// It happens on demand when [`build`](Self::build) (or its alternative) is called.
/// Therefore all errors, related to any of the [`Source`] will only show up then.
///
/// # Examples
///
/// ```rust
/// # use config::*;
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let mut builder = ConfigBuilder::default()
///     .set_default("default", "1")?
///     .add_source(File::new("config/settings", FileFormat::Json))
///     .set_override("override", "1")?;
///
/// match builder.build() {
///     Ok(config) => {
///         // use your config
///     },
///     Err(e) => {
///         // something went wrong
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
/// Calls can be not chained as well
/// ```rust
/// # use std::error::Error;
/// # use config::*;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let mut builder = ConfigBuilder::default();
/// builder = builder.set_default("default", "1")?;
/// builder = builder.add_source(File::new("config/settings", FileFormat::Json));
/// builder = builder.add_source(File::new("config/settings.prod", FileFormat::Json));
/// builder = builder.set_override("override", "1")?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct ConfigBuilder {
    defaults: HashMap<Expression, Value>,
    overrides: HashMap<Expression, Value>,
    sources: Vec<Box<dyn Source + Send + Sync>>,
}

impl ConfigBuilder {
    /// Set a default `value` at `key`
    ///
    /// This value can be overwritten by any [`Source`] or override.
    ///
    /// # Errors
    ///
    /// Fails if `Expression::from_str(key)` fails.
    pub fn set_default<S, T>(mut self, key: S, value: T) -> Result<ConfigBuilder>
    where
        S: AsRef<str>,
        T: Into<Value>,
    {
        self.defaults
            .insert(Expression::from_str(key.as_ref())?, value.into());
        Ok(self)
    }

    /// Registers new [`Source`] in this builder.
    ///
    /// Calling this method does not invoke any I/O. [`Source`] is only saved in internal register for later use.
    pub fn add_source<T>(mut self, source: T) -> Self
    where
        T: Source + Send + Sync + 'static,
    {
        self.sources.push(Box::new(source));
        self
    }

    /// Set an override
    ///
    /// This function sets an overwrite value. It will not be altered by any default or [`Source`]
    ///
    /// # Errors
    ///
    /// Fails if `Expression::from_str(key)` fails.
    pub fn set_override<S, T>(mut self, key: S, value: T) -> Result<ConfigBuilder>
    where
        S: AsRef<str>,
        T: Into<Value>,
    {
        self.overrides
            .insert(Expression::from_str(key.as_ref())?, value.into());
        Ok(self)
    }

    /// Reads all registered [`Source`]s.
    ///
    /// This is the method that invokes all I/O operations.
    /// For a non consuming alternative see [`build_cloned`](Self::build_cloned)
    ///
    /// # Errors
    /// If source collection fails, be it technical reasons or related to inability to read data as `Config` for different reasons,
    /// this method returns error.
    pub fn build(self) -> Result<Config> {
        Self::build_internal(self.defaults, self.overrides, &self.sources)
    }

    /// Reads all registered [`Source`]s.
    ///
    /// Similar to [`build`](Self::build), but it does not take ownership of `ConfigBuilder` to allow later reuse.
    /// Internally it clones data to achieve it.
    ///
    /// # Errors
    /// If source collection fails, be it technical reasons or related to inability to read data as `Config` for different reasons,
    /// this method returns error.
    pub fn build_cloned(&self) -> Result<Config> {
        Self::build_internal(self.defaults.clone(), self.overrides.clone(), &self.sources)
    }

    fn build_internal(
        defaults: HashMap<Expression, Value>,
        overrides: HashMap<Expression, Value>,
        sources: &[Box<dyn Source + Send + Sync>],
    ) -> Result<Config> {
        let mut cache: Value = HashMap::<String, Value>::new().into();

        // Add defaults
        for (key, val) in defaults.into_iter() {
            key.set(&mut cache, val);
        }

        // Add sources
        sources.collect_to(&mut cache)?;

        // Add overrides
        for (key, val) in overrides.into_iter() {
            key.set(&mut cache, val);
        }

        Ok(Config::new(cache))
    }
}
