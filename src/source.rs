use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::error::*;
use crate::path;
use crate::value::{Value, ValueKind};

/// Describes a generic _source_ of configuration properties.
pub trait Source: Debug {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync>;

    /// Collect all configuration properties available from this source and return
    /// a HashMap.
    fn collect(&self) -> Result<HashMap<String, Value>>;

    /// Collects all configuration properties to a provided cache.
    fn collect_to(&self, cache: &mut Value) -> Result<()> {
        self.collect()?
            .iter()
            .for_each(|(key, val)| set_value(cache, key, val));

        Ok(())
    }
}

fn set_value(cache: &mut Value, key: &String, value: &Value) {
    match path::Expression::from_str(key) {
        // Set using the path
        Ok(expr) => expr.set(cache, value.clone()),

        // Set diretly anyway
        _ => path::Expression::Identifier(key.clone()).set(cache, value.clone()),
    }
}

impl Clone for Box<dyn Source + Send + Sync> {
    fn clone(&self) -> Box<dyn Source + Send + Sync> {
        self.clone_into_box()
    }
}

impl Source for Vec<Box<dyn Source + Send + Sync>> {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        let mut cache: Value = HashMap::<String, Value>::new().into();

        for source in self {
            source.collect_to(&mut cache)?;
        }

        if let ValueKind::Table(table) = cache.kind {
            Ok(table)
        } else {
            unreachable!();
        }
    }
}

impl Source for [Box<dyn Source + Send + Sync>] {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.to_owned())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        let mut cache: Value = HashMap::<String, Value>::new().into();

        for source in self {
            source.collect_to(&mut cache)?;
        }

        if let ValueKind::Table(table) = cache.kind {
            Ok(table)
        } else {
            unreachable!();
        }
    }
}

impl<T> Source for Vec<T>
where
    T: Source + Sync + Send,
    T: Clone,
    T: 'static,
{
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        let mut cache: Value = HashMap::<String, Value>::new().into();

        for source in self {
            source.collect_to(&mut cache)?;
        }

        if let ValueKind::Table(table) = cache.kind {
            Ok(table)
        } else {
            unreachable!();
        }
    }
}
