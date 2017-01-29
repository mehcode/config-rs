#![feature(drop_types_in_const)]

#![allow(unknown_lints)]

#[cfg(feature = "toml")]
extern crate toml;

#[cfg(feature = "json")]
extern crate serde_json;

mod value;
mod source;
mod file;
mod env;
mod config;

use std::error::Error;
use std::sync::{Once, ONCE_INIT};
use std::borrow::Cow;

pub use source::{Source, SourceBuilder};
pub use file::{File, FileFormat};
pub use env::Environment;

pub use value::Value;

pub use config::Config;

// Global configuration
static mut CONFIG: Option<Config<'static>> = None;
static CONFIG_INIT: Once = ONCE_INIT;

// Get the global configuration instance
pub fn global() -> &'static mut Config<'static> {
    unsafe {
        CONFIG_INIT.call_once(|| {
            CONFIG = Some(Default::default());
        });

        CONFIG.as_mut().unwrap()
    }
}

pub fn merge<T>(source: T) -> Result<(), Box<Error>>
    where T: SourceBuilder
{
    global().merge(source)
}

pub fn set_default<T>(key: &str, value: T) -> Result<(), Box<Error>>
    where T: Into<Value<'static>>
{
    global().set_default(key, value)
}

pub fn set<T>(key: &str, value: T) -> Result<(), Box<Error>>
    where T: Into<Value<'static>>
{
    global().set(key, value)
}

pub fn get<'a>(key: &str) -> Option<Cow<'a, Value>> {
    global().get(key)
}

pub fn get_str<'a>(key: &str) -> Option<Cow<'a, str>> {
    global().get_str(key)
}

pub fn get_int(key: &str) -> Option<i64> {
    global().get_int(key)
}

pub fn get_float(key: &str) -> Option<f64> {
    global().get_float(key)
}

pub fn get_bool(key: &str) -> Option<bool> {
    global().get_bool(key)
}
