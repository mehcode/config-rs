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
use std::sync::{Once, ONCE_INIT, RwLock};

pub use source::{Source, SourceBuilder};
pub use file::{File, FileFormat};
pub use env::Environment;

pub use value::Value;

pub use config::Config;

// Global configuration
static mut CONFIG: Option<RwLock<Config>> = None;
static CONFIG_INIT: Once = ONCE_INIT;

// Get the global configuration instance
fn global() -> &'static RwLock<Config> {
    unsafe {
        CONFIG_INIT.call_once(|| { CONFIG = Some(Default::default()); });

        CONFIG.as_mut().unwrap()
    }
}

pub fn merge<T>(source: T) -> Result<(), Box<Error>>
    where T: SourceBuilder
{
    global().write()?.merge(source)
}

pub fn set_default<T>(key: &str, value: T) -> Result<(), Box<Error>>
    where T: Into<Value>
{
    global().write()?.set_default(key, value);

    // TODO: `set_default` will be able to fail soon so this will not be needed
    Ok(())
}

pub fn set<T>(key: &str, value: T) -> Result<(), Box<Error>>
    where T: Into<Value>
{
    global().write()?.set(key, value);

    // TODO: `set_default` will be able to fail soon so this will not be needed
    Ok(())
}

pub fn get(key: &str) -> Option<Value> {
    global().read().unwrap().get(key)
}

pub fn get_str(key: &str) -> Option<String> {
    global().read().unwrap().get_str(key)
}

pub fn get_int(key: &str) -> Option<i64> {
    global().read().unwrap().get_int(key)
}

pub fn get_float(key: &str) -> Option<f64> {
    global().read().unwrap().get_float(key)
}

pub fn get_bool(key: &str) -> Option<bool> {
    global().read().unwrap().get_bool(key)
}
